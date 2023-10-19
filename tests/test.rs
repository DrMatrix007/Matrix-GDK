use std::f32::consts::PI;

#[allow(unused_imports)]
use matrix_engine::engine::{
    runtime::MultiThreaded,
    scenes::{components::Component, entities::Entity, scene_builder::SceneBuilder},
    systems::{
        query::components::{ReadC, WriteC},
        QuerySystem,
    },
    Engine,
};
use matrix_engine::{
    engine::{
        events::event_registry::EventRegistry,
        runtime::SingleThreaded,
        scenes::entities::entity_builder::EntityBuilder,
        systems::{query::resources::WriteR, SystemControlFlow},
    },
    math::{
        matrices::Vector3,
        vectors::{Vector3D, Vector4D},
    },
    renderer::{
        matrix_renderer::{
            camera::CameraResource,
            render_object::RenderObject,
            renderer_system::{MatrixRendererResource, MatrixRendererSystem, RendererResourceArgs},
        },
        pipelines::{structures::cube::Cube, transform::Transform},
    },
};
use num_traits::{clamp, clamp_max};
use wgpu::Color;
use winit::window::WindowBuilder;

#[derive(Debug)]
struct A;
impl Component for A {}

struct B;
impl Component for B {}

struct SysC;
impl QuerySystem for SysC {
    type Query = (WriteC<A>, ReadC<B>);

    fn run(
        &mut self,
        _event: &EventRegistry,
        _args: &mut Self::Query,
    ) -> matrix_engine::engine::systems::SystemControlFlow {
        SystemControlFlow::Continue
    }
}

struct SysD;
impl QuerySystem for SysD {
    type Query = (WriteC<B>, ReadC<A>);

    fn run(
        &mut self,
        _event: &EventRegistry,
        _args: &mut Self::Query,
    ) -> matrix_engine::engine::systems::SystemControlFlow {
        for event in _event.all_window_events() {
            if event.is_pressed(winit::event::VirtualKeyCode::A) {
                // spin_sleep::sleep(Duration::from_secs_f64(3.));
                println!("dam");
            }
        }

        SystemControlFlow::Continue
    }
}

struct CameraPlayerSystem;

impl CameraPlayerSystem {
    fn new() -> Self {
        Self
    }
}

impl QuerySystem for CameraPlayerSystem {
    type Query = WriteR<CameraResource>;

    fn run(
        &mut self,
        events: &EventRegistry,
        cam: &mut <Self as QuerySystem>::Query,
    ) -> SystemControlFlow {
        let cam = match cam.get_mut() {
            Some(cam) => cam,
            None => return SystemControlFlow::Continue,
        };

        let mut delta = Vector3::<f32>::zeros();

        let speed = 1.0;
        let _rotate_speed = PI / 4.0;

        for window_events in events.all_window_events() {
            if window_events.is_pressed(winit::event::VirtualKeyCode::A) {
                *delta.x_mut() -= speed;
            }
            if window_events.is_pressed(winit::event::VirtualKeyCode::D) {
                *delta.x_mut() += speed;
            }
            if window_events.is_pressed(winit::event::VirtualKeyCode::W) {
                *delta.z_mut() -= speed;
            }
            if window_events.is_pressed(winit::event::VirtualKeyCode::S) {
                *delta.z_mut() += speed;
            }
            if window_events.is_pressed(winit::event::VirtualKeyCode::Space) {
                *delta.y_mut() += speed;
            }
            if window_events.is_pressed(winit::event::VirtualKeyCode::C) {
                *delta.y_mut() -= speed;
            }
        }

        match events.mouse_scroll_delta().1.total_cmp(&0.) {
            std::cmp::Ordering::Less => {
                cam.camera_mut().prespective.fovy_rad =
                    clamp_max(cam.camera().prespective.fovy_rad * 2., PI / 1.1)
            }
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Greater => cam.camera_mut().prespective.fovy_rad /= 2.,
        }

        // delta = cam.camera().rotation.euler_into_rotation_matrix3() * delta * dt;
        let (x, y) = events.mouse_delta();
        // self.theta += (x as f32) * dt * rotate_speed;
        // self.phi += (y as f32) * dt * rotate_speed;
        // *cam.camera_mut().rotation.y_mut() = self.theta;
        // *cam.camera_mut().rotation.x_mut() = self.phi;
        // cam.camera_mut().position += delta;

        let sens = cam.camera().prespective.fovy_rad;
        cam.camera_mut().rotate_camera(
            x as f32 * events.delta_time().as_secs_f32() * sens,
            y as f32 * events.delta_time().as_secs_f32() * sens,
        );
        cam.camera_mut()
            .move_camera(delta * events.delta_time().as_secs_f32());

        SystemControlFlow::Continue
    }
}

fn main() {
    // let runtime = MultiThreaded::new(4);
    let runtime = SingleThreaded::new();

    let mut engine = Engine::new(runtime, -1);

    let window = WindowBuilder::new()
        .build(engine.event_loop().unwrap())
        .unwrap();

    let renderer_resource = MatrixRendererResource::new(RendererResourceArgs {
        background_color: Color {
            r: 0.69,
            g: 0.69,
            b: 0.69,
            a: 0.69,
        },
        window,
    });

    engine.lock_engine_resources().insert(renderer_resource);

    engine
        .engine_systems_mut()
        .push_send(MatrixRendererSystem::default());

    let builder = SceneBuilder::new(|scene_reg, system_reg| {
        for y in 0..10000 {
            for i in 0..1 {
                let mut t = Transform::identity();
                t.apply_position_diff(Vector3::from([[1.2 * i as f32, 0., y as f32 * 1.2]]));
                EntityBuilder::new(scene_reg.components_mut())
                    .add(A)
                    .unwrap()
                    .add(RenderObject::new(Cube, "tests/dirt.jpg".to_string()))
                    .unwrap()
                    .add(t)
                    .unwrap();
            }
        }
        system_reg.push_send(SysC);
        system_reg.push_send(SysD);
        system_reg.push_send(CameraPlayerSystem::new());
    });

    engine.run(&builder)
}
