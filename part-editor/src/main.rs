mod editor;

use editor::{EditorState, EditorUi};
use eframe::{
    egui,
    egui_wgpu::WgpuConfiguration,
    wgpu::{self, Features},
    Renderer,
};
use geometry::{Curve, Curve3, Curve3Impl, Surface, SurfaceIntersection, Sweep, SweepSurface};
use render::{
    color::rgb,
    light::{AmbientLight, DirectionalLight},
    model::{
        CurveMaterialId, CurveMaterialSpec, CurveMesh, PointMaterialId, PointMaterialSpec,
        SceneCurve, SceneModel, ScenePoint, SceneSurface, SurfaceMaterialId, SurfaceMaterialSpec,
    },
    render::MsaaSamples,
    scene::Scene,
};
use space::{deg, point2, point3, vec3, Point3, Quat, Vec3};
use std::{
    f64::consts::{PI, TAU},
    sync::Arc,
    time::Instant,
};
use tessellate::{Curve3Tesselator, SurfacePointTessellator};

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let get_device_descriptor = |_adapter: &wgpu::Adapter| -> wgpu::DeviceDescriptor<'static> {
        wgpu::DeviceDescriptor {
            features: Features::POLYGON_MODE_LINE,
            ..Default::default()
        }
    };

    let msaa_samples = MsaaSamples::Samples4;

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1000.0, 1024.0)),
        renderer: Renderer::Wgpu,
        wgpu_options: WgpuConfiguration {
            device_descriptor: Arc::new(get_device_descriptor),
            ..Default::default()
        },
        multisampling: 1,
        ..Default::default()
    };

    let mut editor_state: Option<EditorState> = None;

    eframe::run_simple_native("Part Editor", options, move |ctx, frame| {
        let editor_state_left = editor_state
            .get_or_insert_with(|| EditorState::new(ctx, frame, msaa_samples, build_scene()));

        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            if ui.button("Sketch").clicked() {
                println!("Sketch");
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.editor(editor_state_left);
        });
    })
}

fn build_scene() -> Scene {
    let mut scene = Scene::new();

    scene.set_ambient_light(AmbientLight::new(rgb(0.1, 0.1, 0.1)));
    scene.set_camera_directional_lights(vec![
        DirectionalLight::new(vec3(-1.0, -1.0, 2.0), rgb(2.0, 2.0, 2.0)),
        DirectionalLight::new(vec3(1.0, -1.0, 2.0), rgb(1.0, 1.0, 1.5)),
        DirectionalLight::new(vec3(0.0, 1.0, 0.0), rgb(1.5, 1.5, 1.0)),
    ]);

    // Define materials
    let surface_material = scene.materials_mut().insert_surface_material(
        SurfaceMaterialSpec::default()
            .transmit_rgb(rgb(0.5, 0.5, 0.5))
            .roughness_rgb(rgb(0.4, 0.4, 0.4))
            .metallic_rgb(rgb(0.2, 0.2, 0.2)),
    );

    let curve_material = scene
        .materials_mut()
        .insert_curve_material(CurveMaterialSpec::default().color_rgb(rgb(0.0, 0.0, 1.0)));

    let point_material = scene
        .materials_mut()
        .insert_point_material(PointMaterialSpec::default().color_rgb(rgb(1.0, 0.0, 0.0)));

    /*
    let profile0 = Curve::arc(
        1.0,
        deg(180.0),
        Quat::from_axis_angle(Vec3::UNIT_Z, deg(-90.0)),
        vec3(0.0, 3.0, 0.0),
    );
    let path0 = Curve::arc(
        1.0,
        deg(360.0),
        Quat::from_axis_angle(Vec3::UNIT_X, deg(90.0)),
        vec3(0.0, 3.0, 0.0),
    );
    */
    let profile0 = Curve::line(point3(0.0, 0.0, 0.0), point3(0.0, 1.0, 0.0));
    let path0 = Curve::line(point3(0.0, 0.0, 0.0), point3(0.0, 0.0, 3.0));
    let surf0 = Surface::sweep(profile0.clone(), path0.clone());

    /*
    let profile1 = Curve::arc(
        1.0,
        deg(180.0),
        Quat::from_axis_angle(Vec3::UNIT_Z, deg(-90.0)),
        vec3(1.0, 4.0, 1.0),
    );
    let path1 = Curve::arc(
        1.0,
        deg(360.0),
        Quat::from_axis_angle(Vec3::UNIT_X, deg(90.0)),
        vec3(1.0, 4.0, 1.0),
    );
     */
    let profile1 = Curve::line(point3(2.0, 0.0, 2.0), point3(2.0, 1.0, 2.0));
    let path1 = Curve::line(point3(2.0, 0.0, 2.0), point3(-1.0, 0.0, 2.0));
    let surf1 = Surface::sweep(profile1.clone(), path1.clone());

    let intersection = SurfaceIntersection::new(&surf0, &surf1);
    //let mut s0_params = vec![point2(2.0, 0.0)];
    //let mut s1_params = vec![point2(1.4, PI + 0.5)];
    let mut s0_params = vec![point2(0.5, 0.5)];
    let mut s1_params = vec![point2(0.5, 0.5)];

    for i in 0..30 {
        let (new_s0_param, new_s1_param) =
            intersection.next(*s0_params.last().unwrap(), *s1_params.last().unwrap());
        s0_params.push(new_s0_param);
        s1_params.push(new_s1_param);
    }

    let points = vec![Point3::ZERO]
        .into_iter()
        .chain(s0_params.into_iter().map(|uv| *surf0.point(uv).eval()))
        .chain(s1_params.into_iter().map(|uv| *surf1.point(uv).eval()))
        .collect();
    let surfaces = vec![surf0, surf1];
    let curves = vec![
        profile0,
        path0,
        profile1,
        path1,
        Curve::line(point3(0.0, 0.0, 1.0), point3(0.0, 1.0, 1.0)),
        Curve::line(point3(0.0, 0.5, 0.0), point3(0.0, 0.5, 3.0)),
        Curve::line(Point3::ZERO, Vec3::UNIT_X.into_point()),
        Curve::line(Point3::ZERO, Vec3::UNIT_Y.into_point()),
        Curve::line(Point3::ZERO, Vec3::UNIT_Z.into_point()),
    ];

    // Build part
    let part = PartModel::new(surfaces, curves, points);

    const TOLERANCE: f64 = 0.0001;

    scene.add_model(part.scene_model_by_dist_tolerance(
        TOLERANCE,
        surface_material,
        curve_material,
        point_material,
    ));

    scene
}

// BREP model
struct PartModel {
    surfaces: Vec<Surface>,
    curves: Vec<Curve>,
    points: Vec<Point3>,
}
impl PartModel {
    pub fn new(surfaces: Vec<Surface>, curves: Vec<Curve>, points: Vec<Point3>) -> Self {
        Self {
            surfaces,
            curves,
            points,
        }
    }

    pub fn scene_model_by_dist_tolerance(
        &self,
        tolerance: f64,
        surface_material: SurfaceMaterialId,
        curve_material: CurveMaterialId,
        point_material: PointMaterialId,
    ) -> SceneModel {
        let mut scene_model = SceneModel::new();

        let mut normal_lines = Vec::new();

        for surface in self.surfaces.iter() {
            let mut tess = SurfacePointTessellator::new(surface);
            let start = Instant::now();
            tess.tessellate(tolerance);
            let end = Instant::now();
            println!(
                "tessellated surface in {}us with {} vertices",
                (end - start).as_micros(),
                tess.num_points()
            );

            for vert in tess.mesh().vertices().iter() {
                let nl = Curve3::line(vert.position, vert.position + vert.normal);
                normal_lines.push(nl);
            }

            scene_model.add_surface(SceneSurface::new(tess.mesh(), surface_material));
        }

        for curve in self.curves.iter() {
            let mut tess = Curve3Tesselator::new(curve);
            tess.tessellate(tolerance);
            scene_model.add_curve(SceneCurve::new(tess.mesh(), curve_material, 1.0));
        }

        for point in self.points.iter() {
            scene_model.add_point(ScenePoint::new(point.clone(), point_material, 6.0));
        }

        scene_model
    }
}
