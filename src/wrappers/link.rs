
use bevy::{prelude::{Component, Transform}, ecs::query::WorldQuery, reflect::GetTypeRegistration};
use bevy_rapier3d::{prelude::ImpulseJoint, na::SimdBool, parry::math::Isometry};
use urdf_rs::{Joint, JointLimit};
use rapier3d::{dynamics::{GenericJoint, JointAxesMask, JointLimits, JointMotor}, na::Isometry3};
use crate::{traits::ManagedTypeRegistration, queries::FileCheck};
use bevy::prelude::*;



use super::{mesh::{GeometryFlag, GeometryFile}, colliders::ColliderFlag, mass::MassFlag};

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
pub struct JointLimitWrapper {
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

impl From<&Joint> for JointFlag {
    fn from(value: &Joint) -> Self {
        Self {
            offset: Transform {
                translation: Vec3::new(value.origin.xyz.0[0] as f32, value.origin.xyz.0[1] as f32, value.origin.xyz.0[2] as f32),
                rotation: Quat::default(),
                ..default()
            },
            reciever: value.child.link.clone(),
            limit: JointLimitWrapper {
                 lower:  value.limit.lower, 
                 upper: value.limit.upper, 
                 effort: value.limit.effort, 
                 velocity: value.limit.velocity
            },
            dynamics: {
                match value.dynamics.clone() {
                    Some(dynamics) => 
                        Dynamics {
                            damping: dynamics.damping,
                            friction: dynamics.friction,
                        },
                    None => Dynamics::default()
                    
                }
            },
            local_frame1: Transform::default(),
            local_frame2: Transform::default(),
            locked_axes: {
                //clamp axis to between 0-1 for simplicity and for bitmask flipping
                let unit_axis = value.axis.xyz.0
                .map(|n| n.clamp(0.0, 1.0))
                .map(|n| n as u8);
                let mut x = 1 << unit_axis[0];
                x = x | (2 << unit_axis[1]);
                x = x | (3 << unit_axis[2]);
                JointAxesMaskWrapper::from_bits_truncate(x)
            },
            limit_axes: JointAxesMaskWrapper::all(),
            motor_axes: JointAxesMaskWrapper::all(),
            coupled_axes: JointAxesMaskWrapper::all(),
            contacts_enabled: true,
            enabled: true
        }
    }
}

impl From<&JointFlag> for GenericJoint {
    fn from(value: &JointFlag) -> Self {
        //GenericJoint::(locked_axes)
        let joint_limit = JointLimits {
            max: value.limit.upper as f32,
            min: value.limit.lower as f32,
            impulse: value.limit.velocity as f32,
        };
        let joint_motor = JointMotor::default();
        Self {
            local_frame1: Isometry3 {
                translation: value.local_frame1.translation.into(),
                rotation: value.local_frame1.rotation.into()
            },
            local_frame2: Isometry3 {
                translation: value.local_frame2.translation.into(),
                rotation: value.local_frame2.rotation.into()
            },
            locked_axes: JointAxesMask::from_bits_truncate(value.locked_axes.bits()),
            limit_axes: JointAxesMask::from_bits_truncate(value.limit_axes.bits()),
            motor_axes: JointAxesMask::from_bits_truncate(value.motor_axes.bits()),
            coupled_axes: JointAxesMask::from_bits_truncate(value.coupled_axes.bits()),
            //FIXME: this is probably wrong...
            limits: [joint_limit, joint_limit, joint_limit, joint_limit, joint_limit, joint_limit],
            motors: [joint_motor, joint_motor, joint_motor, joint_motor, joint_motor, joint_motor],
            contacts_enabled: value.contacts_enabled,
            //FIXME:  fix jointflag to have a proper enum for this later
            enabled: rapier3d::dynamics::JointEnabled::Enabled,
        }
    }
}

impl From<&LinkageItem<'_>> for ImpulseJoint {
    fn from(value: &LinkageItem) -> Self {
        let joint = GenericJoint::from(value.joint);
        let bevy_rapier_joint = bevy_rapier3d::dynamics::GenericJoint { raw: joint };
        Self { 
            parent: value.entity,
            data: bevy_rapier_joint, 
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

// #[derive(Reflect)]
// struct Foo(u32);

// bitflags::bitflags! {
//     impl Foo: u32 {
//         const A = 0;
//         const B = 1;
//     }
// }

#[derive(Reflect, Clone, Default)]
pub struct JointAxesMaskWrapper(u8);

bitflags::bitflags! {
    impl JointAxesMaskWrapper: u8 {
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
        const LOCKED_REVOLUTE_AXES = Self::X.bits() | Self::Y.bits() | Self::Z.bits() | Self::ANG_Y.bits() | Self::ANG_Z.bits();
        /// The set of degrees of freedom locked by a prismatic joint.
        const LOCKED_PRISMATIC_AXES = Self::Y.bits()| Self::Z.bits()| Self::ANG_X.bits()| Self::ANG_Y.bits()| Self::ANG_Z.bits();
        /// The set of degrees of freedom locked by a fixed joint.
        const LOCKED_FIXED_AXES = Self::X.bits()| Self::Y.bits()| Self::Z.bits()| Self::ANG_X.bits()| Self::ANG_Y.bits()| Self::ANG_Z.bits();
        /// The set of degrees of freedom locked by a spherical joint.
        const LOCKED_SPHERICAL_AXES = Self::X.bits()| Self::Y.bits()| Self::Z.bits();
        /// The set of degrees of freedom left free by a revolute joint.
        const FREE_REVOLUTE_AXES = Self::ANG_X.bits();
        /// The set of degrees of freedom left free by a prismatic joint.
        const FREE_PRISMATIC_AXES = Self::X.bits();
        /// The set of degrees of freedom left free by a fixed joint.
        const FREE_FIXED_AXES = 0;
        /// The set of degrees of freedom left free by a spherical joint.
        const FREE_SPHERICAL_AXES = Self::ANG_X.bits()| Self::ANG_Y.bits()| Self::ANG_Z.bits();
        /// The set of all translational degrees of freedom.
        const LIN_AXES = Self::X.bits() | Self::Y.bits() | Self::Z.bits();
        /// The set of all angular degrees of freedom.
        const ANG_AXES = Self::ANG_X.bits() | Self::ANG_Y.bits() | Self::ANG_Z.bits();
    }
}

// pub struct LazyIsometry {
//     transform: Transform,
// }

// pub struct t {
//     ttt: ImpulseJoint
// }

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
    pub limit: JointLimitWrapper,
    pub dynamics: Dynamics,
    //pub mimic: Option<Mimic>
    //pub safety_controller: Option<SafetyController>,

    /// The joint’s frame, expressed in the first rigid-body’s local-space.
    pub local_frame1: Transform,
    // / The joint’s frame, expressed in the second rigid-body’s local-space.
    pub local_frame2: Transform,
    // / The degrees-of-freedoms locked by this joint.
    pub locked_axes: JointAxesMaskWrapper,
    /// The degrees-of-freedoms limited by this joint.
    pub limit_axes: JointAxesMaskWrapper,
    /// The degrees-of-freedoms motorised by this joint.
    pub motor_axes: JointAxesMaskWrapper,
    /// The coupled degrees of freedom of this joint.
    pub coupled_axes: JointAxesMaskWrapper,
    // The limits, along each degrees of freedoms of this joint.
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

        type_registry.push(JointLimitWrapper::get_type_registration());
        type_registry.push(Dynamics::get_type_registration());
        type_registry.push(JointFlag::get_type_registration());

        return type_registry
        
    }
}

