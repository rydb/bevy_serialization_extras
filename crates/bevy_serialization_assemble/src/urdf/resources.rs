use bevy_asset::Handle;
use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;

use super::*;

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct CachedUrdf {
    pub urdf: Handle<Urdf>,
}
