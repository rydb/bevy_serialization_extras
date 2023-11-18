
// use bevy::core::Name;
use bevy::{prelude::*, ecs::query::WorldQuery};
use urdf_rs::Robot;

use crate::traits::{FromStructure, Structure};

use super::{mass::MassFlag, mesh::GeometryFlag, colliders::ColliderFlag, material::{MaterialFlag, MaterialFile}, link::{LinkFlag, JointFlag}};

// use super::{material::MaterialFlag, link::LinkFlag, joint::JointFlag};
// // pub struct LinkWrapper {}


#[derive(WorldQuery)]
pub struct UrdfQuery {
    link: &'static LinkFlag,
    visual: &'static GeometryFlag,
    joint: Option<&'static JointFlag>,
    material: Option<FileCheck<MaterialFlag, MaterialFile>>,
}


// impl<'w, 's> From<Query<'w, 's, &UrdfQuery>> for Robot {

// }

trait FromQuery<T: WorldQuery> {
    fn from_query(query: Query<T>);
}

impl Structure<&UrdfQueryItem<'_>> for UrdfQuery {
    fn structure(value: &UrdfQueryItem<'_>) -> String {
        value.link.name.clone()
    }
}

// #[derive(WorldQuery)]
// pub struct UrdfQuery {
//     link: &'static LinkFlag,
//     visual: &'static GeometryFlag,
//     joint: Option<&'static JointFlag>,
//     material: Option<&'static MaterialFlag>,
// }

#[derive(WorldQuery)]
pub struct FileCheck<T, U>
    where
        T: Component,
        U: Component 
{
    component: &'static T,
    component_file: Option<&'static U>


}

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

pub struct UrdfStructure {
    
    pub structure: String,
    pub links: Vec<LinkFlag>,
    pub joints: Vec<JointFlag>,
    pub materials: Vec<MaterialFlag>

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