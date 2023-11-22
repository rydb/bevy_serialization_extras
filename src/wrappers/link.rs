
use bevy::{prelude::{Component, Transform}, ecs::query::WorldQuery, reflect::GetTypeRegistration};
use bevy_rapier3d::{prelude::ImpulseJoint, na::SimdBool};
use bevy::ecs::world::World;
use urdf_rs::Joint;
use crate::{traits::{FromStructure, Structure, AssociatedEntity, Unfold, ManagedTypeRegistration}, queries::FileCheck};
use bevy::prelude::*;



use super::{mesh::{GeometryFlag, GeometryFile}, colliders::ColliderFlag, mass::MassFlag, urdf};
// pub struct LinkStructure {
//     pub name: String, 
// }

// #[derive(Component, Clone)]
// pub struct LinkFlag {
//     pub structure: String,
//     pub name: String,
//     pub inertial: MassFlag,
//     pub visual: GeometryFlag,
//     pub collision: ColliderFlag,
// }

/// the "super-structure" that this entity is related to, relevant for serializing disconnected by related entities 
#[derive(Component)]
pub struct StructureFlag {
    pub name: String,
}

/// the collection of things that qualify as a "link", in the ROS 2 context. 
#[derive(WorldQuery)]
pub struct LinkQuery {
    pub name: Option<&'static Name>,
    pub structure: &'static StructureFlag,
    pub inertial: Option<&'static MassFlag>,
    pub visual: FileCheck<GeometryFlag, GeometryFile>,
    pub collision: Option<&'static ColliderFlag>,
    pub joint: Option<&'static JointFlag>,
}
#[derive(Default, Reflect, Clone)]
pub struct Dynamics {
    pub damping: f64,
    pub friction: f64,
}

#[derive(Default, Reflect, Clone)]
pub struct JointLimit {
    pub lower: f64,
    pub upper: f64,
    pub effort: f64,
    pub velocity: f64,
}

/// Recieves joint movements from joint sender flag
#[derive(Component)]
pub struct JointRecieverFlag {
    pub id: String
}


#[derive(WorldQuery)]
pub struct Linkage {
    entity: Entity,
    // It is required that all reference lifetimes are explicitly annotated, just like in any
    // struct. Each lifetime should be 'static.
    link: LinkQuery,
    joint: &'static JointFlag,
}



impl From<&LinkageItem<'_>> for ImpulseJoint {
    fn from(value: &LinkageItem) -> Self {
        Self { 
            parent: value.entity,
            data: self::default(), // for debug, need to implement this later
        }
    }
}

impl From<&ImpulseJoint> for JointFlag {
    fn from(value: &ImpulseJoint) -> Self {
        Self {
            ..default()
        }
    }
}

/// take the properties of a joint, and infer what joint type it is based on those properties.
pub fn infer_joint_type() {

}

// #[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
// #[derive(Copy, Clone, Debug, PartialEq)]
// /// A generic joint.
// pub struct GenericJoint {
//     /// The joint’s frame, expressed in the first rigid-body’s local-space.
//     pub local_frame1: Isometry<Real>,
//     /// The joint’s frame, expressed in the second rigid-body’s local-space.
//     pub local_frame2: Isometry<Real>,
//     /// The degrees-of-freedoms locked by this joint.
//     pub locked_axes: JointAxesMask,
//     /// The degrees-of-freedoms limited by this joint.
//     pub limit_axes: JointAxesMask,
//     /// The degrees-of-freedoms motorised by this joint.
//     pub motor_axes: JointAxesMask,
//     /// The coupled degrees of freedom of this joint.
//     pub coupled_axes: JointAxesMask,
//     /// The limits, along each degrees of freedoms of this joint.
//     ///
//     /// Note that the limit must also be explicitly enabled by the `limit_axes` bitmask.
//     pub limits: [JointLimits<Real>; SPATIAL_DIM],
//     /// The motors, along each degrees of freedoms of this joint.
//     ///
//     /// Note that the mostor must also be explicitly enabled by the `motors` bitmask.
//     pub motors: [JointMotor; SPATIAL_DIM],
//     /// Are contacts between the attached rigid-bodies enabled?
//     pub contacts_enabled: bool,
//     /// Whether or not the joint is enabled.
//     pub enabled: JointEnabled,
// }

// impl Default for GenericJoint {
//     fn default() -> Self {
//         Self {
//             local_frame1: Isometry::identity(),
//             local_frame2: Isometry::identity(),
//             locked_axes: JointAxesMask::empty(),
//             limit_axes: JointAxesMask::empty(),
//             motor_axes: JointAxesMask::empty(),
//             coupled_axes: JointAxesMask::empty(),
//             limits: [JointLimits::default(); SPATIAL_DIM],
//             motors: [JointMotor::default(); SPATIAL_DIM],
//             contacts_enabled: true,
//             enabled: JointEnabled::Enabled,
//         }
//     }
// }

bitflags::bitflags! {
    /// A bit mask identifying multiple degrees of freedom of a joint.
    #[derive(Reflect, Default)]
    pub struct JointAxesMask: u8 {
        /// The translational degree of freedom along the local X axis of a joint.
        const X = 1 << 0;
        /// The translational degree of freedom along the local Y axis of a joint.
        const Y = 1 << 1;
        /// The translational degree of freedom along the local Z axis of a joint.
        const Z = 1 << 2;
        /// The angular degree of freedom along the local X axis of a joint.
        const ANG_X = 1 << 3;
        /// The angular degree of freedom along the local Y axis of a joint.
        const ANG_Y = 1 << 4;
        /// The angular degree of freedom along the local Z axis of a joint.
        const ANG_Z = 1 << 5;
        /// The set of degrees of freedom locked by a revolute joint.
        const LOCKED_REVOLUTE_AXES = Self::X.bits | Self::Y.bits | Self::Z.bits | Self::ANG_Y.bits | Self::ANG_Z.bits;
        /// The set of degrees of freedom locked by a prismatic joint.
        const LOCKED_PRISMATIC_AXES = Self::Y.bits | Self::Z.bits | Self::ANG_X.bits | Self::ANG_Y.bits | Self::ANG_Z.bits;
        /// The set of degrees of freedom locked by a fixed joint.
        const LOCKED_FIXED_AXES = Self::X.bits | Self::Y.bits | Self::Z.bits | Self::ANG_X.bits | Self::ANG_Y.bits | Self::ANG_Z.bits;
        /// The set of degrees of freedom locked by a spherical joint.
        const LOCKED_SPHERICAL_AXES = Self::X.bits | Self::Y.bits | Self::Z.bits;
        /// The set of degrees of freedom left free by a revolute joint.
        const FREE_REVOLUTE_AXES = Self::ANG_X.bits;
        /// The set of degrees of freedom left free by a prismatic joint.
        const FREE_PRISMATIC_AXES = Self::X.bits;
        /// The set of degrees of freedom left free by a fixed joint.
        const FREE_FIXED_AXES = 0;
        /// The set of degrees of freedom left free by a spherical joint.
        const FREE_SPHERICAL_AXES = Self::ANG_X.bits | Self::ANG_Y.bits | Self::ANG_Z.bits;
        /// The set of all translational degrees of freedom.
        const LIN_AXES = Self::X.bits() | Self::Y.bits() | Self::Z.bits();
        /// The set of all angular degrees of freedom.
        const ANG_AXES = Self::ANG_X.bits() | Self::ANG_Y.bits() | Self::ANG_Z.bits();
    }
}

#[derive(Component, Default, Reflect, Clone)]
pub struct JointFlag {
    //pub name: JointStructure,
    //pub joint_type:
    pub offset: Transform,
    // the parent is the entity holding the impulse joint, the "parent", is implied. A joint parent/reciever cannot exist if the parent doesnt exist,
    // but the exact parent is liable to change at runtime. Making the "parent", distiction redundant. 
    //pub parent: String,
    pub reciever: String,
    //pub axis: 
    pub limit: JointLimit,
    pub dynamics: Dynamics,
    //pub mimic: Option<Mimic>
    //pub safety_controller: Option<SafetyController>,

    /// The joint’s frame, expressed in the first rigid-body’s local-space.
    //pub local_frame1: Isometry<Real>,
    /// The joint’s frame, expressed in the second rigid-body’s local-space.
    //pub local_frame2: Isometry<Real>,
    /// The degrees-of-freedoms locked by this joint.
    pub locked_axes: JointAxesMask,
    /// The degrees-of-freedoms limited by this joint.
    pub limit_axes: JointAxesMask,
    /// The degrees-of-freedoms motorised by this joint.
    pub motor_axes: JointAxesMask,
    /// The coupled degrees of freedom of this joint.
    pub coupled_axes: JointAxesMask,
    /// The limits, along each degrees of freedoms of this joint.
    ///
    /// The motors, along each degrees of freedoms of this joint.
    ///
    /// Note that the mostor must also be explicitly enabled by the `motors` bitmask.
    //pub motors: [JointMotor; SPATIAL_DIM],
    /// Are contacts between the attached rigid-bodies enabled?
    pub contacts_enabled: bool,
    /// Whether or not the joint is enabled.
    pub enabled: bool,

}

impl ManagedTypeRegistration for JointFlag {
    fn get_all_type_registrations() -> Vec<bevy::reflect::TypeRegistration> {
        let mut type_registry = Vec::new();

        type_registry.push(JointLimit::get_type_registration());
        type_registry.push(Dynamics::get_type_registration());
        type_registry.push(JointFlag::get_type_registration());

        return type_registry
        
    }
}

