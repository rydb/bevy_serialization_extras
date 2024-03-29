use bevy_rapier3d::{geometry::ComputedColliderShape, prelude::AsyncCollider};
use bevy_serialization_core::traits::ManagedTypeRegistration;
use strum_macros::EnumIter;

use bevy_reflect::prelude::*;
use bevy_ecs::prelude::*;
use bevy_reflect::TypeRegistration;
use bevy_reflect::GetTypeRegistration;


#[derive(Component, EnumIter, Reflect, Clone, Default)]
#[reflect(Component)]
pub enum ColliderFlag {
    /// laggy: no-internal geometry(will clip through things)
    Trimesh,
    #[default]
    /// fast: accurate assuming mesh geometry is convex, inaccurate otherwise.
    Convex,
}
impl From<&ColliderFlag> for AsyncCollider {
    fn from(value: &ColliderFlag) -> Self {
        match value {
            ColliderFlag::Trimesh => AsyncCollider::default(),
            ColliderFlag::Convex => AsyncCollider {
                0: ComputedColliderShape::ConvexHull,
            },
        }
    }
}

impl From<&AsyncCollider> for ColliderFlag {
    fn from(value: &AsyncCollider) -> Self {
        match value {
            _ => Self::Trimesh,
        }
    }
}

impl ManagedTypeRegistration for ColliderFlag {
    fn get_all_type_registrations() -> Vec<TypeRegistration> {
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
