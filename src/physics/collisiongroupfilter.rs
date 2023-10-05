use bevy::prelude::*;
use bevy_rapier3d::prelude::AsyncCollider;
use bevy::ecs::system::SystemState;
use bevy_rapier3d::prelude::CollisionGroups;
use bevy_rapier3d::prelude::SolverGroups;
use bevy_rapier3d::prelude::Group;

#[derive(Component, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct CollisionGroupsFlag {
    pub memberships: u32,
    pub filters: u32,
}


impl Into<CollisionGroups> for CollisionGroupsFlag {
    fn into(self) -> CollisionGroups {
        CollisionGroups {
            memberships: Group::from_bits_truncate(self.memberships),
            filters: Group::from_bits_truncate(self.filters)
        }
    }
}


// impl ECSSerialize for CollisionGroupsFlag {
//     fn serialize(world: &mut World) {
//         let mut system_state: SystemState<(
//             Query<(Entity, &AsyncCollider)>,
//             Commands,
//         )> = SystemState::new(world);
//         let (
//             physics_to_serialize,
//             mut commands,
//         ) = system_state.get_mut(world);
    
//         for (e, collider) in physics_to_serialize.iter() {
//             commands.entity(e).insert(
//                 ColliderFlag::Async
//             );
//         }
//         system_state.apply(world);

//     }
// }

// impl ECSDeserialize for CollisionGroupsFlag {
//     fn deserialize(world: &mut World) {
//         let mut system_state: SystemState<(
//             Query<(Entity, &CollisionGroupsFlag), Without<AsyncCollider>>,
//             Commands,
//         )> = SystemState::new(world);

//         let (
//             models_without_physics,
//             mut commands,
//         ) = system_state.get_mut(world);
    
//         for (e, physics_flag) in models_without_physics.iter() {
//             commands.entity(e).insert(
//                 AsyncCollider::default()
//             );
//         system_state.apply(world);
//         }
//     }
// }