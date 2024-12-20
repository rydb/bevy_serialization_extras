use bevy_rapier3d::prelude::CollisionGroups;
use bevy_rapier3d::prelude::Group;

use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;

#[derive(Component, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct CollisionGroupsFlag {
    pub memberships: Group,
    pub filters: Group,
}

impl Into<CollisionGroups> for CollisionGroupsFlag {
    fn into(self) -> CollisionGroups {
        CollisionGroups {
            memberships: self.memberships,
            filters: self.filters,
        }
    }
}
