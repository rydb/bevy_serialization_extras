
use bevy::{prelude::{Component, Transform}, ecs::query::WorldQuery, reflect::GetTypeRegistration};
use bevy_rapier3d::prelude::ImpulseJoint;
use bevy_serialization_core::{queries::FileCheck, wrappers::mesh::{GeometryFile, GeometryFlag}, traits::{ChangeChecked, ManagedTypeRegistration}};
use nalgebra::{Matrix3, Vector3};
//use urdf_rs::{Joint, Pose, Link, Visual};
use rapier3d::{dynamics::{GenericJoint, JointAxesMask, JointLimits, JointMotor, MotorModel}, na::Isometry3};
use bevy::prelude::*;
use derive_more::From;

use super::{mass::MassFlag, colliders::ColliderFlag};



#[derive(Component, Default)]
pub struct JointBounded;



#[derive(Component, Clone, Copy, Default)]
pub struct GeometryShiftMarked;

/// flags entity geometry as already shifted to account for urdf origin
#[derive(Component, Clone, Copy, Default)]
pub struct GeometryShifted;



/// the "super-structure" that this entity is a part of. This is collecting related "parts" into their monolithic/object-oriented equivilent. 
#[derive(Reflect, Component, Clone)]
pub struct StructureFlag {
    pub name: String,
}



#[derive(Default, Debug, Reflect, Clone)]
pub struct Dynamics {
    pub damping: f64,
    pub friction: f64,
}

#[derive(Debug, Reflect, Clone)]
pub struct JointLimitWrapper {
    pub lower: f64,
    pub upper: f64,
    pub effort: f64,
    pub velocity: f64,
}

// default movement should be unrestrained.
impl Default for JointLimitWrapper {
    fn default() -> Self {
        Self {
            lower: f64::MIN,
            upper: f64::MAX,
            effort: f64::MAX,
            velocity: f64::MAX,
        }
    }
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
    //link: LinkQuery,
    joint: &'static JointFlag,
}

impl ChangeChecked for Linkage {
    type ChangeCheckedComp = JointFlag;
}

// /// enum for convinience functions from converting to different cordinate systems
// pub enum CordBasis {
//     Bevy(Transform), //x right, y up, z backwards
//     Urdf(Transform), // right hand rule
// }

#[derive(Component, Reflect)]
pub struct LinkFlag{
    pub geom_offset: Vec3
}



impl From<&JointFlag> for GenericJoint {
    fn from(value: &JointFlag) -> Self {
        //GenericJoint::(locked_axes)
        let joint_limit = JointLimits {
            max: value.limit.upper as f32,
            min: value.limit.lower as f32,
            impulse: value.limit.velocity as f32,
        };
        //FIXME: this is probably wrong...
        let joint_motors = &value.motors;
        Self {
            local_frame1: Isometry3 {
                translation: value.local_frame1.translation.into(),
                rotation: value.local_frame1.rotation.into()
            },
            local_frame2: Isometry3 {
                translation: value.local_frame2.unwrap_or_default().translation.into(),
                rotation: value.local_frame2.unwrap_or_default().rotation.into()
            },
            locked_axes: JointAxesMask::from_bits_truncate(value.locked_axes.bits()),
            limit_axes: JointAxesMask::from_bits_truncate(value.limit_axes.bits()),
            motor_axes: JointAxesMask::from_bits_truncate(value.motor_axes.bits()),
            coupled_axes: JointAxesMask::from_bits_truncate(value.coupled_axes.bits()),
            //FIXME: this is probably wrong...
            limits: [joint_limit, joint_limit, joint_limit, joint_limit, joint_limit, joint_limit],
            //FIXME: this is probably wrong...
            motors: [
                (&joint_motors[0]).into(), 
                (&joint_motors[1]).into(), 
                (&joint_motors[2]).into(), 
                (&joint_motors[3]).into(), 
                (&joint_motors[4]).into(), 
                (&joint_motors[5]).into()
            ],
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

            parent:  match value.joint.parent_id {
                Some(e) =>  e,
                None => value.entity
            },
            data: bevy_rapier_joint, 
        }
    }
}

// impl From<&JointFlag> for ImpulseJoint {
//         fn from(value: &JointFlag) -> Self {
//             let joint = GenericJoint::from(value);
//             let bevy_rapier_joint = bevy_rapier3d::dynamics::GenericJoint { raw: joint };
//             Self { 
//                 parent: value.entity,
//                 data: bevy_rapier_joint, 
//             }
//         }
//     }

impl From<&ImpulseJoint> for JointFlag {
    fn from(value: &ImpulseJoint) -> Self {
        //return Self::from(value.data.raw);
        
        let joint = value.data.raw;
        let joint_limit = JointLimitWrapper {
            lower: joint.limits[0].min.into(),
            upper: joint.limits[0].max.into(),
            effort: Default::default(),
            velocity: joint.limits[0].impulse.into(),
        };
        Self {
            //FIXME: this is probably wrong...
            //offset: Transform::from_xyz(0.0, 0.0, 0.0),
            parent_name: None,
            parent_id: Some(value.parent),//format!("{:#?}", value.parent),
            //FIXME: this is probably wrong...
            limit: joint_limit,
            //FIXME: implement this properly
            dynamics: Default::default(),
            local_frame1: Transform {
                translation: joint.local_frame1.translation.into(),
                rotation: joint.local_frame1.rotation.into(),
                //FIXME: implement this properly
                scale: default()
            },
            local_frame2: Some(Transform {
                translation: joint.local_frame2.translation.into(),
                rotation: joint.local_frame2.rotation.into(),
                //FIXME: implement this properly
                scale: default()
            }),
            locked_axes: JointAxesMaskWrapper::from_bits_truncate(joint.locked_axes.bits()),
            limit_axes: JointAxesMaskWrapper::from_bits_truncate(joint.limit_axes.bits()),

            motor_axes: JointAxesMaskWrapper::from_bits_truncate(joint.motor_axes.bits()),
            motors: [
                (&joint.motors[0]).into(),
                (&joint.motors[1]).into(), 
                (&joint.motors[2]).into(), 
                (&joint.motors[3]).into(),
                (&joint.motors[4]).into(),
                (&joint.motors[5]).into(),
            ],

            coupled_axes: JointAxesMaskWrapper::from_bits_truncate(joint.coupled_axes.bits()),
            contacts_enabled: joint.contacts_enabled,
            enabled: joint.is_enabled(),

        }
    }
}


#[derive(Reflect, Debug, PartialEq, Eq, Clone, Default)]
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
    //pub axis: 

// pub struct LazyIsometry {
//     transform: Transform,
// }

// pub struct t {
//     ttt: ImpulseJoint
// }

#[derive(Component, Debug, Default, Reflect, Clone)]
pub struct JointFlag {
    // removed. local_frame1 serves the same purpose. 
    //pub offset: Transform,

    //name of the parent "link" of the joint. Some joints may not have named parents, so this is optional
    pub parent_name: Option<String>,
    //the parent entity of this joint. Some joint parents may be referenced by name only, so this has have to be populated later down
    //the deserialization pipeline.
    pub parent_id: Option<Entity>,


    pub limit: JointLimitWrapper,
    pub dynamics: Dynamics,
    //pub mimic: Option<Mimic>
    //pub safety_controller: Option<SafetyController>,

    /// The joint’s frame, expressed in the first rigid-body’s local-space.
    pub local_frame1: Transform,
    // / The joint’s frame, expressed in the second rigid-body’s local-space.
    pub local_frame2: Option<Transform>,
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
    pub motors: [JointMotorWrapper; 6],
    /// Are contacts between the attached rigid-bodies enabled?
    pub contacts_enabled: bool,
    /// Whether or not the joint is enabled.
    pub enabled: bool,

}
#[derive(Reflect, Clone, Debug)]
pub struct JointMotorWrapper {
    /// The target velocity of the motor.
    pub target_vel: f32,
    /// The target position of the motor.
    pub target_pos: f32,
    /// The stiffness coefficient of the motor’s spring-like equation.
    pub stiffness: f32,
    /// The damping coefficient of the motor’s spring-like equation.
    pub damping: f32,
    /// The maximum force this motor can deliver.
    pub max_force: f32,
    /// The impulse applied by this motor.
    pub impulse: f32,
    /// The spring-like model used for simulating this motor.
    pub model: MotorModelWrapper,
}

impl Default for JointMotorWrapper {
    fn default() -> Self {
        Self {
            max_force: f32::MAX,
            target_vel: 0.0,
            target_pos: 0.0,
            stiffness: 0.0,
            damping: 0.0,
            impulse: 0.0,
            model: MotorModelWrapper::default()
        }        
    }
}

impl From<&JointMotor> for JointMotorWrapper {
    fn from(value: &JointMotor) -> Self {
        Self {
            target_vel: value.target_vel,
            target_pos: value.target_pos,
            stiffness: value.stiffness,
            damping: value.damping,
            max_force: value.max_force,
            impulse: value.impulse,
            model: (&value.model).into()
        }
    }
}

impl From<&JointMotorWrapper> for JointMotor {
    fn from(value: &JointMotorWrapper) -> Self {
        Self {
            target_vel: value.target_vel,
            target_pos: value.target_pos,
            stiffness: value.stiffness,
            damping: value.damping,
            max_force: value.max_force,
            impulse: value.impulse,
            model: (&value.model).into()
        }
    }
}

/// The spring-like model used for constraints resolution.
#[derive(Reflect, Clone, Debug, PartialEq, Eq, Default)]
pub enum MotorModelWrapper {
    /// The solved spring-like equation is:
    /// `acceleration = stiffness * (pos - target_pos) + damping * (vel - target_vel)`
    #[default]
    AccelerationBased,
    /// The solved spring-like equation is:
    /// `force = stiffness * (pos - target_pos) + damping * (vel - target_vel)`
    ForceBased,
}

impl From<&MotorModel> for MotorModelWrapper {
    fn from(value: &MotorModel) -> Self {
        match value {
            MotorModel::AccelerationBased => Self::AccelerationBased,
            MotorModel::ForceBased => Self::ForceBased
        }
    }
}

impl From<&MotorModelWrapper> for MotorModel {
    fn from(value: &MotorModelWrapper) -> Self {
        match value {
            MotorModelWrapper::AccelerationBased => Self::AccelerationBased,
            MotorModelWrapper::ForceBased => Self::ForceBased
        }
    }
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

