mod editor;

use editor::{EditorState, EditorUi};
use eframe::{
    egui,
    egui_wgpu::WgpuConfiguration,
    wgpu::{self, Features},
    Renderer,
};
use geometry::math::{deg, point2, point3, vec3, Quat, Vec3};
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
        let sweep1_quat = Quat::from_axis_angle(vec3(1.0, -0.25, -0.25), deg(-60.0));
        let sweep1_loc = vec3(-1.0, -1.0, 0.0);
        let sweep1_path = model.create_arc(2.0, deg(90.0), sweep1_quat, sweep1_loc);
        let sweep1_profile = model.create_arc(
            2.0,
            deg(60.0),
            sweep1_quat * Quat::from_axis_angle(Vec3::UNIT_X, deg(90.0)),
            sweep1_loc,
        );
        let sweep1 = model.create_sweep(sweep1_profile, sweep1_path);
        model.set_surface_material(sweep1, sweep1_material);

        let projection_point = model.create_point(point3(0.0, 1.0, -0.5));
        let solver = model.surface_solver(sweep1).unwrap();

        let projections = solver.project_point(*model.point(projection_point).unwrap());

        println!("projections = {:#?}", projections);

        for projection in projections {
            let id = model.create_point(projection.pos);
            model.set_point_material(id, projected_point_material);
        }

        model.set_point_material(projection_point, projection_point_material);
    }

    let sm = SceneModel::from_primitive_model(&model, TOLERANCE);

    scene.add_model(sm);

    scene
}
