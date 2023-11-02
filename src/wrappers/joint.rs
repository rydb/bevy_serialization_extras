use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use urdf_rs::{Mimic, SafetyController};

pub struct JointStructure {
    pub name: String, 
}




//PrismaticJointBuilder
// pub struct JointTypeFlag {
//     pub data: 
// }


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

#[derive(Component)]
pub struct JointFlag {
    pub name: JointStructure,
    //pub joint_type:
    pub origin: Transform,
    pub parent: String,
    pub child: String,
    //pub axis: 
    pub limit: JointLimit,
    pub dynamics: Option<Dynamics>,
    //pub mimic: Option<Mimic>
    //pub safety_controller: Option<SafetyController>,

}
// Urdf Joint element
// See <http://wiki.ros.org/urdf/XML/joint> for more detail.
// #[derive(Debug, YaDeserialize, YaSerialize, Clone)]
// pub struct Joint {
//     #[yaserde(attribute)]
//     pub name: String,
//     #[yaserde(attribute, rename = "type")]
//     pub joint_type: JointType,
//     pub origin: Pose,
//     pub parent: LinkName,
//     pub child: LinkName,
//     pub axis: Axis,
//     pub limit: JointLimit,
//     pub dynamics: Option<Dynamics>,
//     pub mimic: Option<Mimic>,
//     pub safety_controller: Option<SafetyController>,
// }