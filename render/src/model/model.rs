use super::{CurveId, PointId, SceneCurve, ScenePoint, SceneSurface};
use super::{CurveMesh, SurfaceMesh};
use bytemuck::{Pod, Zeroable};
use common::IdSeries;
use common::SurfaceId;
use geometry::math::{deg, rad, Mat33, Mat44, Quat, Vec3};
use geometry::tessellate::{TessellatedCurve, TessellatedSurface, TessellationTolerance};
use geometry::IGeometry;
use model::PrimitiveModel;
use std::{cell::OnceCell, collections::HashMap};
use visual::IGeometryVisuals;
use wgpu::util::DeviceExt;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ModelId(pub u32);
impl From<u32> for ModelId {
    fn from(id: u32) -> Self {
        ModelId(id)
    }
}
impl From<ModelId> for u32 {
    fn from(id: ModelId) -> Self {
        id.0
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct InstanceId(pub u32);
impl From<u32> for InstanceId {
    fn from(id: u32) -> Self {
        InstanceId(id)
    }
}
impl From<InstanceId> for u32 {
    fn from(id: InstanceId) -> Self {
        id.0
    }
}

#[derive(Debug)]
pub struct SceneModel {
    surfaces: HashMap<SurfaceId, SceneSurface>,
    curves: HashMap<CurveId, SceneCurve>,
    points: HashMap<PointId, ScenePoint>,

    surface_ids: IdSeries<SurfaceId>,
    curve_ids: IdSeries<CurveId>,
    point_ids: IdSeries<PointId>,

    instances: HashMap<InstanceId, ModelInstance>,
    instance_ids: IdSeries<InstanceId>,
    instance_buffer: OnceCell<wgpu::Buffer>,
}
impl SceneModel {
    pub fn new() -> Self {
        let mut model = Self {
            surfaces: HashMap::new(),
            curves: HashMap::new(),
            points: HashMap::new(),

            surface_ids: IdSeries::new(),
            curve_ids: IdSeries::new(),
            point_ids: IdSeries::new(),

            instances: HashMap::new(),
            instance_ids: IdSeries::new(),
            instance_buffer: OnceCell::new(),
        };

        model.add_instance(ModelInstance::default());

        model
    }

    pub fn from_primitive_model(model: &PrimitiveModel, tolerance: &TessellationTolerance) -> Self {
        let mut scene_model = Self::new();

        for surface_id in model.surfaces().keys() {
            let surface_solver = model.surface_solver(*surface_id).unwrap();
            let tessellated = TessellatedSurface::create(&surface_solver, tolerance);
            scene_model.add_surface(SceneSurface::new(
                SurfaceMesh::from_tessellated(&tessellated),
                model.get_surface_material(*surface_id),
            ));
        }

        for curve_id in model.curves().keys() {
            let curve_solver = model.curve_solver(*curve_id).unwrap();
            let tessellated = TessellatedCurve::create(&curve_solver, tolerance);
            scene_model.add_curve(SceneCurve::new(
                CurveMesh::from_tessellated(&tessellated),
                model.get_curve_material(*curve_id),
                2.0,
            ));
        }

        for (point_id, point) in model.points() {
            scene_model.add_point(ScenePoint::new(
                *point,
                model.get_point_material(*point_id),
                6.0,
            ));
        }

        scene_model
    }

    pub fn add_instance(&mut self, instance: ModelInstance) -> InstanceId {
        let id = self.instance_ids.next();
        self.instances.insert(id, instance);
        id
    }

    pub fn add_surface(&mut self, surface: SceneSurface) -> SurfaceId {
        let id = self.surface_ids.next();
        self.surfaces.insert(id, surface);
        id
    }

    pub fn add_curve(&mut self, curve: SceneCurve) -> CurveId {
        let id = self.curve_ids.next();
        self.curves.insert(id, curve);
        id
    }

    pub fn add_point(&mut self, point: ScenePoint) -> PointId {
        let id = self.point_ids.next();
        self.points.insert(id, point);
        id
    }

    pub fn surfaces(&self) -> &HashMap<SurfaceId, SceneSurface> {
        &self.surfaces
    }

    pub fn curves(&self) -> &HashMap<CurveId, SceneCurve> {
        &self.curves
    }

    pub fn points(&self) -> &HashMap<PointId, ScenePoint> {
        &self.points
    }

    pub fn instances(&self) -> &HashMap<InstanceId, ModelInstance> {
        &self.instances
    }
    pub fn instance_buffer(&self, device: &wgpu::Device) -> &wgpu::Buffer {
        self.instance_buffer.get_or_init(|| {
            let instance_data = self
                .instances
                .iter()
                .map(|(id, inst)| inst.to_raw(id))
                .collect::<Vec<_>>();

            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX,
            })
        })
    }

    pub fn num_instances(&self) -> u32 {
        self.instances.len() as u32
    }
}

#[derive(Debug, Clone)]
pub struct ModelInstance {
    pub orientation: Quat,
    pub translation: Vec3,
}
impl ModelInstance {
    fn to_raw(&self, id: &InstanceId) -> ModelInstanceRaw {
        let model = Mat44::translation(self.translation) * Mat44::from(self.orientation);
        ModelInstanceRaw {
            id: id.0,
            model: model.transpose().into(),
            normal: Mat33::from(self.orientation).transpose().into(),
        }
    }
}
impl Default for ModelInstance {
    fn default() -> Self {
        Self {
            orientation: Quat::from_axis_angle(Vec3::UNIT_Y, rad(0.0)),
            translation: Vec3::ZERO,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct ModelInstanceRaw {
    pub id: u32,
    pub model: [[f32; 4]; 4],
    pub normal: [[f32; 3]; 3],
}
impl ModelInstanceRaw {
    const SURFACE_ATTRIBS: [wgpu::VertexAttribute; 8] = wgpu::vertex_attr_array![
        7 => Uint32,

        8 => Float32x4,
        9 => Float32x4,
        10 => Float32x4,
        11 => Float32x4,

        12 => Float32x3,
        13 => Float32x3,
        14 => Float32x3,
    ];

    const CURVE_ATTRIBS: [wgpu::VertexAttribute; 8] = wgpu::vertex_attr_array![
        4 => Uint32,

        5 => Float32x4,
        6 => Float32x4,
        7 => Float32x4,
        8 => Float32x4,

        9 => Float32x3,
        10 => Float32x3,
        11 => Float32x3,
    ];

    const POINT_ATTRIBS: [wgpu::VertexAttribute; 8] = wgpu::vertex_attr_array![
        3 => Uint32,

        4 => Float32x4,
        5 => Float32x4,
        6 => Float32x4,
        7 => Float32x4,

        8 => Float32x3,
        9 => Float32x3,
        10 => Float32x3,
    ];

    pub fn surface_desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<ModelInstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::SURFACE_ATTRIBS,
        }
    }

    pub fn curve_desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<ModelInstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::CURVE_ATTRIBS,
        }
    }

    pub fn point_desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<ModelInstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::POINT_ATTRIBS,
        }
    }
}
