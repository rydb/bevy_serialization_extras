use bevy_rapier3d::prelude::AdditionalMassProperties;
#[derive(Clone)]
pub struct MassFlag {
    mass: f32
}

impl From<MassFlag> for AdditionalMassProperties {
    fn from(value: MassFlag) -> Self {
        AdditionalMassProperties::Mass(value.mass)
    }
}