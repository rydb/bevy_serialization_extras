
use std::collections::HashMap;

// use bevy::core::Name;
use bevy::{prelude::*, ecs::query::WorldQuery};
use urdf_rs::{Robot, Joint, Pose};

use crate::{traits::{FromStructure, Structure}, queries::FileCheck};

use super::{mesh::{GeometryFlag, GeometryFile, GeometrySource}, material::{MaterialFlag, MaterialFile, MaterialSource}, link::{JointFlag, LinkQuery, JointAxesMask}};


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

// pub struct UrdfQuery {

// }

//ReflectComponent

#[derive(Default, Resource)]
pub struct Urdfs {
    pub world_urdfs: HashMap<String, Robot>
}

// impl Iterator for Urdfs {
//     type Item = Robot;
//     fn next(&mut self) -> Option<Self::Item> {
//         // ???
//     }
// }

impl<'a> IntoIterator for &'a Urdfs {
    type Item = &'a Robot;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        let mut values = Vec::new();
        for value in self.world_urdfs.values() {
            values.push(value)
        }
        values.into_iter()

    }
}

// #[derive(Hash, Resource)]
// pub struct Urdfs {
//     name: String,
//     contents: Robot,
// }


impl From<Query<'_, '_, LinkQuery>> for Urdfs {
    fn from(value: Query<LinkQuery>) -> Self {
        let mut urdf_map = Self::default();
        for link in value.iter() {
            let structure_name = link.structure.name.clone();
            let entry = urdf_map.world_urdfs.entry(structure_name.clone())
            .or_insert(Robot { name: link.structure.name.clone(), links: Vec::new(), joints: Vec::new(), materials: Vec::new() })
            ;
            
            match link.joint {
                Some(joint) => {
                    let link_name = link.name.unwrap_or(&Name::new(entry.joints.len().to_string())).to_string();
                    let joint_name = link_name.clone() + "_joint";
                    //let urdf_link_name = link_name + "_link";
                    entry.joints.push
                    (
                        Joint 
                        {
                            name: joint_name,
                            //(TODO) implement this properly have this be a consequence of joint data via a function. This is a placeholder.
                            joint_type: urdf_rs::JointType::Continuous,
                            origin: Pose {
                                xyz: urdf_rs::Vec3([joint.offset.translation.x.into(), joint.offset.translation.y.into(), joint.offset.translation.z.into()]),
                                rpy: {
                                    let rot = joint.offset.rotation.to_euler(EulerRot::XYZ);
                                    urdf_rs::Vec3([rot.0.into(), rot.1.into(), rot.2.into()])
                                }
                                
                            },
                            parent: urdf_rs::LinkName { link: link_name.clone() },
                            child: urdf_rs::LinkName { link: joint.reciever.clone() },
                            axis: urdf_rs::Axis { 
                                xyz:  {
                                    let x = joint.limit_axes.contains(JointAxesMask::ANG_X) as u32 as f64;
                                    let y = joint.limit_axes.contains(JointAxesMask::ANG_Y) as u32 as f64;
                                    let z = joint.limit_axes.contains(JointAxesMask::ANG_Z) as u32 as f64;
                                    urdf_rs::Vec3([x, y, z])
                                }
                            },
                            limit: urdf_rs::JointLimit {
                                lower: joint.limit.lower,
                                upper: joint.limit.upper,
                                //(TODO) implement this properly
                                effort: 99999999999.0,
                                //(TODO) implement this properly
                                velocity: 999999999999.0
                            },
                            //(TODO) implement this properly
                            dynamics: None,
                            //(TODO) implement this properly
                            mimic: None,
                            //(TODO) implement this properly
                            safety_controller: None
                            
                    
                        }
                    )
                }
                None => {}                
            }
        }
        urdf_map
    }
}

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