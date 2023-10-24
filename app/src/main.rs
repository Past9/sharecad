mod themes;
mod widgets;

use eframe::egui::{self, Margin, Visuals};
use eframe::egui_wgpu::WgpuConfiguration;
use eframe::{epaint, wgpu, Theme};
use egui_tiles::{Behavior, TileId};
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

#[derive(Debug)]
struct FilePane {
    file_id: usize,
    name: String,
    kind: FileKindId,
}

struct TreeBehavior {}
impl egui_tiles::Behavior<FilePane> for TreeBehavior {
    fn pane_ui(
        &mut self,
        ui: &mut egui::Ui,
        _tile_id: egui_tiles::TileId,
        pane: &mut FilePane,
    ) -> egui_tiles::UiResponse {
        let text = match pane.kind {
            FileKindId::Part => format!("Part {}: {}", pane.file_id, pane.name),
            FileKindId::Assembly => format!("Assembly {}: {}", pane.file_id, pane.name),
        };

        ui.label(text);

        Default::default()
    }

    fn tab_title_for_pane(&mut self, pane: &FilePane) -> egui::WidgetText {
        match pane.kind {
            FileKindId::Part => format!("Part {}: {}", pane.file_id, pane.name),
            FileKindId::Assembly => format!("Assembly {}: {}", pane.file_id, pane.name),
        }
        .into()
    }

    fn simplification_options(&self) -> egui_tiles::SimplificationOptions {
        egui_tiles::SimplificationOptions {
            prune_empty_containers: false,
            prune_empty_tabs: false,
            prune_single_child_tabs: false,
            prune_single_child_containers: false,
            all_panes_must_have_tabs: false,
            join_nested_linear_containerss: false,
        }
    }
}

#[derive(Debug)]
struct File {
    id: usize,
    kind: FileKind,
}

#[derive(Debug)]
enum FileKindId {
    Part,
    Assembly,
}

#[derive(Debug)]
enum FileKind {
    Part(PartFile),
    Assembly(AssemblyFile),
}
impl FileKind {
    fn path(&self) -> Option<String> {
        match self {
            FileKind::Part(part) => part.path.clone(),
            FileKind::Assembly(assembly) => assembly.path.clone(),
        }
    }
}

#[derive(Debug)]
struct PartFile {
    path: Option<String>,
}
impl PartFile {
    pub fn new() -> Self {
        Self { path: None }
    }
}

#[derive(Debug)]
struct AssemblyFile {
    path: Option<String>,
}
impl AssemblyFile {
    pub fn new() -> Self {
        Self { path: None }
    }
}

#[derive(Debug)]
struct State {
    theme: AppTheme,
    open_files: Vec<File>,
    root_id: TileId,
    tree: egui_tiles::Tree<FilePane>,
}
impl State {
    pub fn new() -> Self {
        let mut tiles = egui_tiles::Tiles::<FilePane>::default();
        let root = egui_tiles::Container::new_tabs(vec![]);
        let root_id = tiles.insert_container(root);
        let tree = egui_tiles::Tree::new(root_id, tiles);
        println!("INIT TREE {:#?}", tree);
        let state = Self {
            theme: AppTheme::Dark,
            open_files: vec![],
            root_id,
            tree,
        };

        println!("STATE {:#?}", state);

        state
    }

    pub fn new_part_file(&mut self) {
        self.open_files.push(File {
            id: self.open_files.len(),
            kind: FileKind::Part(PartFile::new()),
        });
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

                    println!("CHECK STATE 1 {:#?}", self.state);
                    println!("CHECK TREE 1 {:#?}", self.state.tree);

                    let file_id = self.state.open_files.len();

                    let tile_id = self.state.tree.tiles.insert_pane(FilePane {
                        file_id,
                        name: match &self.state.open_files[file_id - 1].kind {
                            FileKind::Part(part) => match &part.path {
                                Some(path) => format!("Part {}: {}", file_id, path),
                                None => format!("Part {} (Untitled)", file_id),
                            },
                            FileKind::Assembly(assembly) => todo!(),
                        },
                        kind: FileKindId::Part,
                    });

                    if let Some(egui_tiles::Tile::Container(egui_tiles::Container::Tabs(tabs))) =
                        self.state.tree.tiles.get_mut(self.state.root_id)
                    {
                        tabs.add_child(tile_id);
                        tabs.set_active(tile_id);
                    } else {
                        println!("CHECK STATE 2 {:#?}", self.state);
                        println!("CHECK TREE 2 {:#?}", self.state.tree);
                        println!("{:#?}", self.state.tree.tiles.get_mut(self.state.root_id));
                    }
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

                /*
                let mut tiles = egui_tiles::Tiles::default();

                let tile_ids = self
                    .state
                    .open_files
                    .iter()
                    .map(|file| {
                        tiles.insert_pane(FilePane {
                            file_id: file.id,
                            name: match file.kind.path() {
                                Some(path) => path,
                                None => "Untitled".to_string(),
                            },
                            kind: match file.kind {
                                FileKind::Part(_) => FileKindId::Part,
                                FileKind::Assembly(_) => FileKindId::Assembly,
                            },
                        })
                    })
                    .collect::<Vec<_>>();

                let root = tiles.insert_tab_tile(tile_ids);

                let mut tree = egui_tiles::Tree::new(root, tiles);
                     */

                let mut behavior = TreeBehavior {};

                println!("CHECK STATE -1 {:#?}", self.state);
                self.state.tree.ui(&mut behavior, ui);
                println!("CHECK STATE 0 {:#?}", self.state);

                //ui.heading("Hello world!");
            });

        self.handle_commands(frame);
    }
}
