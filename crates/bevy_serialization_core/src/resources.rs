use bevy_derive::{Deref, DerefMut};
use bevy_ecs::component::ComponentId;
use bevy_ecs::system::SystemId;
use bevy_reflect::Reflect;
use bevy_render::camera::{CameraMainTextureUsages, CameraRenderGraph, Exposure};
use bevy_render::prelude::*;
use moonshine_save::GetFilePath;
use moonshine_save::save::{EntityFilter, SaveInput};
use std::any::TypeId;
use std::collections::HashMap;
use std::path::Path;

use bevy_ecs::prelude::*;

/// keeps track of number of times refresh request has been sent. For ui utils.
#[derive(Resource, Default)]
pub struct RefreshCounter {
    pub counter: usize,
}

#[derive(Resource, Default, Deref)]
pub struct WrapAssetSerializers(pub HashMap<ComponentId, SystemId>);
#[derive(Resource, Default, Deref)]
pub struct WrapAssetDeserializers(pub HashMap<ComponentId, SystemId>);

#[derive(Resource, Default, Deref)]
pub struct WrapCompSerializers(pub HashMap<ComponentId, SystemId>);

#[derive(Resource, Default, Deref)]
pub struct WrapCompDeserializers(pub HashMap<ComponentId, SystemId>);

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

/// contains the state of the type registry since the last [`SaveRequest`]/refresh.
#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct TypeRegistryOnSave {
    pub registry: HashMap<TypeId, String>,
}

/// contains the components marked to saved since last save/refresh.
#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct ComponentsOnSave {
    pub components: HashMap<TypeId, String>,
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct ShowSerializable {
    pub check: bool,
}

impl Default for ShowSerializable {
    fn default() -> Self {
        Self { check: false }
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct ShowUnserializable {
    pub check: bool,
}

impl Default for ShowUnserializable {
    fn default() -> Self {
        Self { check: true }
    }
}
