use bevy::{ecs::component::Component, reflect::Reflect};
use bevy_rapier3d::prelude::AdditionalMassProperties;

use bevy::reflect::GetTypeRegistration;
use bevy_serialization_core::traits::ManagedTypeRegistration;

#[derive(Reflect, Component, Clone)]
pub struct MassFlag {
    pub mass: f32
}
// W.I.P
impl Default for MassFlag {
    fn default() -> Self {
        Self {
            mass: 1.0
        }
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
            AdditionalMassProperties::Mass(g) => Self { mass: *g},
            AdditionalMassProperties::MassProperties(mass_properties) => Self {mass: mass_properties.mass}
        }
    }
}

impl ManagedTypeRegistration for MassFlag {
    fn get_all_type_registrations() -> Vec<bevy::reflect::TypeRegistration> {
        let mut type_registry = Vec::new();

        type_registry.push(Self::get_type_registration());
        
        // for enum_variant in Self::iter() {
        //     match enum_variant {
        //         Self::Async(..) => type_registry.push(ColliderFlag::get_type_registration()),
        //     }
        // }
        return type_registry
        
    }
}