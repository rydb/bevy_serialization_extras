
use std::{collections::{HashMap, VecDeque}, ffi::IntoStringError, rc::Rc, borrow::Cow};

// use bevy::core::Name;
use bevy::{prelude::*, ecs::{query::WorldQuery, system::EntityCommands}, utils::hashbrown::hash_map::IntoIter, transform::commands};
use egui::Link;
use urdf_rs::{Robot, Joint, Pose};

use crate::{traits::Structure, queries::{FileCheck, FileCheckItem}, resources::{LoadRequest, AssetSpawnRequest}, loaders::urdf_loader::Urdf};

use super::{mesh::{GeometryFlag, GeometryFile, GeometrySource}, material::{MaterialFlag, MaterialFile, MaterialSource}, link::{JointFlag, LinkQuery, JointAxesMask, LinkageItem, LinkQueryItem, StructureFlag}, mass::MassFlag, colliders::ColliderFlag};


// use super::{material::MaterialFlag, link::LinkFlag, joint::JointFlag};
// // pub struct LinkWrapper {}


// #[derive(WorldQuery, Clone)]
// pub struct UrdfQuery {
//     pub link: &'static LinkFlag,
//     pub visual: FileCheck<GeometryFlag, GeometryFile>,
//     pub joint_check: Option<&'static JointFlag>,
//     pub material_check: Option<FileCheck<MaterialFlag, MaterialFile>>,
// }


// pub struct UrdfQuery {

// }

//ReflectComponent

#[derive(Default, Resource, Clone)]
pub struct Urdfs {
    pub world_urdfs: HashMap<String, Robot>
}

impl<'a> FromStructure for Urdf {
    fn into_structures(commands: &mut Commands, value: Self, spawn_request: AssetSpawnRequest<Self>){
        //let name = request.item.clone();
        //let robot = value.world_urdfs.get(&request.item).unwrap();
        let robot = value.robot;

        let mut structured_link_map = HashMap::new();
        let mut structured_joint_map = HashMap::new();
        let mut structured_material_map = HashMap::new();
        for link in &robot.links {
            structured_link_map.insert(link.name.clone(), link.clone());
        }
        for joint in &robot.joints {
            structured_joint_map.insert(joint.parent.link.clone(), joint.clone());
        }
        for material in &robot.materials {
            structured_material_map.insert(material.name.clone(), material.clone());
        }
        // let query_items =structured_link_map.iter().map(|(key, link)| 
        //     {
        //         LinkQueryItem {
        //             name: Some(&Name::new(link.name.clone())),
        //             structure: &StructureFlag { name: value.name.clone() },
        //             inertial: Some(&MassFlag { mass: link.inertial.mass.value as f32}), 
        //             // implement visual properly
        //             visual: FileCheckItem {component: &GeometryFlag::default(), component_file: None}, 
        //             // implement collision properly. Grouped colliders will need to be ignored for the sake of model coherence.
        //             collision: Some(&ColliderFlag::default()), 
        //             // implement joint loading properly..
        //             joint: Some(&JointFlag::default()) }
        //     }
        // ).collect::<Vec<Self>>();
        for (key , link) in structured_link_map.iter() {
            let mut e = commands.spawn_empty();

            e
            .insert(Name::new(link.name.clone()))
            .insert(StructureFlag { name: robot.name.clone() })
            .insert(MassFlag { mass: link.inertial.mass.value as f32})
            .insert(GeometryFlag::default())
            .insert(ColliderFlag::default())
            .insert(JointFlag::default())
            .insert(MaterialFlag::default())
            .insert(VisibilityBundle::default())
            .insert(TransformBundle {
                local: spawn_request.position, 
                ..default()
            })
            ;
        }
    }
}

// impl FromStructure for Urdfs {
//     fn into_structures(commands: &mut Commands, value: Self, mut resload_request: VecDeque<ResLoadRequest>) -> VecDeque<ResLoadRequest> {
//         //let mut my_resload_requests = resload_request;
//         //println!("robots are {:#?}", value.world_urdfs);
//         //println!("load requests are {:#?}", resload_request.);
//         while resload_request.len() != 0{
//             if let Some(request) = resload_request.pop_front() {
//                 //println!("loading request for {:#?}", request.item.clone());
//                 if value.world_urdfs.contains_key(&request.item) {
//                     //let name = request.item.clone();
//                     let robot = value.world_urdfs.get(&request.item).unwrap();


//                     let mut structured_link_map = HashMap::new();
//                     let mut structured_joint_map = HashMap::new();
//                     let mut structured_material_map = HashMap::new();
//                     for link in &robot.links {
//                         structured_link_map.insert(link.name.clone(), link.clone());
//                     }
//                     for joint in &robot.joints {
//                         structured_joint_map.insert(joint.parent.link.clone(), joint.clone());
//                     }
//                     for material in &robot.materials {
//                         structured_material_map.insert(material.name.clone(), material.clone());
//                     }
//                     // let query_items =structured_link_map.iter().map(|(key, link)| 
//                     //     {
//                     //         LinkQueryItem {
//                     //             name: Some(&Name::new(link.name.clone())),
//                     //             structure: &StructureFlag { name: value.name.clone() },
//                     //             inertial: Some(&MassFlag { mass: link.inertial.mass.value as f32}), 
//                     //             // implement visual properly
//                     //             visual: FileCheckItem {component: &GeometryFlag::default(), component_file: None}, 
//                     //             // implement collision properly. Grouped colliders will need to be ignored for the sake of model coherence.
//                     //             collision: Some(&ColliderFlag::default()), 
//                     //             // implement joint loading properly..
//                     //             joint: Some(&JointFlag::default()) }
//                     //     }
//                     // ).collect::<Vec<Self>>();
//                     for (key , link) in structured_link_map.iter() {
//                         let mut e = commands.spawn_empty();
            
//                         e
//                         .insert(Name::new(link.name.clone()))
//                         .insert(StructureFlag { name: robot.name.clone() })
//                         .insert(MassFlag { mass: link.inertial.mass.value as f32})
//                         .insert(GeometryFlag::default())
//                         .insert(ColliderFlag::default())
//                         .insert(JointFlag::default())
//                         .insert(MaterialFlag::default())
//                         .insert(VisibilityBundle::default())
//                         .insert(TransformBundle {
//                             local: request.position, 
//                             ..default()
//                         })
//                         ;
//                     }
//                 }
//                 // for (name, robot) in value.world_urdfs.iter() {

//                 // }
//             }

//         }
//         return resload_request


//     }
// }


// impl<'a> IntoIterator for LinkQueryItem<'a> {
//     type Item = EntityCommands;
//     fn into_iter(self) -> Self::IntoIter {
        
//     }
// }

// impl<'a> ComponentIter for LinkQueryItem<'a> {
//     fn spawn_iter(commands: Commands) {
//         commands.spawn()
//     }
// }




pub trait ComponentIter {
    fn spawn_iter(commands: Commands);
}

pub trait FromStructure
    where
        Self: Sized + Asset
{
    fn into_structures(commands: &mut Commands, value: Self, spawn_request: AssetSpawnRequest<Self>);
}

// impl<'a> IntoIterator for &'a Robot {
//     type Item = LinkQueryItem<'a>;
//     type IntoIter = std::vec::IntoIter<Self::Item>;
//     fn into_iter(self) -> Self::IntoIter {
        
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

// impl From<Query<'_, '_, LinkQuery>> for HashMap<String, Urdf> {

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