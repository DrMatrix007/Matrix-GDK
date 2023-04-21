use std::{collections::VecDeque, io, sync::Arc};

use crate::dispatchers::{
    dispatchers::DispatcherArgs,
    system_registry::{BoxedSystem, SystemGroup},
    systems::SystemArgs,
};

use super::{
    access::Access,
    schedulers::Scheduler,
    thread_pool::{ThreadPool, ThreadPoolSender},
};

pub struct MultiThreadedScheduler {
    pool: ThreadPool<BoxedSystem>,
    done: VecDeque<BoxedSystem>,
    pending: VecDeque<BoxedSystem>,
    access_state: Access,
}

impl MultiThreadedScheduler {
    pub fn new(thread_count: usize) -> Self {
        Self {
            pool: ThreadPool::new(thread_count),
            done: Default::default(),
            pending: Default::default(),
            access_state: Default::default(),
        }
    }
    pub fn with_amount_of_cpu_cores() -> io::Result<Self> {
        Ok(Self::new(std::thread::available_parallelism()?.get()))
    }

    unsafe fn send_dispatcher<'a>(
        sender: &ThreadPoolSender<BoxedSystem>,
        mut dis: BoxedSystem,
        args: &mut DispatcherArgs,
        system_args: Arc<SystemArgs>,
    ) {
        let data = unsafe { dis.as_mut().dispatch(args) };

        sender
            .send(move || {
                dis.as_mut()
                    .try_run(system_args, data)
                    .map_err(|_| ())
                    .expect("this function should work");
                dis
            })
            .expect("this value should be sent");
    }
}

impl Scheduler for MultiThreadedScheduler {
    fn run<'a>(
        &mut self,
        dispatchers: &mut SystemGroup,
        args: &mut DispatcherArgs<'a>,
        system_args: Arc<SystemArgs>,
    ) {
        self.access_state.clear();
        let sender = self.pool.sender();

        while let Some(dis) = dispatchers.pop_normal() {
            match self.access_state.try_combine(dis.as_access()) {
                Ok(_) => {
                    unsafe { Self::send_dispatcher(&sender, dis, args, system_args.clone()) };
                }
                Err(_) => self.pending.push_back(dis),
            }
            for dis in self.pool.try_recv_iter() {
                self.access_state.remove(dis.as_access());
                self.done.push_back(dis);

                for _ in 0..self.pending.len() {
                    let dis = self.pending.pop_back().expect("this should work");
                    match self.access_state.try_combine(dis.as_access()) {
                        Ok(_) => {
                            unsafe {
                                Self::send_dispatcher(&sender, dis, args, system_args.clone())
                            };
                        }
                        Err(_) => self.pending.push_front(dis),
                    }
                }
            }
        }
        for i in self.pool.recv_iter() {
            self.access_state.remove(i.as_access());
            self.done.push_back(i);
            for _ in 0..self.pending.len() {
                let dis = self.pending.pop_back().expect("this should work");
                match self.access_state.try_combine(dis.as_access()) {
                    Ok(_) => {
                        unsafe { Self::send_dispatcher(&sender, dis, args, system_args.clone()) };
                    }
                    Err(_) => self.pending.push_front(dis),
                }
            }
        }
        while let Some(dis) = self.done.pop_back() {
            dispatchers.push_normal(dis);
        }

        for b in dispatchers.iter_exclusive() {
            let data = unsafe { b.as_mut().dispatch(args) };
            let Ok(_) = b.as_mut().try_run(system_args.clone(), data) else {
                panic!("Uknown error");
            };
        }
    }
}
