use bevy::{prelude::*, asset::AssetPath};
use crate::traits::ECSLoad;
use crate::physics::mesh::shape::Cube;

#[derive(Component, Reflect, Clone)]
//#[reflect(from_reflect = false)]
#[reflect(Component)]
pub enum GeometryFlag{
    Primitive(MeshPrimitive),
    Mesh {
        filename: String,
        scale: Option<Vec3>,
    },
}

/// Reflect, and Serialization both require a default implementation of structs. The default GeometryFlag resorts to an "fallback" mesh to
/// represent failed load attempts. (TODO): add a system that picks up error meshes, and displays them somewhere.
impl Default for GeometryFlag {
    fn default() -> Self {
        Self::Mesh {
            filename: "fallback.gltf".to_string(),
            scale: None,
        }        
    }
}

#[derive(Debug, Clone, PartialEq, Reflect, Copy)]
#[derive(Component)]
pub enum MeshPrimitive {
    Box { size: [f32; 3] },
    Cylinder { radius: f32, length: f32 },
    Capsule { radius: f32, length: f32 },
    Sphere { radius: f32 },
}

// impl Into<Mesh> for MeshPrimitive {
//     fn into(self) -> Mesh {
//         match self {
//             Self::Box { size } => 
//                 shape::Box{
//                     min_x: -size[0] * 0.5,
//                     max_x: size[0] * 0.5,
//                     min_y: -size[1] * 0.5,
//                     max_y: size[1] * 0.5,
//                     min_z: -size[2] * 0.5,
//                     max_z: size[2] * 0.5,
//                 }.into(),
//             Self::Cylinder { radius, length } => shape::Cylinder{radius: radius, height: length, ..default()}.into(),
//             Self::Capsule { radius, length } => shape::Capsule{radius: radius, depth: length, ..default()}.into(),
//             Self::Sphere { radius } => shape::Capsule{radius: radius, depth: 0.0, ..default()}.into(),
//         }
//     }
// }

impl From<Cube> for GeometryFlag {
    fn from(value: Cube) -> Self {
        return GeometryFlag::Primitive(
            MeshPrimitive::Box { size: [value.size, value.size, value.size] }
        )
    }
}

impl ECSLoad<Mesh> for GeometryFlag {
    fn deserialize_wrapper(value: &Self) -> Result<Mesh, String>{
        match value {
            Self::Primitive(primitive) => {
                match primitive {
                    MeshPrimitive::Box { size } => {
                        return Ok(shape::Box{
                            min_x: -size[0] * 0.5,
                            max_x: size[0] * 0.5,
                            min_y: -size[1] * 0.5,
                            max_y: size[1] * 0.5,
                            min_z: -size[2] * 0.5,
                            max_z: size[2] * 0.5,
                        }.into())
                    }
                    MeshPrimitive::Cylinder { radius, length } => {
                        Ok(shape::Cylinder{radius: *radius, height: *length, ..default()}.into())
                    },
                    MeshPrimitive::Capsule { radius, length } => {
                        Ok(shape::Capsule{radius: *radius, depth: *length, ..default()}.into())
                    },
                    MeshPrimitive::Sphere { radius } => {
                        Ok(shape::Capsule{radius: *radius, depth: 0.0, ..default()}.into())
                    },
                }
            } 
            Self::Mesh { filename, scale } => Err(filename.to_string())
        }
    }
}