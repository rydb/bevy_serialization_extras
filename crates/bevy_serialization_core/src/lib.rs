//! The backbone of bevy_serialziation_extras

use std::ops::Deref;

use bevy_ecs::{component::ComponentId, system::{Commands, Res, Resource, SystemId}};

pub mod plugins;
pub mod resources;
mod systems;
pub mod traits;
pub mod wrappers;

pub mod prelude {
    pub use crate::{plugins::*, resources::*, traits::*, wrappers::*};
}

#[doc = "hidden"]
pub fn run_proxy_system<T>(
    proxy_systems: Res<T>,
    mut commands: Commands
)
    where
        T: Resource + Deref<Target = bevy_utils::HashMap<ComponentId, SystemId>>,
{
    for (_, system) in (*proxy_systems).iter() {
        commands.run_system(*system);
    }
}