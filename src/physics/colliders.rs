use bevy::prelude::*;
use bevy_rapier3d::prelude::AsyncCollider;
use crate::traits::ECSSerialize;
use bevy::ecs::system::SystemState;
use crate::traits::ECSDeserialize;

#[derive(Component, Reflect, Clone, Default)]
#[reflect(Component)]
pub enum ColliderFlag {
    #[default]
    Async,
}

impl Into<AsyncCollider> for ColliderFlag {
    fn into(self) -> AsyncCollider {
        match self {
            Self::Async => AsyncCollider::default()
        }
    }
}

impl ECSSerialize for ColliderFlag {
    fn serialize(world: &mut World) {
        let mut system_state: SystemState<(
            Query<(Entity, &AsyncCollider)>,
            Commands,
        )> = SystemState::new(world);
        let (
            physics_to_serialize,
            mut commands,
        ) = system_state.get_mut(world);
    
        for (e, collider) in physics_to_serialize.iter() {
            commands.entity(e).insert(
                ColliderFlag::Async
            );
        }
        system_state.apply(world);

    }
}

impl ECSDeserialize for ColliderFlag {
    fn deserialize(world: &mut World) {
        let mut system_state: SystemState<(
            Query<(Entity, &ColliderFlag), Without<AsyncCollider>>,
            Commands,
        )> = SystemState::new(world);

        let (
            models_without_physics,
            mut commands,
        ) = system_state.get_mut(world);
    
        for (e, physics_flag) in models_without_physics.iter() {
            commands.entity(e).insert(
                AsyncCollider::default()
            );
        system_state.apply(world);
        }
    }
}