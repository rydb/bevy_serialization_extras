use std::path::Path;

use bevy_derive::{Deref, DerefMut};
use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;
use bevy_render::{camera::{CameraMainTextureUsages, CameraRenderGraph, Exposure}, mesh::Mesh3d};
use moonshine_save::{prelude::GetFilePath, save::{EntityFilter, SaveInput}};




/// Resource version of moonshine-save's [`SaveFilter`].
#[derive(Resource, Clone, DerefMut, Deref)]
pub struct SerializeFilter(pub SaveInput);
impl Default for SerializeFilter {
    fn default() -> Self {
        Self({
            // Due to bevy_scene taking `self` and not `&mut self`, need to initialize this twice.
            let mut new_filter = SaveInput::default();

            new_filter.entities = EntityFilter::Any;
            new_filter.components = new_filter
                .components
                .clone()
                .deny::<CameraMainTextureUsages>()
                .deny::<CameraRenderGraph>()
                .deny::<Exposure>()
                .deny::<Mesh3d>();
            new_filter
        })
    }
}


#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct SaveRequest {
    pub path: String,
}

impl GetFilePath for SaveRequest {
    fn path(&self) -> &Path {
        self.path.as_ref()
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct LoadRequest {
    pub path: String,
}

impl GetFilePath for LoadRequest {
    fn path(&self) -> &Path {
        self.path.as_ref()
    }
}

