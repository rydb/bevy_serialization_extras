use std::any::TypeId;

use bevy::prelude::Resource;

#[derive(Resource, Default)]
pub struct SerializeSkipList {
    /// componnets to be skipped during serialization. This should include components with wrapper equivilents. 
    pub skipped_components: Vec<TypeId>
}

#[derive(Resource)]
pub struct SaveFile {
    pub path: String,
}