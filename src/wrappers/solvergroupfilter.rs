use bevy::prelude::*;
use bevy_rapier3d::prelude::SolverGroups;
use bevy_rapier3d::prelude::Group;

use crate::traits::ManagedTypeRegistration;

#[derive(Component, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct SolverGroupsFlag {
    pub memberships: u32,
    pub filters: u32,
}

impl From<&SolverGroupsFlag> for SolverGroups {
    fn from(value: &SolverGroupsFlag) -> Self {
        Self {
            memberships: Group::from_bits_truncate(value.memberships),
            filters: Group::from_bits_truncate(value.filters),
        }
    }
}

impl From<&SolverGroups> for SolverGroupsFlag {
    fn from(value: &SolverGroups) -> Self {
        Self {
            memberships: value.memberships.bits(),
            filters: value.memberships.bits(),
        }
    }
}

impl ManagedTypeRegistration for SolverGroupsFlag {
    fn get_all_type_registrations() -> Vec<bevy::reflect::TypeRegistration> {
        Vec::new()
    }
}