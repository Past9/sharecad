use std::{cell::OnceCell, collections::HashMap};

use common::{CurveId, IdSeries, PointId, SurfaceId};

use super::{InstanceId, ModelInstance, SceneCurve, ScenePoint, SceneSurface};

pub struct SceneModel2 {
    surfaces: HashMap<SurfaceId, SceneSurface>,
    curves: HashMap<CurveId, SceneCurve>,
    points: HashMap<PointId, ScenePoint>,

    instances: HashMap<InstanceId, ModelInstance>,
    instance_ids: IdSeries<InstanceId>,
    instance_buffer: OnceCell<wgpu::Buffer>,
}
impl SceneModel2 {
    pub fn empty() -> Self {
        Self {
            surfaces: HashMap::new(),
            curves: HashMap::new(),
            points: HashMap::new(),

            instances: HashMap::new(),
            instance_ids: IdSeries::new(),
            instance_buffer: OnceCell::new(),
        }
    }
}
