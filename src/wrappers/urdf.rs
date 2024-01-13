
use std::collections::HashMap;

// use bevy::core::Name;
use bevy::{prelude::*, utils::thiserror};
use bevy_rapier3d::geometry::Group;
use urdf_rs::{Robot, Joint, Pose, UrdfError, Link};

use crate::{queries::FileCheckPicker, resources::AssetSpawnRequest, loaders::urdf_loader::Urdf, traits::{LazyDeserialize, LoadError}, wrappers::link::LinkFlag};

use super::{material::MaterialFlag, link::{JointFlag, LinkQuery, JointAxesMaskWrapper, StructureFlag}, mass::MassFlag, colliders::ColliderFlag, rigidbodies::RigidBodyFlag, continous_collision::CcdFlag, solvergroupfilter::SolverGroupsFlag, collisiongroupfilter::CollisionGroupsFlag};
use bevy::render::mesh::VertexAttributeValues::Float32x3;




impl LazyDeserialize for Urdf {
    fn deserialize(absolute_path: String) -> Result<Self, LoadError>{
        let urdf = urdf_rs::read_file(absolute_path)?;
            Ok(Urdf {robot: urdf })
    }
}


pub struct UrdfLinkage<'a, 'b> {
    link: &'a Link,
    joint: Option<&'b Joint>, 
}


impl<'a> FromStructure for Urdf {
    fn into_entities(commands: &mut Commands, value: Self, spawn_request: AssetSpawnRequest<Self>){
        //let name = request.item.clone();
        //let robot = value.world_urdfs.get(&request.item).unwrap();
        //log::info!("urdf is {:#?}", value.clone());

        let robot = value.robot;

        let mut structured_link_map = HashMap::new();
        let mut structured_joint_map = HashMap::new();
        let mut structured_material_map = HashMap::new();

        for joint in &robot.joints {
            structured_joint_map.insert(joint.child.link.clone(), joint.clone());
        }
        for material in &robot.materials {
            structured_material_map.insert(material.name.clone(), material.clone());
        }
        for link in &robot.links {
            structured_link_map.insert(link.name.clone(), link.clone());
        }
        
        // structured_linkage_map.insert(UrdfLinkage {
        //     link:
        // })
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
        let mut structured_entities_map: HashMap<String, Entity> = HashMap::new();


       
        for (key , link) in structured_link_map.iter() {
            let e = *structured_entities_map.entry(link.name.clone())
            .or_insert(commands.spawn_empty().id());


            commands.entity(e)
            .insert(Name::new(link.name.clone()))
            .insert(LinkFlag::from(link))
            .insert(StructureFlag { name: robot.name.clone() })
            .insert(MassFlag {mass: 1.0})
            //.insert(MassFlag { mass: link.inertial.mass.value as f32})
            ;
            match FileCheckPicker::from(link.visual.clone()){
                    FileCheckPicker::PureComponent(t) => commands.entity(e).insert(t),
                    FileCheckPicker::PathComponent(u) => commands.entity(e).insert(u),
                };
            commands.entity(e)
            .insert(MaterialFlag::from(link.visual.clone()))
            .insert(VisibilityBundle::default())
            .insert(TransformBundle {
                local: spawn_request.position, 
                ..default()
            })
            .insert(ColliderFlag::default())
            .insert(SolverGroupsFlag {
                memberships: Group::GROUP_1,
                filters: Group::GROUP_2,
            })
            // .insert(CollisionGroupsFlag {
            //     memberships: Group::NONE,
            //     filters: Group::NONE,
            // })
            .insert(GeometryShiftMarked::default())
            .insert(RigidBodyFlag::Fixed)
            //.insert(CcdFlag::default())
            //.insert()
            ;
        }

        for (key, joint) in structured_joint_map.iter() {
            let e = *structured_entities_map.entry(joint.child.link.clone())
            .or_insert(commands.spawn_empty().id());

            log::info!("spawning joint on {:#?}", e);
            let new_joint = JointFlag::from(joint);

            commands.entity(e)
            .insert(new_joint)
            .insert(RigidBodyFlag::Dynamic)
            ;
            // for link in structured_link_map.values().filter(|link| link.name == joint.parent.link) {
            //     let new_joint = JointFlag::from(joint);

            //     commands.entity(e)
            //     .insert(new_joint)
            //     .insert(RigidBodyFlag::Dynamic)
            //     ;
            // }

        }
    }
}

pub trait FromStructure
    where
        Self: Sized + Asset
{
    fn into_entities(commands: &mut Commands, value: Self, spawn_request: AssetSpawnRequest<Self>);
}

pub trait IntoHashMap<T>
where
    Self: Sized
{
    fn into_hashmap(value: T) -> HashMap<String, Self>;
}
// impl Deserializer for Urdf {
// }

impl IntoHashMap<Query<'_, '_, LinkQuery>> for Urdf {
    fn into_hashmap(value: Query<'_, '_, LinkQuery>) -> HashMap<String, Self> {
        let mut urdf_map = HashMap::new();
        for link in value.iter() {
            let structure_name = link.structure.name.clone();
            let entry = urdf_map.entry(structure_name.clone())
            .or_insert(
                Urdf {
                    robot: Robot { name: link.structure.name.clone(), links: Vec::new(), joints: Vec::new(), materials: Vec::new() }
                }
            )
            ;
            
            match link.joint {
                Some(joint) => {
                    let link_name = link.name.unwrap_or(&Name::new(entry.robot.joints.len().to_string())).to_string();
                    let joint_name = link_name.clone() + "_joint";
                    let joint_parent = joint.parent_name.clone().unwrap_or_default();
                    //let urdf_link_name = link_name + "_link";
                    entry.robot.joints.push
                    (
                        Joint 
                        {
                            name: joint_name,
                            //(TODO) implement this properly have this be a consequence of joint data via a function. This is a placeholder.
                            joint_type: urdf_rs::JointType::Continuous,
                            origin: Pose {
                                xyz: urdf_rs::Vec3([joint.local_frame1.translation.x.into(), joint.local_frame1.translation.y.into(), joint.local_frame1.translation.z.into()]),
                                rpy: {
                                    let rot = joint.local_frame1.rotation.to_euler(EulerRot::XYZ);
                                    urdf_rs::Vec3([rot.0.into(), rot.1.into(), rot.2.into()])
                                }
                                
                            },
                            parent: urdf_rs::LinkName { link: joint_parent.clone() },
                            child: urdf_rs::LinkName { link: link_name.clone() },
                            axis: urdf_rs::Axis { 
                                xyz:  {
                                    let x = joint.limit_axes.contains(JointAxesMaskWrapper::ANG_X) as u32 as f64;
                                    let y = joint.limit_axes.contains(JointAxesMaskWrapper::ANG_Y) as u32 as f64;
                                    let z = joint.limit_axes.contains(JointAxesMaskWrapper::ANG_Z) as u32 as f64;
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

#[derive(Component, Default)]
pub struct JointBounded;

// get joints and bind them to their named connection if it exists
pub fn bind_joints_to_entities(
    mut joints: Query<(Entity, &mut JointFlag), (Without<JointBounded>)>,
    link_names: Query<(Entity, &Name)>,
    mut commands: Commands,
) {
    for (joint_e, mut joint) in joints.iter_mut() {
        let joint_parent_name = joint.parent_name.clone();
        match joint_parent_name {
            Some(name) => {
                for (i, (e, link_name)) in link_names.iter()
                .filter(|(e, link_name)| name == link_name.to_string())
                .enumerate() {
                    if i > 0 {
                        panic!("more then 1 entity with joint name that this joint can bind to! to prevent undefined behaviour, erroring here!")
                    }
                    if joint.parent_id != Some(e) {
                        joint.parent_id = Some(e);
                        commands.entity(joint_e).insert(JointBounded::default());
                    }

                }
            }
            None => {}
        }
    }
}


pub fn urdf_origin_shift(
    mut unshifted_models: Query<(Entity, &LinkFlag, &mut JointFlag), (Without<GeometryShifted>, With<GeometryShiftMarked>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands
) {
    for (e, link_flag, mut joint_flag) in unshifted_models.iter_mut() {
        println!("shifting model local_frame 2");
        // match joint_flag.local_frame2 {
        //     Some(local_frame2) => {
                
        //     }
        // }
        //joint_flag.local_frame1.translation += link_flag.geom_offset;
        joint_flag.local_frame2 = Some(Transform::from_translation(-link_flag.geom_offset));
        commands.entity(e).insert(GeometryShifted::default());
    }
}

#[derive(Component, Clone, Copy, Default)]
pub struct GeometryShiftMarked;

/// flags entity geometry as already shifted to account for urdf origin
#[derive(Component, Clone, Copy, Default)]
pub struct GeometryShifted;

// take a model's vertices, and shift them by urdf offset
// 
// urdfs shift origin of the geometry it self, so to make "geometry" origins match their urdfs, the geometry it self must be shifted
// pub fn urdf_origin_shift(
//     unshifted_models: Query<(Entity, &LinkFlag, &Handle<Mesh>), (Without<GeometryShifted>, With<GeometryShiftMarked>)>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut commands: Commands
// ) {
//     for (e, link_flag, mesh_check) in unshifted_models.iter() {
//         if let Some(mesh) = meshes.get_mut(mesh_check) {
//             if let Some(topology) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
//                 match topology {
//                     Float32x3(vertex_list) => {
//                         let (x, y, z) = (link_flag.geom_offset.x, link_flag.geom_offset.y, link_flag.geom_offset.z);
//                         println!("geom offset is {:#?}", link_flag.geom_offset);
//                         for vertex in vertex_list {
//                              vertex[0] -= x.clone();
//                              vertex[1] -= z.clone();
//                              vertex[2] -= y.clone();
                             
//                         }
//                         commands.entity(e).insert(GeometryShifted::default());
//                     }
//                     _ => panic!("{:#?}, is not a support mesh attr type for maping mesh vertex visualizaton tug positions.", topology)
//                 }
//             }
//         }
//     }
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