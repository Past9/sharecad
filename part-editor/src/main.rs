mod editor;

use editor::{EditorState, EditorUi};
use eframe::{
    egui,
    egui_wgpu::WgpuConfiguration,
    wgpu::{self, Features},
    Renderer,
};
use geometry::{Curve3, Curve3Impl, Helix};
use render::{
    color::rgb,
    light::{AmbientLight, DirectionalLight},
    model::{
        CurveId, CurveMaterialId, CurveMaterialSpec, CurveMesh, ModelInstance, SceneCurve,
        SceneModel,
    },
    render::MsaaSamples,
    scene::Scene,
};
use space::{deg, vec3, Point3, Quat, Vec3};
use std::{cell::OnceCell, f64::consts::TAU, sync::Arc};

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

    let curve_material = scene
        .materials_mut()
        .insert_curve_material(CurveMaterialSpec::default());

    let model_curve = ModelCurve {
        curve: Curve3::helix(1.0, 1.0 / TAU, 2.0),
        translation: vec3(1.0, 2.0, 3.0),
        orientation: Quat::from_axis_angle(vec3(1.0, 0.0, 0.0), deg(90.0)),
    };

    let part = PartModel::new(vec![model_curve]);

    scene.add_model(part.scene_model(curve_material));

    scene
}

// BREP model
struct PartModel {
    curves: Vec<ModelCurve>,
    scene_model: OnceCell<SceneModel>,
}
impl PartModel {
    pub fn new(curves: Vec<ModelCurve>) -> Self {
        Self {
            curves,
            scene_model: OnceCell::new(),
        }
    }

    pub fn scene_model(&self, material: CurveMaterialId) -> &SceneModel {
        self.scene_model.get_or_init(|| {
            let mut scene_model = SceneModel::new();

            // TODO Surfaces

            for curve in self.curves.iter() {
                scene_model.add_curve(SceneCurve::new(tessellate_curve(&curve), material, 1.5));
            }

            // TODO Points

            scene_model
        })
    }
}

fn tessellate_curve(curve: &ModelCurve) -> CurveMesh {
    const NUM_SEGMENTS: u32 = 100;

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

/// Gives primitive/canonical mathematical curves a position and
/// orientation in space
#[derive(Debug)]
struct ModelCurve {
    curve: Curve3,
    translation: Vec3,
    orientation: Quat,
}
impl ModelCurve {
    const NUM_SEGMENTS: u32 = 100;

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
