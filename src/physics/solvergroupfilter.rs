use bevy::prelude::*;
use bevy_rapier3d::prelude::AsyncCollider;
use crate::traits::ECSSerialize;
use bevy::ecs::system::SystemState;
use crate::traits::ECSDeserialize;
use bevy_rapier3d::prelude::CollisionGroups;
use bevy_rapier3d::prelude::SolverGroups;
use bevy_rapier3d::prelude::Group;

#[derive(Component, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct SolverGroupsFlag {
    pub memberships: u32,
    pub filters: u32,
}

impl From<SolverGroupsFlag> for SolverGroups {
    fn from(value: SolverGroupsFlag) -> Self {
        Self {
            memberships: Group::from_bits_truncate(value.memberships),
            filters: Group::from_bits_truncate(value.filters),
        }
    }
}

impl From<SolverGroups> for SolverGroupsFlag {
    fn from(value: SolverGroups) -> Self {
        Self {
            memberships: value.memberships.bits(),
            filters: value.memberships.bits(),
        }
    }
}

impl ECSSerialize for SolverGroupsFlag {
    fn serialize_for<T>(world: &mut World)
    where
        T: Component,
        Self: From<T>    
    {
        let mut system_state: SystemState<(
            Query<(Entity, &T)>,
            Commands,
        )> = SystemState::new(world);
        let (
            physics_to_serialize,
            mut commands,
        ) = system_state.get_mut(world);
    
        for (e, f) in physics_to_serialize.iter() {
            commands.entity(e).insert(

                Self::from(*f)
            );
        }
        system_state.apply(world);

    }
}

impl ECSDeserialize for SolverGroupsFlag {
    fn deserialize(world: &mut World) {
        let mut system_state: SystemState<(
            Query<(Entity, &Self), Without<AsyncCollider>>,
            Commands,
        )> = SystemState::new(world);

        let (
            models_without_physics,
            mut commands,
        ) = system_state.get_mut(world);
    
        for (e, f) in models_without_physics.iter() {
            commands.entity(e).insert(
                SolverGroups::from(f)
            );
        system_state.apply(world);
        }
    }
}