use common::IdSeries;
use visual::material::MaterialLibrary;

use crate::{
    light::{AmbientLight, DirectionalLight},
    model::{ModelId, SceneModel},
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Scene {
    models: HashMap<ModelId, SceneModel>,
    model_ids: IdSeries<ModelId>,
    materials: MaterialLibrary,
    world_directional_lights: Vec<DirectionalLight>,
    camera_directional_lights: Vec<DirectionalLight>,
    ambient_lights: Vec<AmbientLight>,
}
unsafe impl Send for Scene {}
impl Scene {
    pub fn new() -> Self {
        Self {
            models: HashMap::new(),
            model_ids: IdSeries::new(),
            materials: MaterialLibrary::new(),
            world_directional_lights: vec![],
            camera_directional_lights: vec![],
            ambient_lights: vec![],
        }
    }

    pub fn models(&self) -> &HashMap<ModelId, SceneModel> {
        &self.models
    }

    pub fn add_model(&mut self, model: SceneModel) -> ModelId {
        let id = self.model_ids.next();
        self.models.insert(id, model);
        id
    }

    pub fn ambient_lights(&self) -> &[AmbientLight] {
        &self.ambient_lights
    }

    pub fn materials(&self) -> &MaterialLibrary {
        &self.materials
    }

    pub fn materials_mut(&mut self) -> &mut MaterialLibrary {
        &mut self.materials
    }

    pub fn world_directional_lights(&self) -> &[DirectionalLight] {
        &self.world_directional_lights
    }

    pub fn set_world_directional_lights(&mut self, lights: Vec<DirectionalLight>) {
        self.world_directional_lights = lights;
    }

    pub fn camera_directional_lights(&self) -> &[DirectionalLight] {
        &self.camera_directional_lights
    }

    pub fn set_camera_directional_lights(&mut self, lights: Vec<DirectionalLight>) {
        self.camera_directional_lights = lights;
    }

    pub fn set_ambient_light(&mut self, light: AmbientLight) {
        self.ambient_lights.push(light);
    }
}
