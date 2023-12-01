use bevy::ecs::component::Component;
use bevy_rapier3d::prelude::AdditionalMassProperties;
#[derive(Component, Clone)]
pub struct MassFlag {
    pub mass: f32
}

impl From<MassFlag> for AdditionalMassProperties {
    fn from(value: MassFlag) -> Self {
        AdditionalMassProperties::Mass(value.mass)
    }
}