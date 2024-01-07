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
    primitives::SurfaceIntersection,
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
use std::{
    f64::consts::{PI, TAU},
    sync::Arc,
    time::Instant,
};
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

    let start_point_material = scene
        .materials_mut()
        .insert_point_material(PointMaterialSpec::default().color_rgb(rgb(0.0, 0.0, 1.0)));

    let walk_point_material_1 = scene
        .materials_mut()
        .insert_point_material(PointMaterialSpec::default().color_rgb(rgb(0.0, 1.0, 0.0)));

    let walk_point_material_2 = scene
        .materials_mut()
        .insert_point_material(PointMaterialSpec::default().color_rgb(rgb(0.0, 1.0, 1.0)));

    let inverted_point_material = scene
        .materials_mut()
        .insert_point_material(PointMaterialSpec::default().color_rgb(rgb(1.0, 0.0, 0.0)));

    let mut model = PrimitiveModel::new();

    {
        let profile1_start = model.create_point(point3(-1.0, -1.0, 0.0));
        let profile1_end = model.create_point(point3(-1.0, 1.0, 0.0));
        let profile1 = model.create_line_between(profile1_start, profile1_end);
        let path1 = model.create_arc(
            1.0,
            deg(350.0),
            Quat::from_axis_angle(Vec3::UNIT_Y, deg(180.0))
                * Quat::from_axis_angle(Vec3::UNIT_X, deg(90.0)),
            Vec3::ZERO,
        );
        let sweep1 = model.create_sweep(profile1, path1);
        model.set_surface_material(sweep1, sweep1_material);

        let x_offset = 1.5;
        let radius = 1.0;
        let profile2_start = model.create_point(point3(x_offset + radius, 0.0, -1.0));
        let profile2_end = model.create_point(point3(x_offset + radius, 0.0, 1.0));
        let profile2 = model.create_line_between(profile2_start, profile2_end);
        let path2 = model.create_arc(radius, deg(350.0), Quat::ZERO, vec3(x_offset, 0.0, 0.0));
        let sweep2 = model.create_sweep(profile2, path2);
        model.set_surface_material(sweep2, sweep2_material);

        // Intersection
        {
            let sweep1_uv_start = point2(0.55, 1.0 * PI);
            let sweep1_start = model.create_point(
                *model
                    .surface_solver(sweep1)
                    .unwrap()
                    .point(sweep1_uv_start)
                    .pos(),
            );
            model.set_point_material(sweep1_start, start_point_material);

            let sweep2_uv_start = point2(0.45, 1.0 * PI);
            let sweep2_start = model.create_point(
                *model
                    .surface_solver(sweep2)
                    .unwrap()
                    .point(sweep2_uv_start)
                    .pos(),
            );
            model.set_point_material(sweep2_start, start_point_material);

            let s1_solver = model.surface_solver(sweep1).unwrap();
            let s2_solver = model.surface_solver(sweep2).unwrap();
            //let s2_solver = model.surface_solver(sweep1).unwrap();

            let intersection = SurfaceIntersection::new(&s1_solver, &s2_solver);

            const MAX_ITER: usize = 100;
            let mut s1_uv = sweep1_uv_start;
            let mut s2_uv = sweep2_uv_start;
            for i in 0..MAX_ITER {
                (s1_uv, s2_uv) = intersection.next(s1_uv, s2_uv);

                //println!("{}, {}", s1_uv, s2_uv);

                let s1_pos = model.create_point(*s1_solver.point(s1_uv).pos());
                let s2_pos = model.create_point(*s2_solver.point(s2_uv).pos());
                model.set_point_material(s1_pos, walk_point_material_1);
                model.set_point_material(s2_pos, walk_point_material_2);
            }
        }

        // Coordinate system
        {
            let origin = model.create_point(point3(0.0, 0.0, 0.0));
            let x_extent = model.create_point(point3(3.0, 0.0, 0.0));
            let y_extent = model.create_point(point3(0.0, 3.0, 0.0));
            let z_extent = model.create_point(point3(0.0, 0.0, 3.0));
            model.create_line_between(origin, x_extent);
            model.create_line_between(origin, y_extent);
            model.create_line_between(origin, z_extent);
        }
    }

    let sm = SceneModel::from_primitive_model(
        &model,
        &TessellationTolerance::DistanceAndAngle(0.001, deg(3.0)),
    );

    scene.add_model(sm);

    scene
}
