
use bevy::prelude::{Component, Transform};
use bevy_rapier3d::prelude::ImpulseJoint;

use crate::traits::FromStructure;

use super::{mesh::GeometryFlag, colliders::ColliderFlag, mass::MassFlag};
pub struct LinkStructure {
    pub name: String, 
}

#[derive(Component)]
pub struct LinkFlag {
    pub name: LinkStructure,
    pub inertial: MassFlag,
    pub visual: GeometryFlag,
    pub collision: ColliderFlag,
}

/// all of the things that compose a "Link"
type LinkAsTuple = (LinkStructure, MassFlag, GeometryFlag, ColliderFlag);

impl From<LinkAsTuple> for LinkFlag {
    fn from(value: LinkAsTuple) -> Self {
        Self {
            name: value.0,
            inertial: value.1,
            visual: value.2,
            collision: value.3,
        }
    }
}

impl From<LinkFlag> for LinkAsTuple {
    fn from(value: LinkFlag) -> Self {
        (
            value.name,
            value.inertial,
            value.visual,
            value.collision
        )
    }
}

pub struct Dynamics {
    pub damping: f64,
    pub friction: f64,
}

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

/// Sends joint movements to all joint recievers with equivilent ids. 
#[derive(Component)]
pub struct JointSenderFlag {
    pub id: String
}

#[derive(Component)]
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
    pub dynamics: Option<Dynamics>,
    //pub mimic: Option<Mimic>
    //pub safety_controller: Option<SafetyController>,

}

// impl FromStructure<(LinkFlag, JointFlag)> for ImpulseJoint {
//     fn from_world(value: (LinkFlag, JointFlag), world: &bevy::prelude::World) -> Self {
        
//     }
// }

// impl From<ImpulseJoint> for JointFlag {
//     fn from(value: ImpulseJoint) -> Self {
        
//     }
// }

// /// Urdf Link element
// /// See <http://wiki.ros.org/urdf/XML/link> for more detail.
// #[derive(Debug, YaDeserialize, YaSerialize, Clone)]
// pub struct Link {
//     #[yaserde(attribute)]
//     pub name: String,
//     pub inertial: Inertial,
//     pub visual: Vec<Visual>,
//     pub collision: Vec<Collision>,
// }