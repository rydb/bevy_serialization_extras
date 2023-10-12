use bevy::prelude::*;
use bevy_rapier3d::prelude::CollisionGroups;
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
