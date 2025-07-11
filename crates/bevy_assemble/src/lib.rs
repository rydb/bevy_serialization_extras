use std::any::TypeId;
use std::collections::{HashMap, HashSet};

use bevy_derive::{Deref, DerefMut};
use bevy_ecs::prelude::ReflectComponent;
use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;
use bevy_synonymize_physics::prelude::JointInfo;

pub mod components;
pub mod gltf;
pub mod plugins;
pub mod resources;
pub(crate) mod systems;
pub mod traits;
pub mod urdf;

pub mod prelude {
    pub use super::{plugins::*, resources::*, urdf::*};
}

/// Id of an assembled structure
#[derive(Component, Reflect, PartialEq, Deref, DerefMut, Clone)]
#[reflect(Component)]
pub struct AssemblyId(pub i64);

#[derive(Resource, Default, Debug)]
pub struct Assemblies(pub HashMap<i64, HashSet<Entity>>);

#[derive(Event)]
pub struct SaveSuccess {
    pub file_name: String,
    pub asset_type_id: TypeId,
}

/// current stage of request for joint from increasing context.
#[derive(Debug, Reflect, Clone)]
pub enum JointRequestStage {
    Name(String),
    Entity(Entity),
}

/// Request for a joint. Split into stages depending on available info on joint at time of initialization. Eventually elevated to [`JointFlag`]
#[derive(Component, Debug, Reflect, Clone)]
pub struct JointRequest {
    pub stage: JointRequestStage,
    pub joint: JointInfo,
}
