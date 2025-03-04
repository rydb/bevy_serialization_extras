use bevy_derive::Deref;
use bevy_ecs::prelude::*;
use bevy_log::warn;
use bevy_serialization_core::prelude::mesh::{Mesh3dFlag, MeshPrefab, MeshWrapper};
use derive_more::From;
use glam::Vec3;
use nalgebra::Vector3;
use urdf_rs::{Geometry, Visual};

use bevy_math::prelude::*;

use crate::{
    components::{RequestAssetStructure, Resolve},
    gltf::GltfPhysicsMeshPrimitive,
    traits::{Disassemble, Split, Structure},
};

#[derive(From, Clone, Deref)]
pub struct VisualWrapper(pub Vec<Visual>);

impl Disassemble for VisualWrapper {
    fn components(value: Self) -> Structure<impl Bundle> {
        let mut children = Vec::new();
        for visual in value.0 {
            (children.push(Resolve::from(GeometryWrapper(visual.geometry))));
        }
        Structure::Children(children, Split(false))
    }
}

const FALLBACK_GEOMETRY: Geometry = Geometry::Box {
    size: urdf_rs::Vec3([0.1, 0.1, 0.1]),
};

pub struct GeometryWrapper(pub Geometry);

impl From<&Mesh3dFlag> for GeometryWrapper {
    fn from(value: &Mesh3dFlag) -> Self {
        match value {
            Mesh3dFlag::Path(path) => {
                let split = path.split("/Primitive").collect::<Vec<_>>();
                let mut path = path.to_owned();
                if split.len() > 1 {
                    warn!("until: https://github.com/bevyengine/bevy/issues/17661 is resolved, primitives must be loaded through meshes. Chopping off `Primitive` in mean time. for \n {:#}", path);
                    path = split
                        .first()
                        .map(|n| n.to_string())
                        .unwrap_or(path.to_owned());
                }
                Self(Geometry::Mesh {
                    filename: path,
                    //TODO: check if this is correct
                    scale: None,
                })
            }
            Mesh3dFlag::Pure(pure) => match pure {
                MeshWrapper::Procedural => {
                    warn!("procedural meshes not supported in urdf serialization(currently) defaulting to error mesh");
                    Self(FALLBACK_GEOMETRY)
                }
                MeshWrapper::Prefab(mesh_prefab) => match mesh_prefab {
                    MeshPrefab::Cuboid(cuboid) => Self(Geometry::Box {
                        size: urdf_rs::Vec3(cuboid.size().to_array().map(|n| n as f64)),
                    }),
                    MeshPrefab::Cylinder(cylinder) => Self(Geometry::Cylinder {
                        radius: cylinder.radius as f64,
                        length: (cylinder.half_height * 2.0) as f64,
                    }),
                    MeshPrefab::Capsule(capsule3d) => Self(Geometry::Capsule {
                        radius: capsule3d.radius as f64,
                        length: (capsule3d.half_length * 2.0) as f64,
                    }),
                    MeshPrefab::Sphere(sphere) => Self(Geometry::Sphere {
                        radius: sphere.radius as f64,
                    }),
                    MeshPrefab::Cone(_cone) => {
                        warn!("Cones not supported by urdf-rs. Using fallback primitive.");
                        Self(FALLBACK_GEOMETRY)
                    }
                    MeshPrefab::Unimplemented(mesh) => {
                        warn!(
                            "Unimplemented mesh prefab {:#?}. Using fallback primitive",
                            mesh
                        );
                        Self(FALLBACK_GEOMETRY)
                    }
                },
            },
        }
    }
}

impl From<GeometryWrapper>
    for Resolve<Mesh3dFlag, RequestAssetStructure<GltfPhysicsMeshPrimitive>>
{
    fn from(value: GeometryWrapper) -> Self {
        match value.0 {
            urdf_rs::Geometry::Box { size } => {
                let bevy_size = /*urdf_rotation_flip * */ Vector3::new(size[0], size[1], size[2]);
                Resolve::One(Mesh3dFlag::Pure(
                    MeshPrefab::Cuboid(Cuboid {
                        half_size: Vec3::new(
                            bevy_size[0] as f32,
                            bevy_size[1] as f32,
                            bevy_size[2] as f32,
                        ),
                    })
                    .into(),
                ))
            }
            urdf_rs::Geometry::Cylinder { radius, length } => {
                //TODO: double check that this is correct
                Resolve::One(Mesh3dFlag::Pure(
                    MeshPrefab::Cylinder(Cylinder {
                        radius: radius as f32,
                        half_height: length as f32,
                    })
                    .into(),
                ))
            }
            urdf_rs::Geometry::Capsule { radius, length } => {
                //TODO: double check that this is correct
                Resolve::One(Mesh3dFlag::Pure(
                    MeshPrefab::Capsule(Capsule3d {
                        radius: radius as f32,
                        half_length: length as f32,
                    })
                    .into(),
                ))
            }
            urdf_rs::Geometry::Sphere { radius } => Resolve::One(Mesh3dFlag::Pure(
                MeshPrefab::Sphere(Sphere {
                    radius: radius as f32,
                })
                .into(),
            )),
            urdf_rs::Geometry::Mesh { filename, .. } => {
                Resolve::Other(RequestAssetStructure::<GltfPhysicsMeshPrimitive>::Path(
                    filename,
                ))
            }
        }
    }
}
