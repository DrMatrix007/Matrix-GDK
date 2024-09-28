use std::marker::PhantomData;

use super::{
    components::ComponentAccessError,
    query::{Query, QueryError},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SystemError {
    MissingArgs,
}

pub trait System<Queryable, EngineArgs> {
    fn prepare_args(&mut self, queryable: &mut Queryable) -> Result<(), QueryError>;

    fn run(&mut self, run_args: &mut EngineArgs) -> Result<(), SystemError>;

    fn consume(&mut self, queryable: &mut Queryable) -> Result<(), ComponentAccessError>;
}

pub trait QuerySystem<Queryable, EngineArgs> {
    type Query: Query<Queryable>;
    fn run(&mut self, engine_args: &mut EngineArgs, args: &mut Self::Query);
}
pub struct QuerySystemWrapper<
    Q: Query<Queryable>,
    Queryable,
    EngineArgs,
    QS: QuerySystem<Queryable, EngineArgs, Query = Q>,
> {
    system: QS,
    args: Option<Q>,
    marker: PhantomData<(Queryable, EngineArgs)>,
}

impl<
        Q: Query<Queryable>,
        Queryable,
        EngineArgs,
        QS: QuerySystem<Queryable, EngineArgs, Query = Q>,
    > QuerySystemWrapper<Q, Queryable, EngineArgs, QS>
{
    pub fn new(system: QS) -> Self {
        Self {
            system,
            args: None,
            marker: PhantomData,
        }
    }
}

impl<
        Q: Query<Queryable>,
        Queryable,
        EngineArgs,
        QS: QuerySystem<Queryable, EngineArgs, Query = Q>,
    > System<Queryable, EngineArgs> for QuerySystemWrapper<Q, Queryable, EngineArgs, QS>
{
    fn prepare_args(&mut self, queryable: &mut Queryable) -> Result<(), QueryError> {
        assert!(self.args.is_none());
        self.args = Some(Q::query(queryable)?);
        Ok(())
    }

    fn run(&mut self, engine_args: &mut EngineArgs) -> Result<(), SystemError> {
        let args = self.args.as_mut();
        if let Some(args) = args {
            self.system.run(engine_args, args);
            Ok(())
        } else {
            Err(SystemError::MissingArgs)
        }
    }

    fn consume(&mut self, queryable: &mut Queryable) -> Result<(), ComponentAccessError> {
        self.args.take().unwrap().consume(queryable)
    }
}

pub struct QuerySystemFn<Q: Query<Queryable>, Queryable, Fn: FnMut(&mut Q)>(
    Fn,
    PhantomData<(Q, Queryable)>,
);

impl<Q: Query<Queryable>, Queryable, Fn: FnMut(&mut Q)> QuerySystemFn<Q, Queryable, Fn> {
    pub fn new(f: Fn) -> Self {
        Self(f, PhantomData)
    }
}

impl<Q: Query<Queryable>, Queryable, Fn: FnMut(&mut Q), EngineArgs>
    QuerySystem<Queryable, EngineArgs> for QuerySystemFn<Q, Queryable, Fn>
{
    type Query = Q;

    fn run(&mut self, _engine_args: &mut EngineArgs, args: &mut Self::Query) {
        (self.0)(args);
    }
}

trait IntoSystem<Queryable, EngineArgs, Placeholder> {
    fn into_system(self) -> impl System<Queryable, EngineArgs>;
}

struct FnPlaceHolder<Q: Query<Queryable>, Queryable>(PhantomData<(Q, Queryable)>);

impl<Q: Query<Queryable>, Queryable, EngineArgs, F: FnMut(&mut Q)>
    IntoSystem<Queryable, EngineArgs, FnPlaceHolder<Q, Queryable>> for F
{
    fn into_system(self) -> impl System<Queryable, EngineArgs> {
        QuerySystemWrapper::new(QuerySystemFn::new(self))
    }
}
struct QsPlaceHolder<Q: Query<Queryable>, Queryable>(PhantomData<(Q, Queryable)>);
impl<
        Q: Query<Queryable>,
        Queryable,
        EngineArgs,
        QS: QuerySystem<Queryable, EngineArgs, Query = Q>,
    > IntoSystem<Queryable, EngineArgs, QsPlaceHolder<Q, Queryable>> for QS
{
    fn into_system(self) -> impl System<Queryable, EngineArgs> {
        QuerySystemWrapper::new(self)
    }
}

#[cfg(test)]
mod test {
    use crate::engine::{
        components::ComponentRegistry,
        query::{ReadC, WriteC},
        scene::SceneRegistry,
    };

    use super::{IntoSystem, QuerySystem, System};

    fn system_a(_data: &mut ReadC<()>) {}

    struct A;
    impl QuerySystem<SceneRegistry, ()> for A {
        type Query = WriteC<()>;

        fn run(&mut self, _engine_args: &mut (), _args: &mut Self::Query) {}
    }

    #[test]
    fn test_systems() {
        let reg = ComponentRegistry::new();

        let reg = &mut SceneRegistry { components: reg };

        let mut b: Box<dyn System<SceneRegistry, ()>> = Box::new(system_a.into_system());
        let mut c: Box<dyn System<SceneRegistry, ()>> = Box::new(system_a.into_system());

        let mut d: Box<dyn System<SceneRegistry, ()>> = Box::new(A.into_system());
        let mut e: Box<dyn System<SceneRegistry, ()>> = Box::new(A.into_system());

        b.prepare_args(reg).unwrap();
        c.prepare_args(reg).unwrap();
        d.prepare_args(reg).unwrap_err();

        b.run(&mut ()).unwrap();
        c.run(&mut ()).unwrap();

        b.consume(reg).unwrap();
        c.consume(reg).unwrap();

        d.prepare_args(reg).unwrap();
        e.prepare_args(reg).unwrap_err();

        d.run(&mut ()).unwrap();

        d.consume(reg).unwrap();
    }
}
