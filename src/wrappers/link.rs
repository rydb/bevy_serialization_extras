
use bevy::{prelude::{Component, Transform}, ecs::query::WorldQuery, reflect::GetTypeRegistration};
use bevy_rapier3d::prelude::ImpulseJoint;
use nalgebra::{Matrix3, Vector3};
use urdf_rs::{Joint, Pose, Link, Visual};
use rapier3d::{dynamics::{GenericJoint, JointAxesMask, JointLimits, JointMotor}, na::Isometry3};
use crate::{traits::{ManagedTypeRegistration, ChangeChecked}, queries::FileCheck};
use bevy::prelude::*;
use derive_more::From;


use super::{mesh::{GeometryFlag, GeometryFile}, colliders::ColliderFlag, mass::MassFlag};

/// the "super-structure" that this entity is related to, relevant for serializing disconnected by related entities 
#[derive(Reflect, Component, Clone)]
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
    //link: LinkQuery,
    joint: &'static JointFlag,
}

impl ChangeChecked for Linkage {
    type ChangeCheckedComp = JointFlag;
}

/// enum for convinience functions from converting to different cordinate systems
pub enum CordBasis {
    Bevy(Transform), //x right, y up, z backwards
    Urdf(Transform), // right hand rule
}

#[derive(Component, Reflect)]
pub struct LinkFlag{
    pub geom_offset: Vec3
}

impl From<&Link> for LinkFlag {
    fn from(value: &Link) -> Self {
        let visual = value.visual.first()
        .unwrap_or(&Visual::default())
        .to_owned();
        Self {
            //FIXME: implement this properly to account for urdfs with multiple visual elements 
            geom_offset: Vec3::from_array([visual.origin.xyz[0] as f32, visual.origin.xyz[1] as f32, visual.origin.xyz[2] as f32])
        }
    }
}

#[derive(From)]
pub struct UrdfTransform(Pose);

impl From<UrdfTransform> for Transform {
    fn from(value: UrdfTransform) -> Self {
        // based on this explanation
        //https://towardsdatascience.com/change-of-basis-3909ef4bed43
        let urdf_cord_flip = Matrix3::new(
            1.0, 0.0, 0.0,
            0.0, 0.0, 1.0,
            0.0, 1.0, 0.0,
        );
        // based on this explanation
        //https://stackoverflow.com/questions/31191752/right-handed-euler-angles-xyz-to-left-handed-euler-angles-xyz
        let urdf_rotation_flip = Matrix3::new(
            -1.0, 0.0, 0.0,
            0.0, -1.0, 0.0,
            0.0, 0.0, 1.0,
        );
        let pos = value.0;
        
        let compliant_trans = 
            urdf_cord_flip * Vector3::new(pos.xyz.0[0], pos.xyz.0[1], pos.xyz.0[2]);
        let compliant_rot = 
            urdf_rotation_flip * Vector3::new(pos.rpy.0[0], pos.rpy.0[1], pos.rpy.0[2]);


        println!("origin is {:#?}", pos);
        // let mut new_trans = Transform::from_translation(Vec3::new(compliant_trans.x as f32, compliant_trans.y as f32, compliant_trans.z as f32));
        // new_trans.rotate_local_x(180.0);
        Self {
            translation:  Vec3::new(compliant_trans.x as f32, compliant_trans.y as f32, compliant_trans.z as f32),
            rotation: Quat::from_euler(EulerRot::XYZ, compliant_rot.x as f32, compliant_rot.y as f32, compliant_rot.z as f32),
            ..default()
        }
    }
} 

impl From<&Joint> for JointFlag {
    fn from(value: &Joint) -> Self {
        Self {
            offset: Transform {
                 translation: Vec3::new(value.origin.xyz.0[0] as f32, value.origin.xyz.0[1] as f32, value.origin.xyz.0[2] as f32),
                // rotation: Quat::default(),
                ..default()
            },
            parent_name: Some(value.parent.link.clone()),
            parent_id: None,
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
            local_frame1: UrdfTransform::from(value.origin.clone()).into(),
            local_frame2: Transform::default(),
            locked_axes: {
                //clamp axis to between 0-1 for simplicity and for bitmask flipping
                // let unit_axis = value.axis.xyz.0
                // .map(|n| n.clamp(0.0, 1.0))
                // .map(|n| n as u8);
                // let mut x = 1 << unit_axis[0];
                // x = x | (2 << unit_axis[1]);
                // x = x | (3 << unit_axis[2]);
                // JointAxesMaskWrapper::from_bits_truncate(x)
                JointAxesMaskWrapper::LOCKED_FIXED_AXES
            },
            limit_axes: JointAxesMaskWrapper::empty(),
            motor_axes: JointAxesMaskWrapper::empty(),
            coupled_axes: JointAxesMaskWrapper::empty(),
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
            offset: Transform::from_xyz(0.0, 0.0, 0.0),
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
            local_frame2: Transform {
                translation: joint.local_frame2.translation.into(),
                rotation: joint.local_frame2.rotation.into(),
                //FIXME: implement this properly
                scale: default()
            },
            locked_axes: JointAxesMaskWrapper::from_bits_truncate(joint.locked_axes.bits()),
            limit_axes: JointAxesMaskWrapper::from_bits_truncate(joint.limit_axes.bits()),
            motor_axes: JointAxesMaskWrapper::from_bits_truncate(joint.motor_axes.bits()),
            coupled_axes: JointAxesMaskWrapper::from_bits_truncate(joint.coupled_axes.bits()),
            contacts_enabled: joint.contacts_enabled,
            enabled: joint.is_enabled(),

        }
    }
}


#[derive(Reflect, PartialEq, Eq, Clone, Default)]
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

#[derive(Component, Default, Reflect, Clone)]
pub struct JointFlag {
    //pub name: JointStructure,
    //pub joint_type:
    pub offset: Transform,

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

