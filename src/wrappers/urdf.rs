
// use bevy::core::Name;
use bevy::{prelude::*, ecs::query::WorldQuery};
use bevy_egui::EguiContext;
use urdf_rs::{Robot, Joint};

use crate::traits::FromStructure;

// use super::{material::MaterialFlag, link::LinkFlag, joint::JointFlag};
// // pub struct LinkWrapper {}

// type RobotPropertiesAsTuple = (String, LinkFlag, JointFlag, MaterialFlag);

// impl FromStructure<RobotPropertiesAsTuple> for Robot {
//     fn from_world(value: RobotPropertiesAsTuple, world: &World) -> Self {
//         //let b = value.2;
//         let links_query = world.query::<value.1>();
//         let joints_query = world.query::<value.2>();
//         let materials_query = world.query::<value.3>();

//     }

// }
// pub struct UrdfFlag {
//     pub name: String,

//     pub links: Vec<Link>,

//     pub joints: Vec<Joint>,

//     pub materials: Vec<Material>,

// };