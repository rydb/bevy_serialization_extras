use bevy::prelude::*;
use bevy_rapier3d::prelude::SolverGroups;
use bevy_rapier3d::prelude::Group;

use crate::traits::ManagedTypeRegistration;

pub const PHYSICS_FIXED: SolverGroupsFlag = SolverGroupsFlag {
    memberships: Group::ALL,
    filters: Group::ALL,
};

#[derive(Component, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct SolverGroupsFlag {
    pub memberships: Group,
    pub filters: Group,
}

impl From<&SolverGroupsFlag> for SolverGroups {
    fn from(value: &SolverGroupsFlag) -> Self {
        Self {
            memberships: value.memberships,
            filters: value.filters,
        }
    }
}

impl From<&SolverGroups> for SolverGroupsFlag {
    fn from(value: &SolverGroups) -> Self {
        Self {
            memberships: value.memberships,
            filters: value.memberships,
        }
    }
}

impl ManagedTypeRegistration for SolverGroupsFlag {
    fn get_all_type_registrations() -> Vec<bevy::reflect::TypeRegistration> {
        Vec::new()
    }
}