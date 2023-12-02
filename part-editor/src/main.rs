mod editor;

use editor::{EditorState, EditorUi};
use eframe::{
    egui,
    egui_wgpu::WgpuConfiguration,
    wgpu::{self, Features},
    Renderer,
};
use geometry::{Curve3, Curve3Impl};
use render::{
    color::Rgba,
    model::{CurveInstance, CurveInstanceId, CurveMaterialId, CurveMesh, CurvePoint, PolyCurve},
    render::MsaaSamples,
};
use space::{deg, Point3, Quat, Vec3};
use std::{cell::OnceCell, sync::Arc};

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
        multisampling: 1, // msaa_samples.samples() as u16,
        ..Default::default()
    };

    let mut editor_state: Option<EditorState> = None;

    eframe::run_simple_native("Part Editor", options, move |ctx, frame| {
        let editor_state_left =
            editor_state.get_or_insert_with(|| EditorState::new(ctx, frame, msaa_samples));

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

// BREP model
struct PartModel {
    curves: Vec<Curve3>,
}
impl PartModel {
    pub fn new(curves: Vec<Curve3>) -> Self {
        Self { curves }
    }
}

#[derive(Debug)]
struct ModelCurve {
    curve: Curve3,
    translation: Vec3,
    orientation: Quat,

    poly_curve: OnceCell<PolyCurve>,
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

    fn poly_curve(&self) -> &PolyCurve {
        self.poly_curve.get_or_init(|| {
            let u_min = self.u_min();
            let u_max = self.u_max();
            let param_interval = self.u_len() / Self::NUM_SEGMENTS as f64;

            let mut points = Vec::with_capacity(Self::NUM_SEGMENTS as usize + 1);
            for i in 0..=Self::NUM_SEGMENTS {
                let u = match i {
                    0 => u_min,
                    i if i == Self::NUM_SEGMENTS => u_max,
                    i => u_min + param_interval * i as f64,
                };

                points.push(CurvePoint {
                    position: self.eval(u),
                    width: 1.5,
                });
            }

            let mesh = CurveMesh::new(points);

            PolyCurve::new(
                mesh,
                vec![CurveInstance {
                    id: CurveInstanceId(0),
                    rotation: Quat::from_axis_angle(Vec3::UNIT_Y, deg(0.0)),
                    position: Vec3::ZERO,
                    tint: Rgba::TRANSPARENT,
                }],
                CurveMaterialId(0),
            )
        })
    }
}
