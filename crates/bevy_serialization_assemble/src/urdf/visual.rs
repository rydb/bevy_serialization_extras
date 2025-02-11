
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
use urdf_rs::{Geometry, Visual};

use bevy_math::prelude::*;

use crate::{components::{RequestAssetStructure, Resolve}, gltf::{GltfMeshPrimitiveOne, GltfMeshWrapper, GltfNodeWrapper, GltfPrimitiveWrapper}, traits::{Disassemble, Split, Structure}};



#[derive(From, Clone)]
pub struct VisualWrapper(pub Vec<Visual>);

impl Disassemble for VisualWrapper {
    fn components(value: Self) -> Structure<impl Bundle> {
        let mut children = Vec::new();
        for visual in value.0 {
        (
            children.push( 
                Resolve::from(GeometryWrapper(visual.geometry))
            ));
        }
        Structure::Children(children, Split(false))
    }
}


pub struct GeometryWrapper(pub Geometry);

impl From<&MeshFlag3d> for GeometryWrapper {
    fn from(value: &MeshFlag3d) -> Self {
        match value {
            MeshFlag3d::AssetPath(path) => {
                let split = path.split("/Primitive").collect::<Vec<_>>();
                let mut path = path.to_owned();
                if split.len() > 1 {
                    warn!("until: https://github.com/bevyengine/bevy/issues/17661 is resolved, primitives must be loaded through meshes. Chopping off `Primitive` in mean time. for \n {:#}", path);
                    path = split.first().map(|n| n.to_string()).unwrap_or(path.to_owned());
                }
                Self(Geometry::Mesh { 
                    filename: path, 
                    //TODO: check if this is correct
                    scale: None 
                })
            },
            MeshFlag3d::Procedural(mesh_wrapper) => {
                warn!("procedural meshes not supported in urdf serialization(currently) defaulting to error mesh");
                Self(Geometry::Box { size: urdf_rs::Vec3([0.1, 0.1, 0.1]) })
            },
            MeshFlag3d::Prefab(mesh_prefab) => match mesh_prefab {
                MeshPrefab::Cuboid(cuboid) => Self(
                    Geometry::Box { size: urdf_rs::Vec3(cuboid.size().to_array().map(|n| n as f64)) }
                ),
                MeshPrefab::Cylinder(cylinder) => Self(
                    Geometry::Cylinder { radius: cylinder.radius as f64, length: (cylinder.half_height * 2.0) as f64 }
                ),
                MeshPrefab::Capsule(capsule3d) => Self(
                    Geometry::Capsule { radius: capsule3d.radius as f64, length: (capsule3d.half_length * 2.0) as f64 }
                ),
                MeshPrefab::Sphere(sphere) => Self(
                    Geometry::Sphere { radius: sphere.radius as f64 }
                ),
            },
        }
    }
}

impl From<GeometryWrapper> for Resolve<MeshFlag3d, RequestAssetStructure<GltfMeshPrimitiveOne>> {
    fn from(value: GeometryWrapper) -> Self {
        match value.0 {
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
                    RequestAssetStructure::<GltfMeshPrimitiveOne>::Path(filename)
                )

            }
        }
    }
}