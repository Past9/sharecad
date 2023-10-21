use space::{deg, point3, vec3, Point3, Quat, Vec3};
use winit::{event::WindowEvent, window::Window};

use crate::{
    camera::{Camera, CameraController},
    light::Light,
    model::{InstanceId, TransformedInstance},
    render::VisualRenderer,
    scene::Scene,
};

const NUM_INSTANCES_PER_ROW: u32 = 3;
const SPACE_BETWEEN: f64 = 3.0;

pub struct State {
    visual_render: VisualRenderer,
    camera: Camera,
    camera_controller: CameraController,
    scene: Scene,
    window: Window,
}
impl State {
    pub async fn new(window: Window) -> Self {
        let visual_render = VisualRenderer::new(&window).await;

        let camera = Camera::new(
            point3(0.0, 0.0, 0.0),
            5.0,
            50.0 * 2f64.sqrt(),
            vec3(0.0, 0.0, -5.0),
            Vec3::UNIT_Y,
            deg(45.0),
        );

        let camera_controller = CameraController::new(Point3::new(0.0, 0.0, 0.0));

        let scene = {
            let mut scene = Scene::new();

            let instances = (0..NUM_INSTANCES_PER_ROW)
                .flat_map(|z| {
                    (0..NUM_INSTANCES_PER_ROW).flat_map(move |y| {
                        (0..NUM_INSTANCES_PER_ROW).map(move |x| {
                            let id = InstanceId(
                                y * NUM_INSTANCES_PER_ROW.pow(2) + z * NUM_INSTANCES_PER_ROW + x,
                            );

                            let scale = vec3(
                                x as f64 / NUM_INSTANCES_PER_ROW as f64,
                                y as f64 / NUM_INSTANCES_PER_ROW as f64,
                                z as f64 / NUM_INSTANCES_PER_ROW as f64,
                            );

                            let rotation = Quat::from_axis_angle(Vec3::UNIT_Y, deg(0.0));

                            let position = vec3(
                                SPACE_BETWEEN
                                    * (x as f64 - NUM_INSTANCES_PER_ROW as f64 / 2.0 + 0.5),
                                SPACE_BETWEEN
                                    * (y as f64 - NUM_INSTANCES_PER_ROW as f64 / 2.0 + 0.5),
                                SPACE_BETWEEN
                                    * (z as f64 - NUM_INSTANCES_PER_ROW as f64 / 2.0 + 0.5),
                            );
                            /*
                            let rotation = if position.is_zero() {
                                Quat::from_axis_angle(Vec3::UNIT_Z, deg(0.0))
                            } else {
                                Quat::from_axis_angle(position.normalize(), deg(45.0))
                            };
                             */

                            TransformedInstance {
                                id,
                                scale,
                                rotation,
                                position,
                            }
                        })
                    })
                })
                .collect::<Vec<_>>();

            scene
                .load_model_file::<TransformedInstance>(
                    "rounded-cube/rounded-cube.obj",
                    vec![instances],
                )
                .await;

            scene.set_light(Light::new(point3(2.0, 2.0, 2.0), [1.0, 1.0, 1.0]));

            scene
        };

        Self {
            visual_render,
            camera,
            camera_controller,
            scene,
            window,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.visual_render.resize((new_size.width, new_size.height));
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        self.camera_controller.process_events(event)
    }

    pub fn update(&mut self) {
        self.camera_controller.update_camera(&mut self.camera);

        let mut light = self.scene.light().clone();
        light.position = Quat::from_axis_angle(vec3(0.0, 1.0, 0.0), deg(1.0)) * light.position;

        self.scene.set_light(light);
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.visual_render.render(&self.scene, &self.camera)
    }
}
