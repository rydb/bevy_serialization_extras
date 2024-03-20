use bevy_rapier3d::prelude::Group;
use bevy_rapier3d::prelude::SolverGroups;
use bevy_serialization_core::traits::ManagedTypeRegistration;

use bevy_reflect::prelude::*;
use bevy_ecs::prelude::*;
use bevy_reflect::TypeRegistration;

pub const PHYSICS_FIXED: SolverGroupsFlag = SolverGroupsFlag {
    memberships: GroupWrapper::ALL,
    filters: GroupWrapper::ALL,
};

/// wrapper around rapier groups to prevent libraries that use this from needing to import the entirety of rapier.
#[derive(Reflect, Clone, Copy)]
pub struct GroupWrapper(pub u32);

bitflags::bitflags! {
    impl GroupWrapper: u32 {
        /// The group n°1.
        const GROUP_1 = 1 << 0;
        /// The group n°2.
        const GROUP_2 = 1 << 1;
        /// The group n°3.
        const GROUP_3 = 1 << 2;
        /// The group n°4.
        const GROUP_4 = 1 << 3;
        /// The group n°5.
        const GROUP_5 = 1 << 4;
        /// The group n°6.
        const GROUP_6 = 1 << 5;
        /// The group n°7.
        const GROUP_7 = 1 << 6;
        /// The group n°8.
        const GROUP_8 = 1 << 7;
        /// The group n°9.
        const GROUP_9 = 1 << 8;
        /// The group n°10.
        const GROUP_10 = 1 << 9;
        /// The group n°11.
        const GROUP_11 = 1 << 10;
        /// The group n°12.
        const GROUP_12 = 1 << 11;
        /// The group n°13.
        const GROUP_13 = 1 << 12;
        /// The group n°14.
        const GROUP_14 = 1 << 13;
        /// The group n°15.
        const GROUP_15 = 1 << 14;
        /// The group n°16.
        const GROUP_16 = 1 << 15;
        /// The group n°17.
        const GROUP_17 = 1 << 16;
        /// The group n°18.
        const GROUP_18 = 1 << 17;
        /// The group n°19.
        const GROUP_19 = 1 << 18;
        /// The group n°20.
        const GROUP_20 = 1 << 19;
        /// The group n°21.
        const GROUP_21 = 1 << 20;
        /// The group n°22.
        const GROUP_22 = 1 << 21;
        /// The group n°23.
        const GROUP_23 = 1 << 22;
        /// The group n°24.
        const GROUP_24 = 1 << 23;
        /// The group n°25.
        const GROUP_25 = 1 << 24;
        /// The group n°26.
        const GROUP_26 = 1 << 25;
        /// The group n°27.
        const GROUP_27 = 1 << 26;
        /// The group n°28.
        const GROUP_28 = 1 << 27;
        /// The group n°29.
        const GROUP_29 = 1 << 28;
        /// The group n°30.
        const GROUP_30 = 1 << 29;
        /// The group n°31.
        const GROUP_31 = 1 << 30;
        /// The group n°32.
        const GROUP_32 = 1 << 31;

        /// All of the groups.
        const ALL = u32::MAX;
        /// None of the groups.
        const NONE = 0;
    }
}

impl Default for GroupWrapper {
    fn default() -> Self {
        GroupWrapper::ALL
    }
}

impl From<GroupWrapper> for Group {
    fn from(value: GroupWrapper) -> Self {
        Self::from_bits_truncate(value.bits())
    }
}

impl From<Group> for GroupWrapper {
    fn from(value: Group) -> Self {
        Self::from_bits_truncate(value.bits())
    }
}

#[derive(Component, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct SolverGroupsFlag {
    pub memberships: GroupWrapper,
    pub filters: GroupWrapper,
}

impl From<&SolverGroupsFlag> for SolverGroups {
    fn from(value: &SolverGroupsFlag) -> Self {
        Self {
            memberships: value.memberships.into(),
            filters: value.filters.into(),
        }
    }
}

impl From<&SolverGroups> for SolverGroupsFlag {
    fn from(value: &SolverGroups) -> Self {
        Self {
            memberships: value.memberships.into(),
            filters: value.memberships.into(),
        }
    }
}

impl ManagedTypeRegistration for SolverGroupsFlag {
    fn get_all_type_registrations() -> Vec<TypeRegistration> {
        Vec::new()
    }
}
