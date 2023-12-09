mod editor;

use editor::{EditorState, EditorUi};
use eframe::{
    egui,
    egui_wgpu::WgpuConfiguration,
    wgpu::{self, Features},
    Renderer,
};
use geometry::{Curve3, Curve3Distance, Curve3Impl, Helix};
use render::{
    color::rgb,
    light::{AmbientLight, DirectionalLight},
    model::{
        CurveId, CurveMaterialId, CurveMaterialSpec, CurveMesh, ModelInstance, PointMaterialId,
        PointMaterialSpec, SceneCurve, SceneModel, ScenePoint,
    },
    render::MsaaSamples,
    scene::Scene,
};
use space::{deg, point3, vec3, Point3, Quat, Vec3};
use std::{
    cell::OnceCell,
    f64::consts::{PI, TAU},
    sync::Arc,
    time::Instant,
};

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

    // Define curves
    let curve_material = scene
        .materials_mut()
        .insert_curve_material(CurveMaterialSpec::default());

    let c0 = Curve3::helix(1.0, 0.2, 5.0, Quat::ZERO, Vec3::ZERO);
    let c1 = Curve3::helix(
        1.0,
        0.4,
        5.0,
        Quat::from_axis_angle(vec3(1.0, 1.0, 1.0).normalize(), deg(32.7)),
        vec3(-2.0, 0.0, -5.0),
    );
    //let c1 = Curve3::line(point3(1.0, -5.0, PI + 1.0), point3(3.0, 5.0, PI - 1.0));

    let start = Instant::now();
    let results = Curve3::distance_extrema(&c0, &c1);
    let end = Instant::now();
    println!("{}us", (end - start).as_micros());

    let results = Curve3Distance::dedup(results);

    println!("results.len() = {}", results.len());

    /*
    let helix = Curve3::helix(
        1.0,
        1.0 / TAU,
        10.0,
        Quat::from_axis_angle(vec3(0.0, 1.0, 0.0), deg(45.0)),
        vec3(3.0, 0.0, 0.0),
    );

    let line = Curve3::line(point3(1.0, 1.0, 1.0), point3(-1.0, -1.0, -1.0));
     */

    let line0 = c0.eval(c0.u_min());
    let line1 = c0.eval(c0.u_max());
    let line = Curve3::line(line0, line1);

    let hd = c0.line_deviation(c0.u_min(), c0.u_max()).unwrap();
    let hd_line = Curve3::line(hd.cu_pos, hd.cv_pos);

    // Define points
    let point_material = scene
        .materials_mut()
        .insert_point_material(PointMaterialSpec::default());

    let origin = Point3::ZERO;

    /*
    let upper = point3(0.0, 3.0, 0.0);
    let helix_start = helix.eval(helix.u_min());
    let helix_end = helix.eval(helix.u_max());
     */

    let mut curves = vec![c0, c1, line, hd_line];
    let mut points = vec![origin];

    let shortest = Curve3Distance::shortest(&results).unwrap();
    curves.push(Curve3::line(shortest.cu_pos, shortest.cv_pos));

    let longest = Curve3Distance::longest(&results).unwrap();
    curves.push(Curve3::line(longest.cu_pos, longest.cv_pos));

    /*
    for res in results.iter() {
        curves.push(Curve3::line(res.cu_pos, res.cv_pos));
    }
     */

    // Build part
    let part = PartModel::new(curves, points);

    scene.add_model(part.scene_model(curve_material, point_material));

    scene
}

// BREP model
struct PartModel {
    curves: Vec<Curve3>,
    points: Vec<Point3>,
}
impl PartModel {
    pub fn new(curves: Vec<Curve3>, points: Vec<Point3>) -> Self {
        Self { curves, points }
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
