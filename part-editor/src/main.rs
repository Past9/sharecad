mod editor;

use editor::{EditorState, EditorUi};
use eframe::{
    egui,
    egui_wgpu::WgpuConfiguration,
    wgpu::{self, Features},
    Renderer,
};
use geometry::math::{deg, point3, vec3, Quat, Vec3};
use geometry::IGeometry;
use model::PrimitiveModel;
use render::{
    light::{AmbientLight, DirectionalLight},
    model::SceneModel,
    render::MsaaSamples,
    scene::Scene,
};
use std::{sync::Arc, time::Instant};
use visual::{
    color::{rgb, Rgb},
    material::{PointMaterialSpec, SurfaceMaterialSpec},
    IGeometryVisuals,
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

    scene.set_ambient_light(AmbientLight::new(rgb(0.35, 0.35, 0.35)));
    scene.set_camera_directional_lights(vec![
        DirectionalLight::new(vec3(-1.0, -1.0, 2.0), rgb(2.0, 2.0, 2.0)),
        DirectionalLight::new(vec3(1.0, -1.0, 2.0), rgb(1.0, 1.0, 1.5)),
        DirectionalLight::new(vec3(0.0, 1.0, 0.0), rgb(1.5, 1.5, 1.0)),
    ]);

    // Define materials
    let sweep1_material = scene.materials_mut().insert_surface_material(
        SurfaceMaterialSpec::default()
            .color(Rgb::STEEL_BLUE)
            .semigloss(),
    );

    let sweep2_material = scene.materials_mut().insert_surface_material(
        SurfaceMaterialSpec::default()
            .color(Rgb::PALE_TAUPE)
            .semigloss(),
    );

    let default_point_material = scene
        .materials_mut()
        .insert_point_material(PointMaterialSpec::default().color_rgb(rgb(1.0, 0.5, 0.0)));
    scene
        .materials_mut()
        .set_default_point_material(default_point_material);

    let projection_point_material = scene
        .materials_mut()
        .insert_point_material(PointMaterialSpec::default().color_rgb(rgb(0.0, 0.0, 1.0)));

    let projected_point_material = scene
        .materials_mut()
        .insert_point_material(PointMaterialSpec::default().color_rgb(rgb(0.0, 1.0, 0.0)));

    let inverted_point_material = scene
        .materials_mut()
        .insert_point_material(PointMaterialSpec::default().color_rgb(rgb(1.0, 0.0, 0.0)));

    const TOLERANCE: f64 = 0.0001;

    let mut model = PrimitiveModel::new();

    {
        let sweep1_path = model.create_arc(1.0, deg(360.0), Quat::ZERO, Vec3::ZERO);

        // Test point projection
        let arc = model.curve_solver(sweep1_path).unwrap();
        let projection_point = point3(0.1, 0.0, 0.0);
        let projection_point_id = model.create_point(projection_point.clone());
        model.set_point_material(projection_point_id, projection_point_material);
        arc.project_point(projection_point);
        let start = Instant::now();
        let results = arc.project_point(projection_point);
        let end = Instant::now();

        println!("results = {:#?}", results);
        println!("res in {}us", (end - start).as_micros());

        for result in results.iter() {
            let id = model.create_point(result.pos);
            model.set_point_material(id, projected_point_material)
        }
    }

    let sm = SceneModel::from_primitive_model(&model, TOLERANCE);

    scene.add_model(sm);

    scene
}
