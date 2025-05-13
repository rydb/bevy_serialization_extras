//! library for syncing synonomous components with eachother.

use std::ops::Deref;

use bevy_ecs::{
    component::ComponentId,
    resource::Resource,
    system::{Commands, Res, SystemId},
};

// pub mod plugins;
pub mod resources;
mod systems;
pub mod traits;
pub mod plugins;
pub mod synonyms;

pub mod prelude {
    pub use crate::{resources::*, synonyms::*, traits::*};
}

#[doc = "hidden"]
pub fn run_proxy_system<T>(proxy_systems: Res<T>, mut commands: Commands)
where
    T: Resource + Deref<Target = std::collections::HashMap<ComponentId, SystemId>>,
{
    for (_, system) in (*proxy_systems).iter() {
        commands.run_system(*system);
    }
}
