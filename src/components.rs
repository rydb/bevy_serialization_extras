use bevy::prelude::*;
use crate::physics::mesh::GeometryFlag;
use crate::physics::material::MaterialFlag;
/// Wrapper bundle made to tie together everything that composes a "model", in a serializable format
/// !!! THIS WILL LIKELY BE REFACTORED AWAY WITH ASSETSV2 IN 0.12!!!
#[derive(Bundle, Default)]
pub struct ModelBundle {
    pub mesh: GeometryFlag,
    pub material: MaterialFlag,
    pub visibility: Visibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,

}

// /// Reflect, and Serialization both require a default implementation of structs. The default GeometryFlag resorts to an "fallback" mesh to
// /// represent failed load attempts. (TODO): add a system that picks up error meshes, and displays them somewhere.
// impl Default for GeometryFlag {
//     fn default() -> Self {
//         Self::Mesh {
//             filename: "fallback.gltf".to_string(),
//             scale: None,
//         }        
//     }
// }

// impl From<Cube> for GeometryFlag {
//     fn from(value: Cube) -> Self {
//         return GeometryFlag::Primitive(
//             MeshPrimitive::Box { size: [value.size, value.size, value.size] }
//         )
//     }
// }

// impl From<Plane> for GeometryFlag {
//     fn from(value: Plane) -> Self {
//         return GeometryFlag::Primitive(
//             MeshPrimitive::Box { size: [value.size, 1.0, value.size]} 
//         )
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

// impl From<&str> for GeometryFlag {
//     fn from(value: &str) -> Self {
//         Self::Mesh {
//             filename: value.to_string(),
//             scale: None,
//         }
//     }
// }

