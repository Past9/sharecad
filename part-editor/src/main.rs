mod editor;

use editor::{EditorState, EditorUi};
use eframe::{
    egui,
    egui_wgpu::WgpuConfiguration,
    wgpu::{self, Features},
    Renderer,
};
use geometry::{Curve3, Curve3Impl, Surface3, Surface3Impl};
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
use space::{deg, point3, vec3, Point3, Quat, Vec3};
use std::{
    f64::consts::{PI, TAU},
    sync::Arc,
    time::Instant,
};
use tessellate::{Curve3Tesselator, Surface3Tessellator};

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
        //DirectionalLight::new(vec3(0.0, 0.0, 1.0), rgb(1.0, 0.0, 0.0)),
    ]);

    // Define materials
    let surface_material = scene.materials_mut().insert_surface_material(
        SurfaceMaterialSpec::default()
            .transmit_rgb(rgb(0.5, 0.5, 0.5))
            .roughness_rgb(rgb(0.4, 0.4, 0.4)) //.roughness_rgb(rgb(0.8, 0.8, 0.8)),
            .metallic_rgb(rgb(0.1, 0.1, 0.1)),
    );

    let curve_material = scene
        .materials_mut()
        .insert_curve_material(CurveMaterialSpec::default());

    let point_material = scene
        .materials_mut()
        .insert_point_material(PointMaterialSpec::default());

    let helix = Curve3::helix(
        1.0,
        0.1 + 2.0 / TAU,
        20.0,
        Quat::from_axis_angle(Vec3::UNIT_X, deg(90.0)),
        vec3(-2.0, 0.0, 0.0), //Quat::from_axis_angle(vec3(1.0, 1.0, 1.0).normalize(), deg(32.7)),
                              //vec3(-2.0, 0.0, -5.0),
    );

    //let profile = Curve3::helix(1.0, 0.4, 0.25, Quat::ZERO, Vec3::ZERO);
    let profile = Curve3::arc(
        1.0,
        deg(180.0),
        //Quat::from_axis_angle(Vec3::UNIT_X, deg(-90.0)),
        Quat::from_axis_angle(Vec3::UNIT_Z, deg(-90.0)),
        //Quat::ZERO,
        //Vec3::ZERO,
        vec3(0.0, 0.0, 0.0),
    );
    //let profile = Curve3::line(point3(1.0, 0.0, 0.0), point3(1.0, 1.0, 0.0));
    /*
    let path = Curve3::helix(
        1.0,
        0.4,
        0.25,
        Quat::from_axis_angle(Vec3::UNIT_X, deg(90.0)),
        vec3(-1.0, 0.0, 0.0),
    );
    */
    let path = Curve3::arc(
        1.0,
        deg(360.0),
        Quat::from_axis_angle(Vec3::UNIT_X, deg(90.0)),
        vec3(0.0, 0.0, 0.0),
    );

    let line = Curve3::line(point3(0.0, 0.0, 0.0), point3(0.0, 0.0, 5.0));

    let sweep = Surface3::sweep(profile.clone(), path.clone());

    //let uv = (PI, 2.0 * PI);
    let uv = (0.0, 0.0);
    let test_point = sweep.eval(uv.0, uv.1);
    let n = sweep.normal(uv.0, uv.1).normalize();
    let (du, dv) = sweep.der1(uv.0, uv.1);
    let du_norm = du.normalize();
    let dv_norm = dv.normalize();

    let correct_normal = du.cross(dv)
        / (du.magnitude().powi(2) * dv.magnitude().powi(2) - du.dot(dv).powi(2)).sqrt();
    println!("correct_normal = {}", correct_normal);

    println!("test_point = {}", test_point);
    println!("n = {}", n);
    println!("du = {}", du);
    println!("dv = {}", dv);
    println!("du_norm = {}", du_norm);
    println!("dv_norm = {}", dv_norm);

    println!("dv_fixed = {}", n.cross(du_norm));
    println!("du_fixed = {}", dv_norm.cross(n));

    let normal_line = Curve3::line(test_point, test_point + n);
    let du_line = Curve3::line(test_point, test_point + du_norm);
    let dv_line = Curve3::line(test_point, test_point + dv_norm);

    const TOLERANCE: f64 = 0.01;

    let mut tess = Surface3Tessellator::new(&sweep);
    let param_points = tess
        .tess_uvs(TOLERANCE)
        .into_iter()
        .flat_map(|row| row.into_iter())
        .map(|uv| point3(uv.x, uv.y, 3.0))
        .collect::<Vec<_>>();

    let points = vec![Point3::ZERO, test_point]
        .into_iter()
        .chain(param_points.into_iter())
        .collect();

    let surfaces = vec![sweep];

    let curves = vec![
        //helix,
        profile,
        path,
        normal_line,
        du_line,
        dv_line, // Axes
                 /*
                 Curve3::line(Point3::ZERO, Vec3::UNIT_X.into_point()),
                 Curve3::line(Point3::ZERO, Vec3::UNIT_Y.into_point()),
                 Curve3::line(Point3::ZERO, Vec3::UNIT_Z.into_point()),
                  */
    ];

    // Build part
    let part = PartModel::new(surfaces, curves, points);

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
    surfaces: Vec<Surface3>,
    curves: Vec<Curve3>,
    points: Vec<Point3>,
}
impl PartModel {
    pub fn new(surfaces: Vec<Surface3>, curves: Vec<Curve3>, points: Vec<Point3>) -> Self {
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
            let mut tess = Surface3Tessellator::new(surface);
            let start = Instant::now();
            tess.tess(tolerance);
            let end = Instant::now();
            println!("tess surface in {}us", (end - start).as_micros());

            for vert in tess.mesh().vertices().iter() {
                let nl = Curve3::line(vert.position, vert.position + vert.normal);
                normal_lines.push(nl);
            }

            //tess.tessellate(0.02);
            scene_model.add_surface(SceneSurface::new(tess.mesh(), surface_material));
        }

        for curve in normal_lines.iter() {
            let mut tess = Curve3Tesselator::new(curve);
            tess.tessellate(tolerance);
            scene_model.add_curve(SceneCurve::new(tess.mesh(), curve_material, 1.0));
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

    pub fn scene_model(
        &self,
        curve_material: CurveMaterialId,
        point_material: PointMaterialId,
    ) -> SceneModel {
        let mut scene_model = SceneModel::new();

        // TODO Surfaces

        for curve in self.curves.iter() {
            scene_model.add_curve(SceneCurve::new(
                tessellate_curve(&curve),
                curve_material,
                1.5,
            ));
        }

        for point in self.points.iter() {
            scene_model.add_point(ScenePoint::new(point.clone(), point_material, 6.0));
        }

        scene_model
    }
}

fn tessellate_curve(curve: &Curve3) -> CurveMesh {
    const NUM_SEGMENTS: u32 = 500;

    let u_min = curve.u_min();
    let u_max = curve.u_max();
    let param_interval = curve.u_len() / NUM_SEGMENTS as f64;

    let mut points = Vec::with_capacity(NUM_SEGMENTS as usize + 1);
    for i in 0..=NUM_SEGMENTS {
        let u = match i {
            0 => u_min,
            i if i == NUM_SEGMENTS => u_max,
            i => u_min + param_interval * i as f64,
        };

        points.push(curve.eval(u));
    }

    CurveMesh::new(points)
}
