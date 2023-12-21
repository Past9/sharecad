use crate::{
    camera::{Camera, CameraController, CameraControllerRequest},
    color::{rgb, rgba, Rgba},
    input::InputEvent,
    light::{AmbientLight, DirectionalLight},
    model::{GeometryId, InstanceId, ModelInstance},
    render::{
        MsaaSamples, ObjectRenderer, PositionRenderer, RenderContext, RenderTarget, VisualRenderer,
    },
    scene::Scene,
};
use space::{deg, point3, vec3, Point2, Point3, Quat, Vec3};
use std::path::Path;
use wgpu::Surface;

const NUM_X_INSTANCES: u32 = 3;
const NUM_Y_INSTANCES: u32 = 3;
const NUM_Z_INSTANCES: u32 = 1;
const SPACE_BETWEEN: f64 = 5.0;

const SELECTED_SURFACE_TINT: Rgba = rgba(0.0, 0.6, 0.8, 0.7);
const SELECTED_CURVE_TINT: Rgba = rgba(0.0, 1.0, 1.0, 1.0);
const SELECTED_POINT_TINT: Rgba = SELECTED_CURVE_TINT;

pub struct ViewState {
    visual_renderer: VisualRenderer,
    position_renderer: PositionRenderer,
    object_renderer: ObjectRenderer,
    camera_controller: CameraController,
    scene: Scene,
    directional_lights: Vec<DirectionalLight>,
    needs_position_update: bool,
    needs_object_update: bool,
}
impl ViewState {
    #[cfg(all(not(feature = "winit"), feature = "egui"))]
    pub fn new_from_resources(
        render_state: &egui_wgpu::RenderState,
        visual_texture_usage: Option<wgpu::TextureUsages>,
        msaa_samples: MsaaSamples,
        pixels_per_point: f32,
        scene: Scene,
    ) -> ViewState {
        let render_context = RenderContext::from_resources(
            render_state.adapter.clone(),
            render_state.device.clone(),
            render_state.queue.clone(),
        );
        let visual_render_target = render_context.render_into_memory(
            (1, 1),
            render_state.target_format,
            visual_texture_usage,
            MsaaSamples::Samples1,
        );
        Self::create(
            render_context,
            visual_render_target,
            msaa_samples,
            pixels_per_point,
            scene,
        )
    }

    pub fn scene(&self) -> &Scene {
        &self.scene
    }

    pub fn scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
    }

    #[cfg(feature = "winit")]
    pub async fn new_on_window(
        window: &winit::window::Window,
        out_dir: &str,
        pixels_per_point: f32,
    ) -> Self {
        let render_context = RenderContext::new().await;
        let visual_render_target = render_context.render_on_window(window);
        Self::create(render_context, visual_render_target, out_dir)
    }

    pub async fn new_on_surface(
        surface: Surface,
        size: (u32, u32),
        pixels_per_point: f32,
        scene: Scene,
    ) -> ViewState {
        let render_context = RenderContext::new().await;
        let visual_render_target = render_context.render_on_surface(surface, size);
        Self::create(
            render_context,
            visual_render_target,
            MsaaSamples::Samples1,
            pixels_per_point,
            scene,
        )
    }

    fn create(
        render_context: RenderContext,
        visual_render_target: RenderTarget,
        msaa_samples: MsaaSamples,
        pixels_per_point: f32,
        scene: Scene,
    ) -> ViewState {
        let visual_renderer = VisualRenderer::new(
            &render_context,
            visual_render_target,
            msaa_samples,
            pixels_per_point,
        );
        let object_renderer = ObjectRenderer::new(
            render_context.render_into_memory(
                visual_renderer.size(),
                wgpu::TextureFormat::R32Uint,
                None,
                MsaaSamples::Samples1,
            ),
            pixels_per_point,
        );
        let position_renderer = PositionRenderer::new(
            render_context.render_into_memory(
                Self::downsize(visual_renderer.size()),
                wgpu::TextureFormat::Rgba32Float,
                None,
                MsaaSamples::Samples1,
            ),
            pixels_per_point,
        );

        let camera = Camera::new(
            point3(0.0, 0.0, 0.0),
            10.0,
            800.0 * 2f64.sqrt(),
            -Vec3::UNIT_Z,
            Vec3::UNIT_Y,
            deg(45.0),
        );

        let camera_controller = CameraController::new(camera);

        Self {
            visual_renderer,
            object_renderer,
            position_renderer,
            camera_controller,
            scene,
            directional_lights: vec![],
            needs_position_update: true,
            needs_object_update: true,
        }
    }

    fn downsize(size: (u32, u32)) -> (u32, u32) {
        ((size.0 / 10).max(1), (size.1 / 10).max(1))
    }

    pub fn visual_target(&self) -> &RenderTarget {
        self.visual_renderer.target()
    }

    pub fn object_target(&self) -> &RenderTarget {
        self.object_renderer.target()
    }

    pub fn resize(&mut self, new_size: (u32, u32)) -> bool {
        if new_size != self.visual_renderer.size() {
            self.visual_renderer.resize(new_size);
            self.position_renderer.resize(Self::downsize(new_size));
            self.object_renderer.resize(new_size);
            self.camera_controller.resize(new_size);
            true
        } else {
            false
        }
    }

    pub fn input(&mut self, event: &InputEvent) -> bool {
        match event {
            InputEvent::CursorMoved(point) => {
                let id = self.get_instance_id_at(point);
                if let Some(id) = id {
                    //println!("hover {:?}", id);
                }
            }
            _ => {}
        };

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
        //
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let camera = self.camera_controller.camera();

        self.visual_renderer.render(&self.scene, camera).unwrap();
        self.object_renderer.render(&self.scene, camera).unwrap();

        self.needs_position_update = true;
        self.needs_object_update = true;

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
                    // Pixels that are background (not geometry) are not counted.
                    // They are identified by having an Alpha of 0.0.
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
            match self
                .position_renderer
                .render(&self.scene, &self.camera_controller.camera())
            {
                Ok(_) => {
                    self.needs_position_update = false;
                    Ok(())
                }
                err @ Err(_) => err,
            }
        } else {
            Ok(())
        }
    }

    fn get_instance_id_at(&mut self, coords: &Point2) -> Option<GeometryId> {
        self.render_object().unwrap();

        let coords = (coords.x as u32, coords.y as u32);
        let id = pollster::block_on(self.object_renderer.get_id_at(coords));

        GeometryId::from_shader_value(id)
    }

    fn render_object(&mut self) -> Result<(), wgpu::SurfaceError> {
        if self.needs_object_update {
            match self
                .object_renderer
                .render(&self.scene, &self.camera_controller.camera())
            {
                Ok(_) => {
                    self.needs_object_update = false;
                    Ok(())
                }
                err @ Err(_) => err,
            }
        } else {
            Ok(())
        }
    }
}
