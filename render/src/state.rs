use std::path::Path;

use crate::{
    camera::{Camera, CameraController, CameraControllerRequest},
    input::InputEvent,
    light::Light,
    model::{InstanceId, TransformedInstance},
    render::{PositionRenderer, RenderContext, RenderTarget, VisualRenderer},
    scene::Scene,
};
use space::{deg, point3, vec3, Point3, Quat, Vec3};
use wgpu::Surface;

const NUM_INSTANCES_PER_ROW: u32 = 3;
const SPACE_BETWEEN: f64 = 3.0;

pub struct ViewState {
    visual_renderer: VisualRenderer,
    position_renderer: PositionRenderer,
    camera_controller: CameraController,
    scene: Scene,
    needs_position_update: bool,
}
impl ViewState {
    #[cfg(all(not(feature = "winit"), feature = "egui"))]
    pub fn new_from_resources(
        render_state: &egui_wgpu::RenderState,
        visual_texture_usage: Option<wgpu::TextureUsages>,
        resource_dir: &str,
    ) -> ViewState {
        let render_context = RenderContext::from_resources(
            render_state.adapter.clone(),
            render_state.device.clone(),
            render_state.queue.clone(),
        );
        let visual_render_target = render_context.render_into_memory(
            (300, 300),
            render_state.target_format,
            visual_texture_usage,
        );
        Self::create(render_context, visual_render_target, resource_dir)
    }

    #[cfg(feature = "winit")]
    pub async fn new_on_window(window: &winit::window::Window, out_dir: &str) -> Self {
        let render_context = RenderContext::new().await;
        let visual_render_target = render_context.render_on_window(window);
        Self::create(render_context, visual_render_target, out_dir)
    }

    pub async fn new_on_surface(surface: Surface, size: (u32, u32), out_dir: &str) -> ViewState {
        let render_context = RenderContext::new().await;
        let visual_render_target = render_context.render_on_surface(surface, size);
        Self::create(render_context, visual_render_target, out_dir)
    }

    fn create(
        render_context: RenderContext,
        visual_render_target: RenderTarget,
        resource_dir: &str,
    ) -> ViewState {
        let visual_renderer = VisualRenderer::new(visual_render_target);
        let position_renderer = PositionRenderer::new(render_context.render_into_memory(
            visual_renderer.size(),
            wgpu::TextureFormat::Rgba32Float,
            None,
        ));

        let camera = Camera::new(
            point3(0.0, 0.0, 0.0),
            6.0,
            500.0 * 2f64.sqrt(),
            -Vec3::UNIT_Z,
            Vec3::UNIT_Y,
            deg(0.0),
        );

        let camera_controller = CameraController::new(camera);

        let scene = {
            let mut scene = Scene::new();

            let instances = (0..NUM_INSTANCES_PER_ROW)
                .flat_map(|z| {
                    (0..NUM_INSTANCES_PER_ROW).flat_map(move |y| {
                        (0..NUM_INSTANCES_PER_ROW).map(move |x| {
                            let id = InstanceId(
                                y * NUM_INSTANCES_PER_ROW.pow(2) + z * NUM_INSTANCES_PER_ROW + x,
                            );

                            let scale = vec3(1.0, 1.0, 1.0);

                            let rotation = Quat::from_axis_angle(Vec3::UNIT_Y, deg(0.0));

                            let position = vec3(
                                SPACE_BETWEEN
                                    * (x as f64 - NUM_INSTANCES_PER_ROW as f64 / 2.0 + 0.5),
                                SPACE_BETWEEN
                                    * (y as f64 - NUM_INSTANCES_PER_ROW as f64 / 2.0 + 0.5),
                                SPACE_BETWEEN
                                    * (z as f64 - NUM_INSTANCES_PER_ROW as f64 / 2.0 + 0.5),
                            );

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

            let path = Path::new(resource_dir).join(Path::new("res/rounded-cube/rounded-cube.obj"));

            let path_str = path.to_str().unwrap();

            scene.load_model_file::<TransformedInstance>(
                //"rounded-cube/rounded-cube.obj",
                path_str,
                vec![instances],
            );

            scene.set_light(Light::new(point3(2000.0, 2000.0, -2000.0), [1.0, 1.0, 1.0]));

            scene
        };

        Self {
            visual_renderer,
            position_renderer,
            camera_controller,
            scene,
            needs_position_update: true,
        }
    }

    pub fn visual_target(&self) -> &RenderTarget {
        self.visual_renderer.target()
    }

    pub fn resize(&mut self, new_size: (u32, u32)) -> bool {
        if new_size != self.visual_renderer.size() {
            self.visual_renderer.resize(new_size);
            self.position_renderer
                .resize((new_size.0 / 10, new_size.1 / 10));
            self.camera_controller.resize(new_size);
            true
        } else {
            false
        }
    }

    pub fn input(&mut self, event: &InputEvent) -> bool {
        let result = self.camera_controller.process_events(event);

        for request in result.requests {
            match request {
                CameraControllerRequest::RequestOrbitPoint => {
                    let orbit_point = self.get_orbit_point();
                    self.camera_controller.set_orbit_point(orbit_point)
                }
            }
        }

        result.processed
    }

    pub fn update(&mut self) {
        let mut light = self.scene.light().clone();
        light.position = Quat::from_axis_angle(vec3(0.0, 1.0, 0.0), deg(1.0)) * light.position;

        self.scene.set_light(light);
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.visual_renderer
            .render(&self.scene, self.camera_controller.camera())
            .unwrap();

        self.needs_position_update = true;

        Ok(())
    }

    fn get_orbit_point(&mut self) -> Point3 {
        self.render_position().unwrap();

        let eye = self.camera_controller.camera().eye();
        let mut avg_pos = Vec3::ZERO;

        pollster::block_on(self.position_renderer.visit_pixels(|pixels| {
            let mut total_weight: f64 = 0.0;
            for pixel in pixels.iter() {
                if pixel[3] == 0.0 {
                    continue;
                }

                let pos = point3(pixel[0] as f64, pixel[1] as f64, pixel[2] as f64);
                let dist_from_camera = (eye.location - pos).magnitude();
                let weight = 1.0 / dist_from_camera;

                avg_pos += pos.into_vec() * weight;
                total_weight += weight;
            }

            if total_weight > 0.0 {
                avg_pos = avg_pos / total_weight;
            }
        }));

        avg_pos.into_point()
    }

    fn render_position(&mut self) -> Result<(), wgpu::SurfaceError> {
        if self.needs_position_update {
            self.position_renderer
                .render(&self.scene, &self.camera_controller.camera())
        } else {
            Ok(())
        }
    }
}
