use bevy_rapier3d::dynamics::Ccd;

use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;
use bevy_synonymize::traits::ComponentSynonym;

#[derive(Reflect, PartialEq, Component, Clone)]
#[reflect(Component)]
pub struct CcdFlag {
    pub enabled: bool,
}

impl ComponentSynonym for CcdFlag {
    type SynonymTarget = Ccd;
}

impl Default for CcdFlag {
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl From<&CcdFlag> for Ccd {
    fn from(value: &CcdFlag) -> Self {
        Self {
            enabled: value.enabled,
        }
    }
}

impl From<&Ccd> for CcdFlag {
    fn from(value: &Ccd) -> Self {
        Self {
            enabled: value.enabled,
        }
    }
}
