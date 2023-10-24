use std::sync::Arc;

use eframe::egui;
use eframe::egui_wgpu::WgpuConfiguration;
use eframe::{epaint, wgpu, Theme};

fn main() {
    let native_options = eframe::NativeOptions {
        drag_and_drop_support: true,
        icon_data: None,
        initial_window_pos: None,
        initial_window_size: Some(epaint::Vec2::new(1600.0, 900.0)),
        min_window_size: Some(epaint::Vec2::new(400.0, 400.0)),
        max_window_size: None,
        vsync: true,
        multisampling: 0,
        hardware_acceleration: eframe::HardwareAcceleration::Preferred,
        renderer: eframe::Renderer::Wgpu,
        follow_system_theme: true,
        default_theme: Theme::Dark,
        run_and_return: true,
        event_loop_builder: None,
        window_builder: None,
        shader_version: None,
        centered: true,
        wgpu_options: WgpuConfiguration {
            supported_backends: wgpu::Backends::all(),
            device_descriptor: Arc::new(|_adapter| wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
            }),
            power_preference: wgpu::PowerPreference::HighPerformance,
            on_surface_error: Arc::new(|err| {
                //
                panic!("SurfaceError: {}", err)
            }),
            ..Default::default()
        },
        app_id: Some("ShareCAD".into()),
        persist_window: false,
        ..Default::default()
    };

    eframe::run_native(
        "ShareCAD",
        native_options,
        Box::new(|cc| Box::new(ShareCad::new(cc))),
    )
    .unwrap();
}

struct ShareCad {}
impl ShareCad {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {}
    }
}
impl eframe::App for ShareCad {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello world!");
        });
    }
}
