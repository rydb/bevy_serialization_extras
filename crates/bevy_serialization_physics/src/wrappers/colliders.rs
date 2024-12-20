use bevy_rapier3d::{geometry::ComputedColliderShape, prelude::AsyncCollider};
use strum_macros::EnumIter;

use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;

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
