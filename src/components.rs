use bevy::{prelude::*};
use bevy::render::mesh::shape::*;


/// Component which marks entity as target for serialization/deserialization
// #[derive(Component, Default, Reflect, Clone)]
// #[reflect(Component)]
// pub struct Serializable;
/// Component which save/load systems pick up on to tell them "what" a thing is for serialization purposes. 
/// 
/// if serialize enum  is set to [`Skip`], then that particular entity is skipped. during the save/load process
/// ['Skip'] is generally applied to entities apart of nested structures
// #[derive(Component, Default, Reflect, Clone)]
// #[reflect(Component)]
// pub enum SerializeType {
//     #[default]
//     Skip,
//     SingleModel,
//     Urdf(String),
// }


/// Wrapper bundle made to tie together everything that composes a "model", in a serializable format
/// !!! THIS WILL LIKELY BE REFACTORED AWAY WITH ASSETSV2 IN 0.12!!!
#[derive(Bundle)]
pub struct ModelBundle {
    pub model: ModelFlag,
    pub transform: Transform
}


/// The type of physics an entity should be serialized with, this is set to dynamic by default
#[derive(Component, Reflect, Clone, Default)]
#[reflect(Component)]
pub enum Physics {
    #[default]
    Dynamic,
    Fixed,
}
/// component which flags entity as a model for spawning purposes. !!!TREAT THIS AS READ ONLY!!!
/// (TODO) reimplement this to 
#[derive(Component, Reflect, Clone)]
//#[reflect(from_reflect = false)]
#[reflect(Component)]
pub struct ModelFlag {
    pub geometry: Geometry,
    pub material: StandardMaterial,
    pub physics: Physics
    //pub thing_type: Transform, 

}

impl Default for ModelFlag {
    fn default() -> Self {
        Self {
            geometry: Default::default(),
            material: Default::default(),
            physics: Default::default(),
            //transform: Default::default()
        }
    }
}

/// geometry type. Should only be set once and left unedited. 
#[derive(Component, Reflect, Clone)]
//#[reflect(from_reflect = false)]
#[reflect(Component)]
pub enum Geometry{
    Primitive(MeshPrimitive),
    Mesh {
        filename: String,
        scale: Option<Vec3>,
    },
}

/// Reflect, and Serialization both require a default implementation of structs. The default GeometryFlag resorts to an "fallback" mesh to
/// represent failed load attempts. (TODO): add a system that picks up error meshes, and displays them somewhere.
impl Default for Geometry {
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

impl From<Cube> for Geometry {
    fn from(value: Cube) -> Self {
        return Geometry::Primitive(
            MeshPrimitive::Box { size: [value.size, value.size, value.size] }
        )
    }
}

impl From<Plane> for Geometry {
    fn from(value: Plane) -> Self {
        return Geometry::Primitive(
            MeshPrimitive::Box { size: [value.size, 1.0, value.size]} 
        )
    }
}

// impl Into<Mesh> for Geometry {
//     fn into(self) -> Mesh {
//         match self {
//             Self::Primitive(variant)  => variant.into(),
//             Mesh => return shape::Cube{size: 1.0}.into(),
//         }
//     }
// }

// impl Into<Option<Mesh>> for MeshPrimitive {
//     fn into(self) -> Option<Mesh> {
//         return Option
//     }
// }

impl Into<Mesh> for MeshPrimitive {
    fn into(self) -> Mesh {
        match self {
            Self::Box { size } => 
                shape::Box{
                    min_x: -size[0] * 0.5,
                    max_x: size[0] * 0.5,
                    min_y: -size[1] * 0.5,
                    max_y: size[1] * 0.5,
                    min_z: -size[2] * 0.5,
                    max_z: size[2] * 0.5,
                }.into(),
            Self::Cylinder { radius, length } => shape::Cylinder{radius: radius, height: length, ..default()}.into(),
            Self::Capsule { radius, length } => shape::Capsule{radius: radius, depth: length, ..default()}.into(),
            Self::Sphere { radius } => shape::Capsule{radius: radius, depth: 0.0, ..default()}.into(),
        }
    }
}

/// flags that entity uses a urdf for model loading. The model load system should pick this model up 
// #[derive(Component, Reflect, Clone)]
// #[reflect(Component)]
// pub struct ModelFlagUrdf {
//     pub urdf_name: String
// }
// impl Default for ModelFlagUrdf {
//     fn default() -> Self {
//         Self {
//             urdf_name: "fallback.xml".to_string(),
            
//         }
//     }
// }

impl From<&urdf_rs::Geometry> for Geometry {
    fn from(geom: &urdf_rs::Geometry) -> Self {
        match geom {
            urdf_rs::Geometry::Box { size } => Geometry::Primitive(MeshPrimitive::Box {
                size: (**size).map(|f| f as f32),
            }),
            urdf_rs::Geometry::Cylinder { radius, length } => {
                Geometry::Primitive(MeshPrimitive::Cylinder {
                    radius: *radius as f32,
                    length: *length as f32,
                })
            }
            urdf_rs::Geometry::Capsule { radius, length } => {
                Geometry::Primitive(MeshPrimitive::Capsule {
                    radius: *radius as f32,
                    length: *length as f32,
                })
            }
            urdf_rs::Geometry::Sphere { radius } => Geometry::Primitive(MeshPrimitive::Sphere {
                radius: *radius as f32,
            }),
            urdf_rs::Geometry::Mesh { filename, scale } => {
                //println!("filename for mesh is {:#?}", filename);
                let scale = scale
                    .clone()
                    .and_then(|s| Some(Vec3::from_array(s.map(|v| v as f32))));
                Geometry::Mesh {
                    filename: filename.clone(),
                    scale,
                }
            }
        }
    }
}

impl From<&str> for Geometry {
    fn from(value: &str) -> Self {
        Self::Mesh {
            filename: value.to_string(),
            scale: None,
        }
    }
}

