mod editor;

use editor::{EditorState, EditorUi};
use eframe::{
    egui,
    egui_wgpu::WgpuConfiguration,
    wgpu::{self, Features},
    Renderer,
};
use geometry::{
    math::{deg, point2, point3, vec3, Quat, Vec3},
    tessellate::TessellationTolerance,
};
use geometry::{primitives::ISurfacePoint, IGeometry};
use model::PrimitiveModel;
use render::{
    light::{AmbientLight, DirectionalLight},
    model::SceneModel,
    render::MsaaSamples,
    scene::Scene,
};
use std::{f64::consts::TAU, sync::Arc, time::Instant};
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
            .transmit_rgb(rgb(0.5, 0.5, 0.5))
            .semigloss(),
    );

    let sweep2_material = scene.materials_mut().insert_surface_material(
        SurfaceMaterialSpec::default()
            .color(Rgb::PALE_TAUPE)
            .semigloss(),
    );

    let projection_point_material = scene
        .materials_mut()
        .insert_point_material(PointMaterialSpec::default().color_rgb(rgb(0.0, 0.0, 1.0)));

    let projected_point_material = scene
        .materials_mut()
        .insert_point_material(PointMaterialSpec::default().color_rgb(rgb(0.0, 1.0, 0.0)));

    let inverted_point_material = scene
        .materials_mut()
        .insert_point_material(PointMaterialSpec::default().color_rgb(rgb(1.0, 0.0, 0.0)));

    let mut model = PrimitiveModel::new();

    {
        let profile = model.create_arc(
            2.0,
            deg(180.0),
            Quat::from_axis_angle(Vec3::UNIT_Z, deg(-90.0)),
            Vec3::ZERO,
        );

        let path = model.create_arc(
            2.0,
            deg(360.0),
            Quat::from_axis_angle(Vec3::UNIT_X, deg(90.0)),
            Vec3::ZERO,
        );

        let profile = model.create_arc(
            0.5,
            deg(360.0),
            Quat::from_axis_angle(Vec3::UNIT_Z, deg(-90.0)),
            vec3(1.0, 0.0, 0.0),
        );

        let path = model.create_helix(
            2.0,
            1.2 / TAU,
            5.0,
            Quat::from_axis_angle(Vec3::UNIT_X, deg(90.0)),
            Vec3::ZERO,
        );

        let sweep = model.create_sweep(profile, path);
        model.set_surface_material(sweep, sweep1_material);

        let origin = model.create_point(point3(0.0, 0.0, 0.0));
        let x_extent = model.create_point(point3(3.0, 0.0, 0.0));
        let y_extent = model.create_point(point3(0.0, 3.0, 0.0));
        let z_extent = model.create_point(point3(0.0, 0.0, 3.0));
        model.create_line_between(origin, x_extent);
        model.create_line_between(origin, y_extent);
        model.create_line_between(origin, z_extent);

        //let projection_point = model.create_point(point3(1.5, 1.5, -1.5));
        let projection_point = model.create_point(point3(0.0, -0.01, 0.0));
        let solver = model.surface_solver(sweep).unwrap();

        solver.projection_starting_params(*model.point(projection_point).unwrap(), true, true);

        let start = Instant::now();
        let projections = solver.project_point(*model.point(projection_point).unwrap());
        let end = Instant::now();
        println!(
            "{} projections in {}",
            projections.len(),
            (end - start).as_micros()
        );

        for projection in projections {
            let id = model.create_point(projection.pos);
            model.set_point_material(id, projected_point_material);
        }

        model.set_point_material(projection_point, projection_point_material);
    }

    let sm = SceneModel::from_primitive_model(
        &model,
        //&TessellationTolerance::DistanceAndAngle(0.001, deg(3.0)),
        &TessellationTolerance::DistanceAndAngle(0.1, deg(5.0)),
    );

    scene.add_model(sm);

    scene
}
