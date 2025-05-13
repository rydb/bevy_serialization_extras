use bevy_rapier3d::prelude::AdditionalMassProperties;

use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;
use bevy_synonomize::traits::ComponentWrapper;

#[derive(Reflect, PartialEq, Component, Clone)]
pub struct MassFlag {
    pub mass: f32,
}

impl ComponentWrapper for MassFlag {
    type WrapperTarget = AdditionalMassProperties;
}

// W.I.P
impl Default for MassFlag {
    fn default() -> Self {
        Self { mass: 1.0 }
    }
}

impl From<&MassFlag> for AdditionalMassProperties {
    fn from(value: &MassFlag) -> Self {
        AdditionalMassProperties::Mass(value.mass)
    }
}

impl From<&AdditionalMassProperties> for MassFlag {
    fn from(value: &AdditionalMassProperties) -> Self {
        match value {
            AdditionalMassProperties::Mass(g) => Self { mass: *g },
            AdditionalMassProperties::MassProperties(mass_properties) => Self {
                mass: mass_properties.mass,
            },
        }
    }
}
