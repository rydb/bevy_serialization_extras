use bevy::prelude::Resource;
use moonshine_save::save::SaveFilter;

#[derive(Resource, Default)]
pub struct SerializeFilter {
    pub filter: SaveFilter
}

#[derive(Resource)]
pub struct SaveFile {
    pub path: String,
}