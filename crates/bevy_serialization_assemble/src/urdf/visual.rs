
use bevy_ecs::{component::StorageType, prelude::*};
use bevy_color::{Color, LinearRgba};
use bevy_gltf::GltfNode;
use bevy_hierarchy::BuildChildren;
use bevy_serialization_core::prelude::{
    material::{MaterialFlag3d, MaterialWrapper},
    mesh::{MeshFlag3d, MeshPrefab},
};
use bevy_log::warn;
use bevy_serialization_physics::prelude::{ColliderFlag, GroupWrapper, MassFlag, SolverGroupsFlag};
use derive_more::From;
use glam::Vec3;
use nalgebra::Vector3;
use urdf_rs::Visual;

use bevy_math::prelude::*;

use crate::{components::RequestAssetStructure, gltf::GltfNodeWrapper, traits::{FromStructure, Structure}};

#[derive(Clone)]
pub enum Resolve<T, U> {
    One(T),
    Other(U)
}

impl<T: Component + Clone, U: Component + Clone> Component for Resolve<T, U> {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    fn register_component_hooks(_hooks: &mut bevy_ecs::component::ComponentHooks) {
        _hooks.on_add(|mut world, e, comp| {
            let comp = {
                match world.entity(e).get::<Self>() {
                    Some(val) => val.clone(),
                    None => {
                        warn!("could not get resolve on: {:#}", e);
                        return
                    },
                }
            };
            match comp {
                Resolve::One(one) => {world.commands().entity(e).insert(one);},
                Resolve::Other(other) => {world.commands().entity(e).insert(other);},
            }
            world.commands().entity(e).remove::<Self>();
        });
    }
}

#[derive(From, Clone)]
pub struct VisualWrapper(pub Vec<Visual>);

impl FromStructure for VisualWrapper {
    fn components(value: Self) -> Structure<impl Bundle> {
        let mut children = Vec::new();
                
        for visual in value.0 {
        (
            // ColliderFlag::default(),
            // SolverGroupsFlag {
            //     memberships: GroupWrapper::GROUP_1,
            //     filters: GroupWrapper::GROUP_2,
            // },
            // MassFlag {mass: 1.0},
            children.push( match visual.geometry {
                urdf_rs::Geometry::Box { size } => {
                    let bevy_size = /*urdf_rotation_flip * */ Vector3::new(size[0], size[1], size[2]);
                    Resolve::One(MeshFlag3d::Prefab(MeshPrefab::Cuboid(Cuboid {
                        half_size: Vec3::new(
                            bevy_size[0] as f32,
                            bevy_size[1] as f32,
                            bevy_size[2] as f32,
                        ),
                    })))
                }
                urdf_rs::Geometry::Cylinder { radius, length } => {
                    //TODO: double check that this is correct
                    Resolve::One(MeshFlag3d::Prefab(MeshPrefab::Cylinder(Cylinder {
                        radius: radius as f32,
                        half_height: length as f32,
                    })))
                }
                urdf_rs::Geometry::Capsule { radius, length } => {
                    //TODO: double check that this is correct
                    Resolve::One(MeshFlag3d::Prefab(MeshPrefab::Capsule(Capsule3d {
                        radius: radius as f32,
                        half_length: length as f32,
                    })))
                }
                urdf_rs::Geometry::Sphere { radius } => Resolve::One(MeshFlag3d::Prefab(MeshPrefab::Sphere(Sphere {
                    radius: radius as f32,
                }))),
                urdf_rs::Geometry::Mesh { filename, .. } => {
                    Resolve::Other(
                        RequestAssetStructure::<GltfNodeWrapper>::Path(filename)
                    )
                    //Err(filename)
                    // commands.entity(child).insert(
                    //     GltfNodeSpawnRequest(filename)
                    // );
                    //Self::AssetPath(filename.to_owned()),

                }
            }
        ));

                                

            // let flag = match &
            // match flag {
            //     Ok(flag) => {commands.entity(child).insert(flag);},
            //     Err(file) => {commands.entity(root).insert(GltfNodeSpawnRequest(crate::gltf::Request::Path(file.to_owned())));},
            // }
            // commands.entity(root).add_child(child);
        }
        Structure::Children(children)
    }
}

// impl From<&VisualWrapper> for MaterialFlag3d {
//     fn from(value: &VisualWrapper) -> Self {
//         if let Some(material) = &value.0.material {
//             if let Some(color) = &material.color {
//                 let rgba = color.rgba.0;
//                 Self::Wrapper(MaterialWrapper {
//                     color: Color::LinearRgba(LinearRgba {
//                         red: rgba[0] as f32,
//                         green: rgba[1] as f32,
//                         blue: rgba[2] as f32,
//                         alpha: rgba[3] as f32,
//                     }),
//                 })
//             } else {
//                 Self::default()
//             }
//         } else {
//             Self::default()
//         }
//     }
// }

// impl From<&VisualWrapper> for MeshFlag3d {
//     fn from(value: &VisualWrapper) -> Self {
//         let visual = &value.0;

//         let urdf_geometry = &visual.geometry;

//         let flag_geometry = match urdf_geometry {
//             urdf_rs::Geometry::Box { size } => {
//                 let bevy_size = /*urdf_rotation_flip * */ Vector3::new(size[0], size[1], size[2]);
//                 Self::Prefab(MeshPrefab::Cuboid(Cuboid {
//                     half_size: Vec3::new(
//                         bevy_size[0] as f32,
//                         bevy_size[1] as f32,
//                         bevy_size[2] as f32,
//                     ),
//                 }))
//             }
//             urdf_rs::Geometry::Cylinder { radius, length } => {
//                 //TODO: double check that this is correct
//                 Self::Prefab(MeshPrefab::Cylinder(Cylinder {
//                     radius: *radius as f32,
//                     half_height: *length as f32,
//                 }))
//             }
//             urdf_rs::Geometry::Capsule { radius, length } => {
//                 //TODO: double check that this is correct
//                 Self::Prefab(MeshPrefab::Capsule(Capsule3d {
//                     radius: *radius as f32,
//                     half_length: *length as f32,
//                 }))
//             }
//             urdf_rs::Geometry::Sphere { radius } => Self::Prefab(MeshPrefab::Sphere(Sphere {
//                 radius: *radius as f32,
//             })),
//             urdf_rs::Geometry::Mesh { filename, .. } => Self::AssetPath(filename.to_owned()),
//         };
//         flag_geometry
//     }
// }
