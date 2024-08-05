//use bevy::prelude::*;
use nalgebra::{Matrix3, Vector3};
//use urdf_rs::Visual;
use crate::traits::*;

use bevy_render::mesh::VertexAttributeValues::Float32x3;
use bevy_ecs::prelude::*;
use bevy_reflect::{prelude::*, TypeRegistration, GetTypeRegistration};
use bevy_render::prelude::*;
use bevy_utils::prelude::*;
use bevy_math::prelude::*;

#[derive(Component, Default, Reflect, Clone)]
#[reflect(Component)]
pub struct GeometryFlag {
    pub primitive: MeshPrimitive,
    // matrix to "flip" the shape by. Not every format expresses orientation the same as bevy, so their positions/transforms are multiplied by their factor
    // (here) to match bevy's orientation.
    pub orientation_matrix: [Vec3; 3],
}
const IDENTITY_MATRIX: [Vec3; 3] = [
    Vec3::new(1.0, 0.0, 0.0),
    Vec3::new(0.0, 1.0, 0.0),
    Vec3::new(0.0, 0.0, 1.0),
];

#[derive(Default, Component, Reflect, Clone)]
#[reflect(Component)]
pub struct GeometryFile {
    pub source: String,
}

// impl ManagedTypeRegistration for GeometryFlag {
//     fn get_all_type_registrations() -> Vec<TypeRegistration> {
//         let mut type_registry = Vec::new();

//         type_registry.push(Self::get_type_registration());

//         type_registry
//         // for enum_variant in Self::iter() {
//         //     match enum_variant {
//         //         Self::Primitive(..) => type_registry.push(MeshPrimitive::get_type_registration()),
//         //         //Self::Mesh {..} => {}
//         //     }
//         // }
//         // return type_registry
//     }
// }

#[derive(Debug, Clone, PartialEq, Reflect, Copy, Component)]
pub enum MeshPrimitive {
    Cuboid { size: [f32; 3] },
    Cylinder { radius: f32, length: f32 },
    Capsule { radius: f32, length: f32 },
    Sphere { radius: f32 },
}

impl Default for MeshPrimitive {
    fn default() -> Self {
        Self::Cuboid {
            size: [1.0, 1.0, 1.0],
        }
    }
}

impl From<Cuboid> for GeometryFlag {
    fn from(value: Cuboid) -> Self {
        Self {
            primitive: MeshPrimitive::Cuboid {
                size: value.size().into(),
            },
            orientation_matrix: IDENTITY_MATRIX,
        }
    }
}

// impl From<Plane3d> for GeometryFlag {
//     fn from(value: Plane3d) -> Self {
//         Self {
//             primitive: MeshPrimitive::Cuboid {
//                 size: [value.size, 1.0, value.size],
//             },
//             orientation_matrix: IDENTITY_MATRIX,
//         }
//     }
// }

impl Unwrap<&GeometryFlag> for Mesh {
    fn unwrap(value: &GeometryFlag) -> Result<Self, String> {
        let i = value.orientation_matrix;
        let rotation_matrix = Matrix3::new(
            i[0].x, i[0].y, i[0].z, i[1].x, i[1].y, i[1].z, i[2].x, i[2].y, i[2].z,
        );

        match value.primitive {
            MeshPrimitive::Cuboid { size } => {
                let mut mesh = Mesh::from(Cuboid {
                    half_size: Vec3::new(size[0] * 0.5, size[1] * 0.5, size[2] * 0.5),
                });

                // rotate mesh vertices to account for differnt file formats orienting directions differently.
                //TODO: Fix normals on rotated mesh.
                if let Some(topology) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
                    match topology {
                        Float32x3(vertex_list) => {
                            for vertex in vertex_list {
                            let triangle = Vector3::new(vertex[0], vertex[1], vertex[2]);

                            let new_triangle = rotation_matrix * triangle;
                                vertex[0] = new_triangle[0];
                                vertex[1] = new_triangle[1];
                                vertex[2] = new_triangle[2];
                                 
                            }
                        }
                        _ => panic!("{:#?}, is not a support mesh attr type for maping mesh vertex visualizaton tug positions.", topology)
                    }
                }
                Ok(mesh)
            }
            MeshPrimitive::Cylinder { radius, length } => {
                //Ok(shape::Cylinder{radius: radius, height: length, ..default()}.into())
                let mut mesh = Mesh::from(Cylinder {
                    radius: radius,
                    half_height: length * 0.5,
                    ..default()
                });
                // let urdf_rotation_flip = Matrix3::new(
                //     -1.0, 0.0, 0.0,
                //     0.0, 0.0, 1.0,
                //     0.0, 1.0, 0.0,
                // );

                if let Some(topology) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
                    match topology {
                        Float32x3(vertex_list) => {
                            for vertex in vertex_list {
                            let triangle = Vector3::new(vertex[0], vertex[1], vertex[2]);

                            let new_triangle = rotation_matrix * triangle;
                                vertex[0] = new_triangle[0];
                                vertex[1] = new_triangle[1];
                                vertex[2] = new_triangle[2];
                                 
                            }
                        }
                        _ => panic!("{:#?}, is not a support mesh attr type for maping mesh vertex visualizaton tug positions.", topology)
                    }
                }
                Ok(mesh)
            }
            MeshPrimitive::Capsule { radius, length } => Ok(Capsule3d {
                radius: radius,
                half_length: length * 0.5,
                ..default()
            }
            .into()),
            MeshPrimitive::Sphere { radius } => Ok(Capsule3d {
                radius: radius,
                //depth: 0.0,
                ..default()
            }
            .into()),
        }
    }
}

impl Unwrap<&GeometryFile> for Mesh {
    fn unwrap(value: &GeometryFile) -> Result<Self, String> {
        return Err(value.source.clone());
    }
}

// impl ManagedTypeRegistration for GeometryFile {
//     fn get_all_type_registrations() -> Vec<TypeRegistration> {
//         let type_registry = Vec::new();

//         type_registry
//     }
// }
