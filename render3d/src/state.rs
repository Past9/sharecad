use std::sync::Arc;

use space::{deg, point3, vec3, Quat, Vec3};
use winit::{event::WindowEvent, window::Window};

use crate::{
    camera::Camera,
    light::Light,
    model::{InstanceId, PositionedInstance},
    render::VisualRenderer,
    scene::Scene,
};

const NUM_INSTANCES_PER_ROW: u32 = 11;
const SPACE_BETWEEN: f64 = 3.0;

pub struct State {
    visual_render: VisualRenderer,
    camera: Camera,
    scene: Scene,
    window: Window,
}
impl State {
    pub async fn new(window: Window) -> Self {
        let visual_render = VisualRenderer::new(&window).await;

        let camera = Camera::new(
            point3(0.0, 0.0, 0.0),
            16.0,
            160.0 * 2f64.sqrt(),
            vec3(0.0, 1.0, 5.0),
            Vec3::UNIT_Y,
            deg(0.0),
        );

        let scene = {
            let mut scene = Scene::new();

            let instances = (0..NUM_INSTANCES_PER_ROW)
                .flat_map(|z| {
                    (0..NUM_INSTANCES_PER_ROW).map(move |x| {
                        let id = InstanceId(z * NUM_INSTANCES_PER_ROW + x);
                        let x =
                            SPACE_BETWEEN * (x as f64 - NUM_INSTANCES_PER_ROW as f64 / 2.0 + 0.5);
                        let z =
                            SPACE_BETWEEN * (z as f64 - NUM_INSTANCES_PER_ROW as f64 / 2.0 + 0.5);

                        let position = vec3(x, 0.0, z);

                        let rotation = if position.is_zero() {
                            Quat::from_axis_angle(Vec3::UNIT_Z, deg(0.0))
                        } else {
                            Quat::from_axis_angle(position.normalize(), deg(45.0))
                        };

                        PositionedInstance {
                            id,
                            position,
                            rotation,
                        }
                    })
                })
                .collect::<Vec<_>>();

            scene
                .load_model_file::<PositionedInstance>("rounded-cube.obj", vec![instances])
                .await;

            scene.set_light(Light::new(point3(2.0, 2.0, 2.0), [1.0, 1.0, 1.0]));

            scene
        };

        Self {
            visual_render,
            camera,
            scene,
            window,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        //
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        //
        false
    }

    pub fn update(&mut self) {
        //
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.visual_render.render()
    }
}
