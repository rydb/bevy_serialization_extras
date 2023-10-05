use bevy::prelude::*;
use bevy_rapier3d::prelude::AsyncCollider;
use bevy::ecs::system::SystemState;

#[derive(Component, Reflect, Clone, Default)]
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
        return Self::Async;
    }
}
