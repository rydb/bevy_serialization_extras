use bevy_derive::{Deref, DerefMut};
use bevy_ecs::component::ComponentId;
use bevy_ecs::system::SystemId;
use bevy_reflect::Reflect;
use bevy_render::camera::{CameraMainTextureUsages, CameraRenderGraph, Exposure};
use bevy_render::prelude::*;
// use moonshine_save::GetFilePath;
// use moonshine_save::save::{EntityFilter, SaveInput};
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
