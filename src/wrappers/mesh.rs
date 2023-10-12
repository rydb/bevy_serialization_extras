
use bevy::prelude::*;
use bevy::reflect::GetTypeRegistration;
use crate::traits::{Unwrap, ManagedTypeRegistration};
use crate::wrappers::mesh::shape::Cube;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use bevy::render::mesh::shape::Plane;



#[derive(Component, Reflect, Clone, EnumIter)]
#[reflect(Component)]
pub enum GeometryFlag{
    Primitive(MeshPrimitive),
    Mesh {
        filename: String,
        scale: Option<Vec3>,
    },
}

impl ManagedTypeRegistration for GeometryFlag {
    fn get_all_type_registrations() -> Vec<bevy::reflect::TypeRegistration> {
        let mut type_registry = Vec::new();

        type_registry.push(Self::get_type_registration());
        
        for enum_variant in Self::iter() {
            match enum_variant {
                Self::Primitive(..) => type_registry.push(MeshPrimitive::get_type_registration()),
                Self::Mesh {..} => {}
            }
        }
        return type_registry
        
    }
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

#[derive(Debug, Clone, PartialEq, Reflect,  Copy)]
#[derive(Component)]
pub enum MeshPrimitive {
    Box { size: [f32; 3] },
    Cylinder { radius: f32, length: f32 },
    Capsule { radius: f32, length: f32 },
    Sphere { radius: f32 },
}

impl Default for MeshPrimitive {
    fn default() -> Self {
        Self::Box { size: [1.0, 1.0, 1.0] }
    }
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

// impl From<&urdf_rs::Geometry> for GeometryFlag {
//     fn from(geom: &urdf_rs::Geometry) -> Self {
//         match geom {
//             urdf_rs::Geometry::Box { size } => GeometryFlag::Primitive(MeshPrimitive::Box {
//                 size: (**size).map(|f| f as f32),
//             }),
//             urdf_rs::Geometry::Cylinder { radius, length } => {
//                 GeometryFlag::Primitive(MeshPrimitive::Cylinder {
//                     radius: *radius as f32,
//                     length: *length as f32,
//                 })
//             }
//             urdf_rs::Geometry::Capsule { radius, length } => {
//                 GeometryFlag::Primitive(MeshPrimitive::Capsule {
//                     radius: *radius as f32,
//                     length: *length as f32,
//                 })
//             }
//             urdf_rs::Geometry::Sphere { radius } => GeometryFlag::Primitive(MeshPrimitive::Sphere {
//                 radius: *radius as f32,
//             }),
//             urdf_rs::Geometry::Mesh { filename, scale } => {
//                 //println!("filename for mesh is {:#?}", filename);
//                 let scale = scale
//                     .clone()
//                     .and_then(|s| Some(Vec3::from_array(s.map(|v| v as f32))));
//                 GeometryFlag::Mesh {
//                     filename: filename.clone(),
//                     scale,
//                 }
//             }
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

impl From<Plane> for GeometryFlag {
    fn from(value: Plane) -> Self {
        return GeometryFlag::Primitive(
            MeshPrimitive::Box { size: [value.size, 1.0, value.size]} 
        )
    }
}

impl From<&str> for GeometryFlag {
    fn from(value: &str) -> Self {
        Self::Mesh {
            filename: value.to_string(),
            scale: None,
        }
    }
}



impl Unwrap<&GeometryFlag> for Mesh {
    fn unwrap(value: &GeometryFlag) -> Result<Self, String>{
        match value {
            GeometryFlag::Primitive(primitive) => {
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
            GeometryFlag::Mesh { filename, .. } => Err(filename.to_string())
        }
    }
}
