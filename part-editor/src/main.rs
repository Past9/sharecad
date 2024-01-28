mod editor;

use editor::{EditorState, EditorUi};
use eframe::{
    egui,
    egui_wgpu::WgpuConfiguration,
    wgpu::{self, Features},
    Renderer,
};
use geometry::{
    math::{deg, vec3, vec4, Quat, Scalar, Vec3},
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
use std::{f64::consts::PI, sync::Arc, time::Instant};
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
    let start = Instant::now();
    let mut scene = Scene::new();

    scene.set_ambient_light(AmbientLight::new(rgb(0.35, 0.35, 0.35)));
    scene.set_camera_directional_lights(vec![
        DirectionalLight::new(vec3(-1.0, -1.0, 2.0), rgb(2.0, 2.0, 2.0)),
        DirectionalLight::new(vec3(1.0, -1.0, 2.0), rgb(1.0, 1.0, 1.5)),
        DirectionalLight::new(vec3(0.0, 1.0, 0.0), rgb(1.5, 1.5, 1.0)),
    ]);

    /*
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
            //.transmit_rgb(rgb(0.5, 0.5, 0.5))
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
    */

    let mut model = PrimitiveModel::new();

    {
        let c1 = model.create_arc(
            1.0,
            deg(360.0),
            Quat::from_axis_angle(Vec3::UNIT_Y, deg(180.0))
                * Quat::from_axis_angle(Vec3::UNIT_X, deg(90.0)),
            Vec3::ZERO,
        );
        let c2 = model.create_arc(
            1.0,
            deg(360.0),
            Quat::ZERO,
            vec3(0.0, f64::SQRT_2 / 2.0, f64::SQRT_2 / 2.0),
        );

        /*
        let p0 = model.create_point3(vec3(0.0, -1.0, 0.0));
        let p1 = model.create_point3(vec3(0.0, 1.0, 0.0));
        let p2 = model.create_point3(vec3(-1.0, 0.0, 0.0));
        let p3 = model.create_point3(vec3(1.0, 0.0, 0.0));

        let c1 = model.create_line_between(p0, p1);
        let c2 = model.create_line_between(p2, p3);
         */

        let c1_solver = model.curve_solver(c1).unwrap();
        let c2_solver = model.curve_solver(c2).unwrap();

        c1_solver.intersect_curve(&c2_solver);

        model.create_point3(*c1_solver.point(5.497787143647968).pos());
        model.create_point3(*c2_solver.point(3.926990816713388).pos());

        //model.create_point3(*arc1_solver.point(3.92699).pos());
        //model.create_point3(*arc2_solver.point(5.49778).pos());

        /*
        let sweep1 = model.create_sweep(profile1, path1);
        model.set_surface_material(sweep1, sweep1_material);

        let x_offset = 1.5;
        let radius = 1.0;
        let profile2_start = model.create_point3(point3(x_offset + radius, 0.0, -1.0));
        let profile2_end = model.create_point3(point3(x_offset + radius, 0.0, 1.0));
        let profile2 = model.create_line_between(profile2_start, profile2_end);
        let path2 = model.create_arc(radius, deg(350.0), Quat::ZERO, vec3(x_offset, 0.0, 0.0));
        let sweep2 = model.create_sweep(profile2, path2);
        model.set_surface_material(sweep2, sweep2_material);

        // Transversal intersection
        {
            let sweep1_uv_start = point2(0.5 + 0.25 * 3f64.sqrt(), PI);
            let sweep1_start = model.create_point3(
                *model
                    .surface_solver(sweep1)
                    .unwrap()
                    .point(sweep1_uv_start)
                    .pos(),
            );
            model.set_point_material(sweep1_start, start_point_material);

            let sweep2_uv_start = point2(0.5, deg(120.0).radians());
            let sweep2_start = model.create_point3(
                *model
                    .surface_solver(sweep2)
                    .unwrap()
                    .point(sweep2_uv_start)
                    .pos(),
            );
            model.set_point_material(sweep2_start, start_point_material);

            let s1_solver = model.surface_solver(sweep1).unwrap();
            let s2_solver = model.surface_solver(sweep2).unwrap();

            let step: f64 = 0.001;
            let s1_uv = sweep1_uv_start;
            let s2_uv = sweep2_uv_start;

            let mut intersection = SSCurveSampler::new(&s1_solver, &s2_solver, s1_uv, s2_uv);

            intersection.fill(step);

            let points = intersection.take_points();

            let mut len = 0.0;
            for i in 1..points.len() {
                let p0 = &points[i - 1];
                let p1 = &points[i];

                len += (p1.pos - p0.pos).magnitude();
            }

            let ss_curve_id = model.create_ss_curve(sweep1, sweep2, points);

            let ss_curve_solver = model.curve_solver(ss_curve_id).unwrap();
        }

        // Coordinate system
        {
            let origin = model.create_point3(point3(0.0, 0.0, 0.0));
            let x_extent = model.create_point3(point3(3.0, 0.0, 0.0));
            let y_extent = model.create_point3(point3(0.0, 3.0, 0.0));
            let z_extent = model.create_point3(point3(0.0, 0.0, 3.0));
            model.create_line_between(origin, x_extent);
            model.create_line_between(origin, y_extent);
            model.create_line_between(origin, z_extent);
        }
         */
    }

    let sm = SceneModel::from_primitive_model(
        &model,
        &TessellationTolerance::DistanceAndAngle(0.0005, deg(3.0)),
    );

    scene.add_model(sm);

    //println!("scene = {:#?}", scene);

    let end = Instant::now();
    println!("Model built in {}us", (end - start).as_micros());

    scene
}
