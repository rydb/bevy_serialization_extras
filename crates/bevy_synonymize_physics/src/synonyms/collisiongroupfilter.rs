use bevy_rapier3d::prelude::CollisionGroups;
use bevy_rapier3d::prelude::Group;

use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;
use bevy_synonymize::traits::ComponentSynonym;

#[derive(Component, PartialEq, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct CollisionGroupsFlag {
    pub memberships: Group,
    pub filters: Group,
}

impl ComponentSynonym for CollisionGroupsFlag {
    type SynonymTarget = CollisionGroups;
}

impl From<&CollisionGroups> for CollisionGroupsFlag {
    fn from(value: &CollisionGroups) -> Self {
        Self {
            memberships: value.memberships,
            filters: value.filters,
        }
    }
}

impl From<&CollisionGroupsFlag> for CollisionGroups {
    fn from(value: &CollisionGroupsFlag) -> Self {
        Self {
            memberships: value.memberships,
            filters: value.filters,
        }
    }
}
