mod editor;

use editor::{EditorState, EditorUi};
use eframe::{
    egui,
    egui_wgpu::WgpuConfiguration,
    wgpu::{self, Features},
    Renderer,
};
use geometry::IGeometry;
use model::PrimitiveModel;
use render::{
    light::{AmbientLight, DirectionalLight},
    model::SceneModel,
    render::MsaaSamples,
    scene::Scene,
};
use space::{deg, point3, vec3, Point3, Quat, Vec3};
use std::sync::Arc;
use visual::{
    color::rgb,
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

    scene.set_ambient_light(AmbientLight::new(rgb(0.1, 0.1, 0.1)));
    scene.set_camera_directional_lights(vec![
        DirectionalLight::new(vec3(-1.0, -1.0, 2.0), rgb(2.0, 2.0, 2.0)),
        DirectionalLight::new(vec3(1.0, -1.0, 2.0), rgb(1.0, 1.0, 1.5)),
        DirectionalLight::new(vec3(0.0, 1.0, 0.0), rgb(1.5, 1.5, 1.0)),
    ]);

    // Define materials
    let sweep1_material = scene.materials_mut().insert_surface_material(
        SurfaceMaterialSpec::default()
            .roughness_rgb(rgb(0.4, 0.4, 0.4))
            .metallic_rgb(rgb(0.2, 0.2, 0.2)),
    );

    let sweep2_material = scene.materials_mut().insert_surface_material(
        SurfaceMaterialSpec::default()
            .diffuse_rgb(rgb(0.8, 0.3, 0.3))
            .roughness_rgb(rgb(0.2, 0.2, 0.2))
            .metallic_rgb(rgb(0.6, 0.6, 0.6)),
    );

    let default_point_material = scene
        .materials_mut()
        .insert_point_material(PointMaterialSpec::default().color_rgb(rgb(1.0, 0.5, 0.0)));
    scene
        .materials_mut()
        .set_default_point_material(default_point_material);

    const TOLERANCE: f64 = 0.0001;

    let mut model = PrimitiveModel::new();

    {
        let profile_start = model.create_point(Point3::ZERO);
        let profile_end = model.create_point(point3(0.0, 1.0, 0.0));
        let profile = model.create_line_between(profile_start, profile_end);

        let path_start = model.create_point(Point3::ZERO);
        let path_end = model.create_point(point3(0.0, 0.0, 3.0));
        let path = model.create_line_between(path_start, path_end);

        let sweep1 = model.create_sweep(profile, path);
        model.set_surface_material(sweep1, sweep1_material);

        let arc_path = model.create_arc(
            1.0,
            deg(180.0),
            Quat::from_axis_angle(Vec3::UNIT_X, deg(-90.0)),
            vec3(-1.0, 0.0, 0.0),
        );
        let sweep2 = model.create_sweep(profile, arc_path);
        model.set_surface_material(sweep2, sweep2_material);
    }

    let sm = SceneModel::from_primitive_model(&model, TOLERANCE);

    scene.add_model(sm);

    scene
}

/*
fn make_scene_model(
    geometry: &PrimitiveGeometry,
    surface_material: SurfaceMaterialId,
    curve_material: CurveMaterialId,
    point_material: PointMaterialId,
    tolerance: f64,
) -> SceneModel {
    let mut scene_model = SceneModel::new();

    for surface_id in geometry.surfaces().keys() {
        let surface_solver = geometry.surface_solver(*surface_id).unwrap();
        let mut tess = SurfaceTessellator::new(&surface_solver);
        let start = Instant::now();
        tess.tessellate(tolerance);
        let end = Instant::now();
        println!(
            "tessellated surface in {}us with {} vertices",
            (end - start).as_micros(),
            tess.num_points()
        );

        scene_model.add_surface(SceneSurface::new(tess.mesh(), surface_material));
    }

    for curve_id in geometry.curves().keys() {
        let curve_solver = geometry.curve_solver(*curve_id).unwrap();
        let mut tess = CurveTesselator::new(&curve_solver);
        tess.tessellate(tolerance);
        scene_model.add_curve(SceneCurve::new(tess.mesh(), curve_material, 2.0));
    }

    for (_, point) in geometry.points().iter() {
        scene_model.add_point(ScenePoint::new(point.clone(), point_material, 6.0));
    }

    scene_model
}
*/
