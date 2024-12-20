use bevy_rapier3d::dynamics::Ccd;

use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;

#[derive(Reflect, Component, Clone)]
#[reflect(Component)]
pub struct CcdFlag {
    pub enabled: bool,
}

impl Default for CcdFlag {
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl From<CcdFlag> for Ccd {
    fn from(value: CcdFlag) -> Self {
        Self {
            enabled: value.enabled,
        }
    }
}

impl From<Ccd> for CcdFlag {
    fn from(value: Ccd) -> Self {
        Self {
            enabled: value.enabled,
        }
    }
}
