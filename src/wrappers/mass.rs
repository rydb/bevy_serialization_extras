use bevy_rapier3d::prelude::AdditionalMassProperties;
pub struct MassFlag {
    mass: f32
}

impl From<MassFlag> for AdditionalMassProperties {
    fn from(value: MassFlag) -> Self {
        AdditionalMassProperties::Mass(value.mass)
    }
}