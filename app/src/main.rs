mod themes;
mod widgets;

use eframe::egui::{self, Margin, Visuals};
use eframe::egui_wgpu::WgpuConfiguration;
use eframe::{epaint, wgpu, Theme};
use std::collections::VecDeque;
use std::sync::Arc;
use themes::AppTheme;

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

enum File {
    Part(PartFile),
}

struct PartFile {
    path: Option<String>,
}
impl PartFile {
    pub fn new() -> Self {
        Self { path: None }
    }
}

struct State {
    theme: AppTheme,
    open_files: Vec<File>,
}
impl State {
    pub fn new() -> Self {
        Self {
            theme: AppTheme::Dark,
            open_files: vec![],
        }
    }

    pub fn new_part_file(&mut self) {
        self.open_files.push(File::Part(PartFile::new()));
    }
}

#[derive(Debug)]
enum Command {
    Exit,
    NewPart,
}

struct ShareCad {
    commands: VecDeque<Command>,
    state: State,
}
impl ShareCad {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            commands: VecDeque::new(),
            state: State::new(),
        }
    }
}
impl ShareCad {
    fn handle_commands(&mut self, frame: &mut eframe::Frame) {
        while let Some(command) = self.commands.pop_back() {
            println!("CMD {:?}", command);
            match command {
                Command::Exit => frame.close(),
                Command::NewPart => {
                    self.state.new_part_file();
                }
            }
        }
    }

    fn command(&mut self, command: Command) {
        self.commands.push_back(command);
    }

    fn visuals(&self) -> Visuals {
        self.state.theme.visuals()
    }
}
impl eframe::App for ShareCad {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.set_visuals(self.visuals());

        egui::CentralPanel::default()
            .frame(egui::Frame::none())
            .show(ctx, |ui| {
                egui::Frame::none()
                    .fill(self.visuals().widgets.active.bg_fill)
                    .inner_margin(Margin::symmetric(1.0, 1.0))
                    .outer_margin(Margin::ZERO)
                    .show(ui, |ui| {
                        egui::menu::bar(ui, |ui| {
                            ui.menu_button("File", |ui| {
                                if ui.button("New Part").clicked() {
                                    self.command(Command::NewPart);
                                    ui.close_menu();
                                }

                                ui.separator();

                                if ui.button("Exit").clicked() {
                                    self.command(Command::Exit);
                                }
                            });
                        });
                    });

                ui.heading("Hello world!");
            });

        self.handle_commands(frame);
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn on_close_event(&mut self) -> bool {
        true
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {}

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).to_normalized_gamma_f32()

        // _visuals.window_fill() would also be a natural choice
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }

    fn warm_up_enabled(&self) -> bool {
        false
    }

    fn post_rendering(&mut self, _window_size_px: [u32; 2], _frame: &eframe::Frame) {}
}
