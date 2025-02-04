

use bevy_core::Name;
use bevy_log::warn;
use bevy_math::Dir3;
use bevy_render::mesh::Mesh3d;
use bevy_render::prelude::InheritedVisibility;
use bevy_render::prelude::Visibility;
use bevy_serialization_core::prelude::{material::MaterialFlag3d, mesh::MeshFlag3d};
use bevy_serialization_physics::prelude::{
    colliders::ColliderFlag,
    continous_collision::CcdFlag,
    link::{
        Dynamics, GeometryShiftMarked, JointAxesMaskWrapper, JointFlag, JointLimitWrapper,
        JointMotorWrapper, LinkFlag, StructureFlag,
    },
    mass::MassFlag,
    rigidbodies::RigidBodyFlag,
    solvergroupfilter::{GroupWrapper, SolverGroupsFlag},
};
use bevy_transform::components::Transform;
use bevy_utils::prelude::default;
use glam::Mat3A;
use glam::Vec3A;
use glam::{EulerRot, Quat, Vec3};
use nalgebra::Matrix3;
use nalgebra::Vector3;
// use nalgebra::{Matrix3, Vector3};
use std::any::Any;
use std::any::TypeId;
use std::collections::HashMap;
use std::f32::consts::PI;
use urdf_rs::{Collision, Joint, Link, Pose, Robot, Visual};
use visual::{GeometryWrapper, VisualWrapper};

use derive_more::From;

use bevy_ecs::{prelude::*, query::QueryData};

use crate::components::Ids;
use crate::components::SplitOff;
use crate::traits::Split;
use crate::{
    components::{Maybe, RequestStructure, Resolve, RollDown}, traits::{FromStructure, IntoHashMap, Structure}
};

use super::*;

// pub trait CordinateSystem {
//     type Handedness = Handed
// }

// pub enum Handed {
//     LEFT,
//     RIGHT,
// }

// pub enum CordUp {

// }

/// the collection of things that qualify as a "link", in the ROS 2 context.
#[derive(QueryData)]
pub struct LinkQuery {
    pub name: Option<&'static Name>,
    pub structure: &'static StructureFlag,
    pub inertial: Option<&'static MassFlag>,
    //pub visual: FileCheck<GeometryFlag, GeometryFile>,
    pub visual: &'static MeshFlag3d,
    pub collision: Option<&'static ColliderFlag>,
    pub joint: Option<&'static JointFlag>,
}

// impl LazyDeserialize for Urdf {
//     fn deserialize(absolute_path: String, world: &World) -> Result<Self, LoadError> {
//         let res = urdf_rs::read_file(absolute_path);
//         let urdf = match res {
//             Ok(urdf) => urdf,
//             Err(err) => return Err(LoadError::Error(err.to_string())),
//         };
//         Ok(Urdf { robot: urdf })
//     }
// }

/// Links + Joints merged together.
/// URDF spec has these two as seperate, but joints are merged into the same entities/are dependent on links,
/// so they are merged here.
#[derive(Clone)]
pub struct LinksNJoints(Vec<(Link, Option<Joint>)>);

impl FromStructure for LinksNJoints {
    fn components(value: Self) -> Structure<impl Bundle> {
        let mut children = Vec::new();

        for (link, joint) in value.0 {
            let joint = joint
            .map(|n| 
                //RollDown(
                    RequestStructure(UrdfJoint(n)),
                    //vec![TypeId::of::<RigidBodyFlag>()]
                //)
            );
            children.push(
                (
                    Name::new(link.name),
                    //TODO: for sanity, refactor will not include this on initial release.
                    //RequestStructure(VisualWrapper(link.visual)),
                    RequestStructure(LinkColliders(link.collision)),
                    Maybe(joint),
                    //Transform::default(),
                    Visibility::default(),
                )
            )
        }
        Structure::Children(children, Split(true))
    }
}

#[derive(Clone)]
pub struct Visuals(pub Vec<Visual>);

#[derive(Clone)]
pub struct UrdfJoint(Joint);

impl FromStructure for UrdfJoint {
    fn components(value: Self)
    -> Structure<impl Bundle> {
        let axis = value.0.axis.xyz.0.map(|n| n as f32 );
        let axis = Vec3A::new(axis[0], axis[1], axis[2]);

        let rotation_matrix = Mat3A {
            x_axis: Vec3A::new(0.0, 0.0, 1.0),
            y_axis: Vec3A::new(0.0, 1.0, 0.0),
            z_axis: Vec3A::new(1.0, 0.0, 0.0),
        };

        // not sure how to do vec3 * matrix3 in glam, soooo doing it like this instead. 
        let rot_partx = Vec3A::new(rotation_matrix.x_axis.x, rotation_matrix.y_axis.x, rotation_matrix.z_axis.x);
        let rot_party = Vec3A::new(rotation_matrix.x_axis.y, rotation_matrix.y_axis.y, rotation_matrix.z_axis.y);
        let rot_partz = Vec3A::new(rotation_matrix.x_axis.z, rotation_matrix.y_axis.z, rotation_matrix.z_axis.z);

        let rotx = axis * rot_partx;
        let roty = axis * rot_party;
        let rotz = axis * rot_partz;
        let rot = rotx + roty + rotz;
        // let urdf_axis_matrix = Mat3A {
        //     x_axis: Vec3A::new(axis[0], 0.0, 0.0),
        //     y_axis: Vec3A::new(0.0, axis[1], 0.0),
        //     z_axis: Vec3A::new(0.0, 0.0, axis[2]),
        // };
        
        //println!("urdf axis matrix is {:#?}", urdf_axis_matrix);


        //let new_transform_matrix = urdf_axis_matrix * rotation_matrix;
        println!("new rot is: {:#?}", rot);
        let new_axis = Dir3::new_unchecked(rot.into());
        Structure::Root(

            (
                JointFlag::from(&JointWrapper(value.0.clone())),
                // Transform::from(UrdfTransform(value.0.origin))
                // .rotate_axis(new_axis, PI)
                // ,
                //Transform::from_rotation(Quat::from_axis_angle(rot.into(), PI/2.0))
            )
        )    
    }
}

// const MESHKINDS = [
//     TypeId::of::<MeshFlag3d>(),
    
//     ]

#[derive(Clone)]
pub struct LinkColliders(pub Vec<Collision>);

impl FromStructure for LinkColliders {
    fn components(value: Self) -> Structure<impl Bundle> {
        //let trans = Transform::from_rotation(Quat::from_rotation_x(PI/2.0));
        let geometry =  {
            if value.0.len() > 1 {
                warn!("multi-collider robots not supported as multi-primitive physics joints not supported(as of this version) in either Rapier or Avian");
                None
            } else {
                value.0.first()
                .map(|n| 
                    {
                        //println!("transform for {:#?} is {:#?}", n.name, trans);
                        n.geometry.clone()
                    }
                )
                .map(|n| Resolve::from(GeometryWrapper(n)))
            }
        };
        Structure::Root(
            (
                //trans,
                ColliderFlag::Convex,
                RigidBodyFlag::Dynamic,
                Maybe(geometry),
                Visibility::default()
            )
        )
    }
    // fn components(value: Self) -> Structure<impl Bundle> {
    //     let mut children = Vec::new();
    //     for collider in value.0 {
    //         children.push(
    //             (
    //                 ColliderFlag::Convex,
    //                 RigidBodyFlag::Dynamic,
    //                 Resolve::from(GeometryWrapper(collider.geometry)),
    //                 Name::new("collider"),
    //                 Transform::default(),
    //                 Visibility::default(),
    //             )
    //         );
    //     }
    //     Structure::Children(children, Split(false))
    // }
}






impl FromStructure for UrdfWrapper {
    fn components(value: Self) -> Structure<impl Bundle> {
        //let robot = value.robot;

        // let mut structured_link_map = HashMap::new();
        let mut structured_joint_map = HashMap::new();
        // let mut structured_material_map = HashMap::new();

        for joint in &value.0.joints {
            structured_joint_map.insert(joint.child.link.clone(), joint.clone());
        }
        // for material in &robot.materials {
        //     structured_material_map.insert(material.name.clone(), material.clone());
        // }
        // for link in &robot.links {
        //     structured_link_map.insert(link.name.clone(), link.clone());
        // }

        // let mut structured_entities_map: HashMap<String, Entity> = HashMap::new();
        let mut linkage = Vec::new();
        for link in value.0.0.links {
            linkage.push((
                link.clone(),
                structured_joint_map
                .get_key_value(&link.name).map(|(_, joint)| joint.clone())
            ))
        }
        Structure::Root(
            (
                Name::new(value.0.0.name),
                RequestStructure(LinksNJoints(linkage)),
                //Transform::default()
            ),
        )
        //FIXME: urdf meshes have their verticies re-oriented to match bevy's cordinate system, but their rotation isn't rotated back
        // to account for this, this will need a proper fix later.
        //temp_rotate_for_demo.rotate_x(-PI * 0.5);

        
        // for (_, link) in structured_link_map.iter() {
        //     children.push(
        //         (
        //             //FromStructure::components(link.visual.clone()),
        //             LinkVisuals(link.visual.clone()),
        //             LinkColliders(link.collision.clone())
        //         )
        //     )
        //     // let e = *structured_entities_map
        //     //     .entry(link.name.clone())
        //     //     .or_insert(commands.spawn_empty().id());

        //     // commands.entity(e)
        //     // .insert(Name::new(link.name.clone()))
        //     // //.insert(LinkFlag::from(&link.clone().into()))
        //     // .insert(StructureFlag { name: robot.name.clone() })
        //     // //.insert(MassFlag { mass: link.inertial.mass.value as f32})
        //     // ;
        //     // // if let Some(visual) = link.visual.first() {
        //     // //     let visual_wrapper = VisualWrapper::from(visual.clone());

        //     // //     let mesh = MeshFlag3d::from(&visual_wrapper);
        //     // //     commands.entity(e).insert(mesh);

        //     // //     commands
        //     // //         .entity(e)
        //     // //         .insert(MaterialFlag3d::from(&visual_wrapper));
        //     // // }

        //     // //FromStructure::into_entities(commands, e, link.visual.clone());
            
        //     // //let mut temp_rotate_for_demo = spawn_request.position;


        //     // commands
        //     //     .entity(e)
        //     //     .insert(Visibility::default())
        //     //     .insert(Transform::default())
        //     //     .insert(RigidBodyFlag::Dynamic)
        //     //     .insert(CcdFlag::default());
        // }

        // for (_, joint) in structured_joint_map.iter() {
        //     let e = *structured_entities_map
        //         .entry(joint.child.link.clone())
        //         .or_insert(commands.spawn_empty().id());

        //     //log::info!("spawning joint on {:#?}", e);
        //     let new_joint = JointFlag::from(&JointWrapper::from(joint.clone()));

        //     commands
        //         .entity(e)
        //         .insert(new_joint)
        //         .insert(RigidBodyFlag::Dynamic);
        // }
        //Ok(())
    }
}

impl IntoHashMap<Query<'_, '_, LinkQuery>> for UrdfWrapper {
    fn into_hashmap(value: Query<'_, '_, LinkQuery>, _world: &World) -> HashMap<String, Self> {
        let mut urdf_map = HashMap::new();
        for link in value.iter() {
            let structure_name = link.structure.name.clone();
            let entry = urdf_map.entry(structure_name.clone()).or_insert(UrdfWrapper (
                Urdf(Robot {
                    name: link.structure.name.clone(),
                    links: Vec::new(),
                    joints: Vec::new(),
                    materials: Vec::new(),
                }),
            ));

            match link.joint {
                Some(joint) => {
                    let link_name = link
                        .name
                        .unwrap_or(&Name::new(entry.0.joints.len().to_string()))
                        .to_string();
                    let joint_name = link_name.clone() + "_joint";
                    let joint_parent = joint.parent_name.clone().unwrap_or_default();
                    //let urdf_link_name = link_name + "_link";
                    entry.0.0.joints.push(Joint {
                        name: joint_name,
                        //FIXME:  implement this properly have this be a consequence of joint data via a function. This is a placeholder.
                        joint_type: urdf_rs::JointType::Continuous,
                        origin: Pose {
                            xyz: urdf_rs::Vec3([
                                joint.local_frame1.translation.x.into(),
                                joint.local_frame1.translation.y.into(),
                                joint.local_frame1.translation.z.into(),
                            ]),
                            rpy: {
                                let rot = joint.local_frame1.rotation.to_euler(EulerRot::XYZ);
                                urdf_rs::Vec3([rot.0.into(), rot.1.into(), rot.2.into()])
                            },
                        },
                        parent: urdf_rs::LinkName {
                            link: joint_parent.clone(),
                        },
                        child: urdf_rs::LinkName {
                            link: link_name.clone(),
                        },
                        axis: urdf_rs::Axis {
                            xyz: {
                                let x = joint.limit_axes.contains(JointAxesMaskWrapper::ANG_X)
                                    as u32 as f64;
                                let y = joint.limit_axes.contains(JointAxesMaskWrapper::ANG_Y)
                                    as u32 as f64;
                                let z = joint.limit_axes.contains(JointAxesMaskWrapper::ANG_Z)
                                    as u32 as f64;
                                urdf_rs::Vec3([x, y, z])
                            },
                        },
                        limit: urdf_rs::JointLimit {
                            lower: joint.limit.lower,
                            upper: joint.limit.upper,
                            //FIXME: implement this properly
                            effort: f64::MAX,
                            //FIXME: implement this properly
                            velocity: f64::MAX,
                        },
                        //FIXME: implement this properly
                        dynamics: None,
                        //FIXME: implement this properly
                        mimic: None,
                        //FIXME: implement this properly
                        safety_controller: None,
                    })
                }
                None => {}
            }
        }
        urdf_map
    }
}
#[derive(From)]
pub struct LinkWrapper(Link);

impl From<&LinkWrapper> for LinkFlag {
    fn from(value: &LinkWrapper) -> Self {
        let visual = value
            .0
            .visual
            .first()
            .unwrap_or(&Visual::default())
            .to_owned();
        Self {
            //FIXME: implement this properly to account for urdfs with multiple visual elements
            geom_offset: Vec3::from_array([
                visual.origin.xyz[0] as f32,
                visual.origin.xyz[1] as f32,
                visual.origin.xyz[2] as f32,
            ]),
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
            0.0, 1.0, 0.0
        );
        // based on this explanation
        //https://stackoverflow.com/questions/31191752/right-handed-euler-angles-xyz-to-left-handed-euler-angles-xyz
        let urdf_rotation_flip = Matrix3::new(
            -1.0, 0.0, 0.0, 
            0.0, -1.0, 0.0, 
            0.0, 0.0, 1.0
        );
        let pos = value.0;

        let compliant_trans =
            urdf_cord_flip * Vector3::new(pos.xyz.0[0], pos.xyz.0[1], pos.xyz.0[2]);
        let compliant_rot =
            urdf_rotation_flip * Vector3::new(pos.rpy.0[0], pos.rpy.0[1], pos.rpy.0[2]);

        Self {
            translation: Vec3::new(
                compliant_trans.x as f32,
                compliant_trans.y as f32,
                compliant_trans.z as f32,
            ),
            rotation: Quat::from_euler(
                EulerRot::XYZ,
                compliant_rot.x as f32,
                compliant_rot.y as f32,
                compliant_rot.z as f32,
            ),
            ..default()
        }
    }
}

#[derive(From)]
pub struct JointWrapper(Joint);

//FIXME: get rid of defaults as needed
impl From<&JointWrapper> for JointFlag {
    fn from(value: &JointWrapper) -> Self {
        //let joint_offset = Transform::from(UrdfTransform::from(value.0.origin.clone()));
        //let axis = value.0.axis.xyz.0.clone();

        //let new_axis = JointAxesMaskWrapper
        let motor_settings = JointMotorWrapper {
            max_force: f32::MAX,
            //FIXME: I pulled this number from rapier joint damping. This should be generated based on something like air-resistance at some point to be
            //accurate.
            damping: 20.0,
            ..default()
        };

        Self {
            parent_name: Some(value.0.parent.link.clone()),
            parent_id: None,
            limit: JointLimitWrapper {
                //FIXME: Workaround for urdfs defaulting to un-movable joints
                //https://github.com/openrr/urdf-rs/issues/99
                lower: -999999999999999.0, //f64::MIN, //value.0.limit.lower,
                upper: 999999999999999.0,  //f64::MAX, //value.0.limit.upper,
                effort: value.0.limit.effort,
                velocity: value.0.limit.velocity,
            },
            dynamics: {
                match value.0.dynamics.clone() {
                    Some(dynamics) => Dynamics {
                        damping: dynamics.damping,
                        friction: dynamics.friction,
                    },
                    None => Dynamics::default(),
                }
            },
            local_frame1: UrdfTransform::from(value.0.origin.clone()).into(),
            local_frame2: Transform::default(),
            locked_axes: {
                //clamp axis to between 0-1 for simplicity and for bitmask flipping
                let default_locked_axes = JointAxesMaskWrapper::LOCKED_FIXED_AXES;
                if value.0.joint_type != urdf_rs::JointType::Fixed {
                    let unit_axis = value
                        .0
                        .axis
                        .xyz
                        .0
                        .map(|n| n.abs().clamp(0.0, 1.0))
                        .map(|n| n as u8);

                    //println!("unit axis is !!! {:#?}", unit_axis);
                    let mut x = default_locked_axes.bits();
                    x ^= unit_axis[0] << 3;
                    x ^= unit_axis[1] << 4;
                    x ^= unit_axis[2] << 5;
                    JointAxesMaskWrapper::from_bits(x).unwrap()
                } else {
                    default_locked_axes
                }

                //FIXME: Replace with proper "axis-alignment" metod for converting from urdf -> bevy
                //JointAxesMaskWrapper::LOCKED_FIXED_AXES.difference(JointAxesMaskWrapper::ANG_Y)
            },
            limit_axes: JointAxesMaskWrapper::empty(),
            motor_axes: JointAxesMaskWrapper::all(),
            coupled_axes: JointAxesMaskWrapper::empty(),
            contacts_enabled: true,
            enabled: true,
            motors: [
                motor_settings.clone(),
                motor_settings.clone(),
                motor_settings.clone(),
                motor_settings.clone(),
                motor_settings.clone(),
                motor_settings.clone(),
            ],
        }
    }
}
