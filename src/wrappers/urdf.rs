
use std::collections::HashMap;

// use bevy::core::Name;
use bevy::{prelude::*, utils::thiserror};
use urdf_rs::{Robot, Joint, Pose, UrdfError};

use crate::{queries::FileCheckPicker, resources::AssetSpawnRequest, loaders::urdf_loader::Urdf, traits::{LazyDeserialize, LoadError}, wrappers::link::LinkFlag};

use super::{material::MaterialFlag, link::{JointFlag, LinkQuery, JointAxesMaskWrapper, StructureFlag}, mass::MassFlag, colliders::ColliderFlag, rigidbodies::RigidBodyFlag, continous_collision::CcdFlag, solvergroupfilter::SolverGroupsFlag};
use bevy::render::mesh::VertexAttributeValues::Float32x3;




impl LazyDeserialize for Urdf {
    fn deserialize(absolute_path: String) -> Result<Self, LoadError>{
        let urdf = urdf_rs::read_file(absolute_path)?;
            Ok(Urdf {robot: urdf })
    }
}



impl<'a> FromStructure for Urdf {
    fn into_entities(commands: &mut Commands, value: Self, spawn_request: AssetSpawnRequest<Self>){
        //let name = request.item.clone();
        //let robot = value.world_urdfs.get(&request.item).unwrap();
        //println!("urdf is {:#?}", value.clone());

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
        let mut structured_entities_map: HashMap<String, Entity> = HashMap::new();

        for (key, joint) in structured_joint_map.iter() {
            let e = *structured_entities_map.entry(joint.child.link.clone())
            .or_insert(commands.spawn_empty().id());

            for link in structured_link_map.values().filter(|link| link.name == joint.parent.link) {
                //println!("found parent link for joint");
                let parent_e = *structured_entities_map.entry(link.name.clone())
                .or_insert(
                    commands.spawn_empty()
                    .insert(RigidBodyFlag::Fixed)
                    .id());

                
                let mut new_joint = JointFlag::from(joint);
                new_joint.parent_id = Some(parent_e);

                commands.entity(e)
                .insert(new_joint)
                .insert(RigidBodyFlag::Dynamic)

                ;
            }

        }

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
            .insert(SolverGroupsFlag::default())
            .insert(GeometryShiftMarked::default())
            //.insert(CcdFlag::default())
            //.insert()
            ;
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



pub fn urdf_origin_shift(
    mut unshifted_models: Query<(Entity, &LinkFlag, &mut JointFlag), (Without<GeometryShifted>, With<GeometryShiftMarked>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands
) {
    for (e, link_flag, mut joint_flag) in unshifted_models.iter_mut() {
        println!("shifting model local_frame 2");
        joint_flag.local_frame2.translation = link_flag.geom_offset;
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