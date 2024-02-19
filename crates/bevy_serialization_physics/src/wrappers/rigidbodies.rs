use bevy::reflect::GetTypeRegistration;
use bevy::{prelude::Component, reflect::Reflect};
use bevy_rapier3d::prelude::RigidBody;
use bevy_serialization_core::traits::ManagedTypeRegistration;
use strum_macros::EnumIter;

#[derive(Component, Reflect, Clone, Default, EnumIter)]
pub enum RigidBodyFlag {
    #[default]
    Fixed,
    Dynamic,
}

impl From<&RigidBodyFlag> for RigidBody {
    fn from(value: &RigidBodyFlag) -> Self {
        match value {
            RigidBodyFlag::Fixed => Self::Fixed,
            RigidBodyFlag::Dynamic => Self::Dynamic,
        }
    }
}
impl From<&RigidBody> for RigidBodyFlag {
    fn from(value: &RigidBody) -> Self {
        match *value {
            RigidBody::Fixed => Self::Fixed,
            RigidBody::Dynamic => Self::Dynamic,
            _ => panic!("Rigidbody serialization only implemented for fixed and dynamic. populate wrapper for more types")
        }
    }
}

impl ManagedTypeRegistration for RigidBodyFlag {
    fn get_all_type_registrations() -> Vec<bevy::reflect::TypeRegistration> {
        let mut type_registry = Vec::new();

        type_registry.push(Self::get_type_registration());

        // for enum_variant in Self::iter() {
        //     match enum_variant {
        //         Self::Async(..) => type_registry.push(ColliderFlag::get_type_registration()),
        //     }
        // }
        return type_registry;
    }
}
