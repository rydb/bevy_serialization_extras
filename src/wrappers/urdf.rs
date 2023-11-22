
// use bevy::core::Name;
use bevy::{prelude::*, ecs::query::WorldQuery, utils::HashMap};
use urdf_rs::Robot;

use crate::{traits::{FromStructure, Structure}, queries::FileCheck};

use super::{mesh::{GeometryFlag, GeometryFile, GeometrySource}, material::{MaterialFlag, MaterialFile, MaterialSource}, link::{JointFlag, LinkQuery}};

// use super::{material::MaterialFlag, link::LinkFlag, joint::JointFlag};
// // pub struct LinkWrapper {}


// #[derive(WorldQuery, Clone)]
// pub struct UrdfQuery {
//     pub link: &'static LinkFlag,
//     pub visual: FileCheck<GeometryFlag, GeometryFile>,
//     pub joint_check: Option<&'static JointFlag>,
//     pub material_check: Option<FileCheck<MaterialFlag, MaterialFile>>,
// }


// impl FromQuery<LinkQuery> for UrdfStructure {
//     fn from_query(query: Query<LinkQuery>) {
        
//     }
// }

// impl FromQuery<UrdfQuery> for UrdfStructure {
//     fn from_query(query: Query<UrdfQuery>) {
//         let mut urdf_structure = UrdfStructure::default();
//         for item in query.iter() {
//             //let x = item.link.clone();
//             urdf_structure.links.push(item.link.clone());
//             urdf_structure.visuals.push(
//                 match item.visual.component_file {
//                     None => GeometrySource::Primitive(item.visual.component.clone()),
//                     Some(file) => GeometrySource::File(file.clone()) 
//                 }

//             );
//             urdf_structure.materials.push(
//             if let Some(material) = item.material_check {

//                     match material.component_file {
//                         None => Some(MaterialSource::Wrapper(material.component.clone())),
//                         Some(file) => Some(MaterialSource::File(file.clone()))
//                     }
                

//             } else {
//                 None
//             }
//             );
//             urdf_structure.joints.push(
//                 if let Some(joint) = item.joint_check {
//                     Some(joint.clone())
//                 } else {
//                     None
//                 }
//             )
//         }
//     }
// }

trait FromQuery<T: WorldQuery> {
    fn from_query(query: Query<T>);
}

// impl Structure<&UrdfQueryItem<'_>> for UrdfQuery {
//     fn structure(value: &UrdfQueryItem<'_>) -> String {
//         value.link.name.clone()
//     }
// }

// #[derive(WorldQuery)]
// pub struct UrdfQuery {
//     link: &'static LinkFlag,
//     visual: &'static GeometryFlag,
//     joint: Option<&'static JointFlag>,
//     material: Option<&'static MaterialFlag>,
// }

// #[derive(Debug, WorldQuery, Clone)]
// pub struct FileCheck<T, U>
//     where
//         T: Component + Clone,
//         U: Component + Clone, 
// {
//     pub component: &'static T,
//     pub component_file: Option<&'static U>


// }


// pub fn file_check<T, U>(
//     file_query: Query<(Entity, &T, Option<&U>)>
// ) -> Vec<(Entity, Result<T, U>)>
//     where
//         T: Component,
//         U: Component
// {
//     let v = Vec::new();
//     for (e, thing, thing_file_check) in file_query.iter() {
//             match thing_file_check {
//                 Some(file) => v.push((e, Ok(file))),
//                 None => v.push((e, Err(thing)))
//             }
//     }
//     return v
// }

// impl FileCheck<MaterialFile> for MaterialFlag {
//     fn return_file(file_query: Query<(&Self, Option<&MaterialFile>)>) -> Vec<Result<MaterialFile, Self>> {
//         let v = Vec::new();

//         for (item, file_check) in file_query.iter() {
//             match file_check {
//                 Some(file) => v.push(Ok(file.clone())),
//                 None => v.push(Err(item))
//             }
//         }
//         return v
//     }
// }

// pub trait FileCheck<T> 
//     where
//         Self: Sized + Component,
//         T: Component
// {
//     fn return_file(file_query: Query<(&Self, Option<&T>)>) -> Vec<Result<T, Self>>;
// }
// #[derive(Default)]
// pub struct UrdfStructure {
    
//     pub name: String,
//     pub links: Vec<LinkFlag>,
//     pub visuals: Vec<GeometrySource>,
//     pub joints: Vec<Option<JointFlag>>,
//     pub materials: Vec<Option<MaterialSource>>

// }

#[derive(Default, Resource)]
pub struct Urdfs {
    pub world_urdfs: HashMap<String, Robot>
}

// /// Top level struct to access urdf.
// #[derive(Debug, YaDeserialize, YaSerialize, Clone)]
// #[yaserde(rename = "robot", namespace = "http://www.ros.org")]
// pub struct Robot {
//     #[yaserde(attribute)]
//     pub name: String,

//     #[yaserde(rename = "link")]
//     pub links: Vec<Link>,

//     #[yaserde(rename = "joint")]
//     pub joints: Vec<Joint>,

//     #[yaserde(rename = "material")]
//     pub materials: Vec<Material>,
// }