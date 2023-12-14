mod editor;

use editor::{EditorState, EditorUi};
use eframe::{
    egui,
    egui_wgpu::WgpuConfiguration,
    wgpu::{self, Features},
    Renderer,
};
use geometry::{Curve3, Curve3Distance, Curve3Impl, Helix, Surface3, Surface3Impl};
use render::{
    color::rgb,
    light::{AmbientLight, DirectionalLight},
    model::{
        CurveId, CurveMaterialId, CurveMaterialSpec, CurveMesh, ModelInstance, PointMaterialId,
        PointMaterialSpec, SceneCurve, SceneModel, ScenePoint, SceneSurface, SurfaceMaterialId,
        SurfaceMaterialSpec,
    },
    render::MsaaSamples,
    scene::{self, Scene},
};
use space::{deg, point3, vec3, Point3, Quat, Vec3};
use std::{
    cell::OnceCell,
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
    ]);

    // Define materials
    let surface_material = scene.materials_mut().insert_surface_material(
        SurfaceMaterialSpec::default().roughness_rgb(rgb(0.4, 0.4, 0.4)), //.roughness_rgb(rgb(0.8, 0.8, 0.8)),
                                                                          //.metallic_rgb(rgb(1.0, 1.0, 1.0)),
    );

    let curve_material = scene
        .materials_mut()
        .insert_curve_material(CurveMaterialSpec::default());

    let point_material = scene
        .materials_mut()
        .insert_point_material(PointMaterialSpec::default());

    let helix = Curve3::helix(
        1.0,
        0.4,
        10.0,
        Quat::ZERO,
        vec3(5.0, 0.0, 0.0), //Quat::from_axis_angle(vec3(1.0, 1.0, 1.0).normalize(), deg(32.7)),
                             //vec3(-2.0, 0.0, -5.0),
    );

    //let profile = Curve3::helix(1.0, 0.4, 0.25, Quat::ZERO, Vec3::ZERO);
    let profile = Curve3::arc(
        1.0,
        deg(90.0),
        Quat::from_axis_angle(Vec3::UNIT_X, deg(-90.0)),
        Vec3::ZERO,
    );
    //let profile = Curve3::line(point3(1.0, 0.0, 0.0), point3(1.0, 0.0, 1.0));
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
        deg(180.0),
        Quat::from_axis_angle(Vec3::UNIT_X, deg(90.0)),
        vec3(0.0, 0.0, 0.0),
    );

    println!("frenet = {:#?}", path.frenet(0.0));

    let sweep = Surface3::sweep(profile.clone(), path.clone());

    let mut points = vec![Point3::ZERO];
    const MAX: usize = 20;
    for i_u in 0..=MAX {
        let u = (i_u as f64 / MAX as f64) * profile.u_len();
        for i_v in 0..=MAX {
            let v = (i_v as f64 / MAX as f64) * path.u_len();
            points.push(sweep.eval(u, v));
        }
    }

    println!(
        "translation = {:?}",
        match &sweep {
            Surface3::Sweep(path) => path.path_translation(sweep.v_min()),
            _ => panic!("wrong curve"),
        }
    );

    println!(
        "rotation = {:#?}",
        match &sweep {
            Surface3::Sweep(path) => path.path_rotation(sweep.v_min()),
            _ => panic!("wrong curve"),
        }
    );

    let surfaces = vec![sweep];

    let curves = vec![
        //helix,
        profile,
        path,
        // Axes
        Curve3::line(Point3::ZERO, Vec3::UNIT_X.into_point()),
        Curve3::line(Point3::ZERO, Vec3::UNIT_Y.into_point()),
        Curve3::line(Point3::ZERO, Vec3::UNIT_Z.into_point()),
    ];

    // Build part
    let part = PartModel::new(surfaces, curves, points);

    scene.add_model(part.scene_model_by_dist_tolerance(
        0.001,
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

        for surface in self.surfaces.iter() {
            let mut tess = Surface3Tessellator::new(surface);
            tess.tess(tolerance);
            //tess.tessellate(0.02);
            scene_model.add_surface(SceneSurface::new(tess.mesh(), surface_material));
        }

        for curve in self.curves.iter() {
            let mut tess = Curve3Tesselator::new(curve);
            let start = Instant::now();
            tess.tessellate(tolerance);
            //tess.tesselate_to_dist(tolerance);
            let end = Instant::now();
            println!(
                "tess {} points in {}us",
                tess.vertices().len(),
                (end - start).as_micros()
            );
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

/*
/// Gives primitive/canonical mathematical curves a position and
/// orientation in space
#[derive(Debug)]
struct ModelCurve {
    curve: Curve3,
    translation: Vec3,
    orientation: Quat,
}
impl ModelCurve {
    fn u_min(&self) -> f64 {
        self.curve.u_min()
    }

    fn u_max(&self) -> f64 {
        self.curve.u_max()
    }

    fn u_len(&self) -> f64 {
        self.curve.u_len()
    }

    pub fn eval(&self, u: f64) -> Point3 {
        self.orientation * self.curve.eval(u) + self.translation
    }

    pub fn der1(&self, u: f64) -> Vec3 {
        self.orientation * self.curve.der1(u)
    }

    pub fn der2(&self, u: f64) -> Vec3 {
        self.orientation * self.curve.der2(u)
    }
}

*/
