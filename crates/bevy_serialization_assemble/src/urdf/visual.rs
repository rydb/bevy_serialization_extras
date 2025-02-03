
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

use crate::{components::{RequestAssetStructure, Resolve}, gltf::{GltfMeshPrimitiveOne, GltfMeshWrapper, GltfNodeWrapper, GltfPrimitiveWrapper}, traits::{FromStructure, Split, Structure}};



#[derive(From, Clone)]
pub struct VisualWrapper(pub Vec<Visual>);

impl FromStructure for VisualWrapper {
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