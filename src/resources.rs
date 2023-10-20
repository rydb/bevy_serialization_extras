use std::any::TypeId;
use bevy::prelude::Resource;
use moonshine_save::{save::SaveFilter, prelude::LoadFromFileRequest};
use std::collections::HashMap;
use moonshine_save::prelude::SaveIntoFileRequest;
use std::path::Path;
#[derive(Resource, Default)]
pub struct SerializeFilter {
    pub filter: SaveFilter
}

#[derive(Resource)]
pub struct SaveRequest {
    pub path: String,
}

impl SaveIntoFileRequest for SaveRequest {
    fn path(&self) -> &Path {
        self.path.as_ref()
    }
}

#[derive(Resource)]
pub struct LoadRequest {
    pub path: String,
}

impl LoadFromFileRequest for LoadRequest {
    fn path(&self) -> &Path {
        self.path.as_ref()
    }
}

#[derive(Resource, Default)]
pub struct TypeRegistryOnSave {
    pub registry: HashMap<TypeId, String>,
}

#[derive(Resource, Default)]
pub struct ComponentsOnSave {
    pub components: HashMap<TypeId, String>
}

#[derive(Resource)]
pub struct ShowSerializable {
    pub check: bool
}

impl Default for ShowSerializable {
    fn default() -> Self {
        Self {
            check: false
        }
    }
}

#[derive(Resource)]
pub struct ShowUnserializable {
    pub check: bool
}

impl Default for ShowUnserializable {
    fn default() -> Self {
        Self { check: true }
    }
}