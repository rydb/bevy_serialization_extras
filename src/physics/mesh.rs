use bevy::{prelude::*, asset::AssetPath};
use crate::traits::ECSLoad;

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

impl ECSLoad<Mesh> for GeometryFlag {
    fn load_from(value: &Self, mut things: ResMut<Assets<Mesh>>, mut asset_server: AssetServer) -> Handle<Mesh>{
        match value {
            Self::Primitive(primitive) => {
                match primitive {
                    MeshPrimitive::Box { size } => {
                        things.add(shape::Box{
                            min_x: -size[0] * 0.5,
                            max_x: size[0] * 0.5,
                            min_y: -size[1] * 0.5,
                            max_y: size[1] * 0.5,
                            min_z: -size[2] * 0.5,
                            max_z: size[2] * 0.5,
                        }.into())
                    }
                    MeshPrimitive::Cylinder { radius, length } => {
                        things.add(shape::Cylinder{radius: *radius, height: *length, ..default()}.into())
                    },
                    MeshPrimitive::Capsule { radius, length } => {
                        things.add(shape::Capsule{radius: *radius, depth: *length, ..default()}.into())
                    },
                    MeshPrimitive::Sphere { radius } => {
                        things.add(shape::Capsule{radius: *radius, depth: 0.0, ..default()}.into())
                    },
                }
            } 
            Self::Mesh { filename, scale } => asset_server.load(filename)
        }
    }
}