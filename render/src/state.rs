use std::path::Path;

use crate::{
    camera::{Camera, CameraController, CameraControllerRequest},
    color::rgb,
    input::InputEvent,
    light::{AmbientLight, DirectionalLight},
    model::{
        CurveInstanceId, CurveMaterial, CurveMaterialSpec, CurvePoint, SceneCurve,
        SceneCurveObject, SurfaceInstanceId, TransformedCurveInstance, TransformedSurfaceInstance,
    },
    render::{PositionRenderer, RenderContext, RenderTarget, VisualRenderer},
    scene::Scene,
};
use space::{deg, point3, vec3, Point3, Quat, Vec3};
use wgpu::Surface;

const NUM_INSTANCES_PER_ROW: u32 = 1;
const SPACE_BETWEEN: f64 = 5.0;

pub struct ViewState {
    visual_renderer: VisualRenderer,
    position_renderer: PositionRenderer,
    camera_controller: CameraController,
    scene: Scene,
    needs_position_update: bool,
    directional_lights: Vec<DirectionalLight>,
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
        let visual_renderer = VisualRenderer::new(&render_context, visual_render_target);
        let position_renderer = PositionRenderer::new(render_context.render_into_memory(
            visual_renderer.size(),
            wgpu::TextureFormat::Rgba32Float,
            None,
        ));

        let camera = Camera::new(
            point3(0.0, 0.0, 0.0),
            10.0,
            800.0 * 2f64.sqrt(),
            -Vec3::UNIT_Z,
            Vec3::UNIT_Y,
            deg(45.0),
        );

        let camera_controller = CameraController::new(camera);

        let scene = {
            let mut scene = Scene::new();

            let instances = (0..NUM_INSTANCES_PER_ROW)
                .flat_map(|z| {
                    (0..NUM_INSTANCES_PER_ROW).flat_map(move |y| {
                        (0..NUM_INSTANCES_PER_ROW).map(move |x| {
                            let id = SurfaceInstanceId(
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

                            TransformedSurfaceInstance {
                                id,
                                scale,
                                rotation,
                                position,
                            }
                        })
                    })
                })
                .collect::<Vec<_>>();

            let path =
                Path::new("C:\\Users\\ross\\Projects\\sharecad\\resources\\gizmo\\gizmo2.obj");

            let path_str = path.to_str().unwrap();

            scene.load_wavefront_obj_file::<TransformedSurfaceInstance>(path_str, vec![instances]);

            /*
            let points = vec![
                point3(-1.5, 0.0, 13.0),  //
                point3(2.0, 0.0, -4.0),   //
                point3(2.0, 2.0, -5.0),   //
                point3(0.0, 2.0, -4.0),   //
                point3(-2.0, -2.0, -3.0), //
                point3(0.0, -1.0, -4.0),  //
                point3(2.0, -4.0, -5.0),  //
                point3(2.0, -6.0, -4.0),  //
                point3(-2.0, -6.0, -3.0), //
                point3(6.0, -5.0, -4.0),  //
                point3(6.0, 5.0, -5.0),   //
                point3(6.0, 8.0, -4.0),   //
                point3(6.0, 3.0, -3.0),   //
            ];
             */

            /*
            let points1 = vec![
                point3(-1.0, -1.0, -5.0), //
                point3(1.0, 1.0, 5.0),    //
            ];
              */

            let d = 1.37237;
            //let d = 1.4;

            let points = vec![
                /*
                vec![
                    point3(-1.0, -1.0, -5.0), //
                    point3(1.0, 1.0, 5.0),    //
                ],
                */
                vec![
                    point3(0.0, 0.0, -5.0), //
                    point3(0.0, 0.0, 5.0),  //
                ],
                vec![
                    point3(d, -d, -2.0),  //
                    point3(d, d, -2.0),   //
                    point3(-d, d, -2.0),  //
                    point3(-d, -d, -2.0), //
                    point3(d, -d, -2.0),  //
                ],
                vec![
                    point3(d, 2.0, -d),  //
                    point3(d, 2.0, d),   //
                    point3(-d, 2.0, d),  //
                    point3(-d, 2.0, -d), //
                    point3(d, 2.0, -d),  //
                ],
                /*
                vec![
                    point3(d, -d, -2.0),  //
                    point3(d, d, -2.0),   //
                    point3(-d, d, -2.0),  //
                    point3(-d, -d, -2.0), //
                    point3(d, -d, -2.0),  //
                ],
                 */
            ];

            let width = 3.0;

            let curve_material = scene.insert_curve_material(CurveMaterialSpec::default());

            let curves = points
                .into_iter()
                .map(|points| {
                    Box::new(SceneCurveObject::new(
                        points
                            .into_iter()
                            .map(|p| CurvePoint { position: p, width })
                            .collect::<Vec<_>>(),
                        vec![TransformedCurveInstance {
                            id: CurveInstanceId(0),
                            scale: vec3(1.0, 1.0, 1.0),
                            rotation: Quat::from_axis_angle(Vec3::UNIT_Y, deg(0.0)),
                            position: Vec3::ZERO,
                        }],
                        curve_material,
                    )) as Box<dyn SceneCurve>
                })
                .collect::<Vec<Box<_>>>();

            scene.set_curves(curves);

            /*
            scene.set_curves(vec![
                Box::new(SceneCurveObject::new(
                    points1
                        .into_iter()
                        .map(|p| CurvePoint { position: p, width })
                        .collect::<Vec<_>>(),
                    vec![TransformedCurveInstance {
                        id: CurveInstanceId(0),
                        scale: vec3(1.0, 1.0, 1.0),
                        rotation: Quat::from_axis_angle(Vec3::UNIT_Y, deg(0.0)),
                        position: Vec3::ZERO,
                    }],
                    curve_material,
                )),
                Box::new(SceneCurveObject::new(
                    points2
                        .into_iter()
                        .map(|p| CurvePoint { position: p, width })
                        .collect::<Vec<_>>(),
                    vec![TransformedCurveInstance {
                        id: CurveInstanceId(0),
                        scale: vec3(1.0, 1.0, 1.0),
                        rotation: Quat::from_axis_angle(Vec3::UNIT_Y, deg(0.0)),
                        position: Vec3::ZERO,
                    }],
                    curve_material,
                )),
            ]);
                 */

            scene.ambient_light(AmbientLight::new(rgb(0.1, 0.1, 0.1)));

            scene
        };

        let directional_lights = vec![
            DirectionalLight::new(vec3(-1.0, -1.0, 2.0), rgb(2.0, 2.0, 2.0)),
            DirectionalLight::new(vec3(1.0, -1.0, 2.0), rgb(1.0, 1.0, 1.5)),
            DirectionalLight::new(vec3(0.0, 1.0, 0.0), rgb(1.5, 1.5, 1.0)),
        ];

        Self {
            visual_renderer,
            position_renderer,
            camera_controller,
            scene,
            needs_position_update: true,
            directional_lights,
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
        // Change the direction of the directional lights so they stay the
        // same relative to the camera.
        {
            let view_matrix = self
                .camera_controller
                .camera()
                .view_rotation_matrix()
                .transpose();

            self.scene.set_directional_lights(
                self.directional_lights
                    .iter()
                    .map(|l| DirectionalLight {
                        direction: l.direction.into_point().transform(view_matrix).into_vec(),
                        color: l.color.clone(),
                    })
                    .collect(),
            );
        }
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
