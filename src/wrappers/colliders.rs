use bevy::prelude::*;
use bevy_rapier3d::prelude::AsyncCollider;
use crate::traits::ManagedTypeRegistration;
use bevy::reflect::GetTypeRegistration;
use strum_macros::EnumIter;
// use strum::IntoEnumIterator;
// use crate::physics::mesh::MeshPrimitive;

#[derive(Component, EnumIter, Reflect, Clone, Default)]
#[reflect(Component)]
pub enum ColliderFlag {
    #[default]
    Async,
}

impl From<&ColliderFlag> for AsyncCollider {
    fn from(value: &ColliderFlag) -> Self {
        match value {
            ColliderFlag::Async => AsyncCollider::default()
        }
    }
}



impl From<&AsyncCollider> for ColliderFlag {
    fn from(value: &AsyncCollider) -> Self {
        match value {
            _ => Self::Async
        } 
    }
}

impl ManagedTypeRegistration for ColliderFlag {
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