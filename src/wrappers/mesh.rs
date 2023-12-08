
use bevy::prelude::*;
use bevy::reflect::GetTypeRegistration;
use urdf_rs::Visual;
use crate::queries::{FileCheckItem, FileCheckPicker};
use crate::traits::{Unwrap, ManagedTypeRegistration};
use crate::wrappers::mesh::shape::Cube;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use bevy::render::mesh::shape::Plane;



#[derive(Component, Default, Reflect, Clone)]
#[reflect(Component)]
pub struct GeometryFlag{
    primitive: MeshPrimitive,
    // Mesh {
    //     filename: String,
    //     scale: Option<Vec3>,
    // },
}

#[derive(Component, Reflect, Clone, EnumIter)]
#[reflect(Component)]
pub enum GeometrySource {
    Primitive(GeometryFlag),
    File(GeometryFile),
}

impl Default for GeometrySource {
    fn default() -> Self {
        Self::Primitive(GeometryFlag::default())
    }
}


#[derive(Default, Component, Reflect, Clone)]
#[reflect(Component)]
pub struct GeometryFile {
    pub path: String
}

impl ManagedTypeRegistration for GeometryFlag {
    fn get_all_type_registrations() -> Vec<bevy::reflect::TypeRegistration> {
        let mut type_registry = Vec::new();

        type_registry.push(Self::get_type_registration());

        type_registry
        // for enum_variant in Self::iter() {
        //     match enum_variant {
        //         Self::Primitive(..) => type_registry.push(MeshPrimitive::get_type_registration()),
        //         //Self::Mesh {..} => {}
        //     }
        // }
        // return type_registry
        
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
        Self {
            primitive: MeshPrimitive::Box { size: [value.size, value.size, value.size] }
        }
        // return GeometryFlag::Primitive(
        //     MeshPrimitive::Box { size: [value.size, value.size, value.size] }
        // )
    }
}

impl From<Plane> for GeometryFlag {
    fn from(value: Plane) -> Self {
        Self {
            primitive: MeshPrimitive::Box { size: [value.size, 1.0, value.size]}
        }
        // return GeometryFlag::Primitive(
        //     MeshPrimitive::Box { size: [value.size, 1.0, value.size]} 
        // )
    }
}

// impl From<&str> for GeometryFlag {
//     fn from(value: &str) -> Self {
//         Self::Mesh {
//             filename: value.to_string(),
//             scale: None,
//         }
//     }
// }


// impl From<Vec<Visual>> for FileCheckItem<'_, GeometryFlag, GeometryFile> {
//     fn from(value: Vec<Visual>) -> Self {
        
//     }
// }
// impl From<&urdf_rs::Geometry> for GeometryFlag {
//     fn from(geom: &urdf_rs::Geometry) -> Self {
//         match geom {
//             urdf_rs::Geometry::Box { size } => 
//             GeometryFlag { primitive:  MeshPrimitive::Box {
//                 size: (**size).map(|f| f as f32),
//             }},            
//             urdf_rs::Geometry::Cylinder { radius, length } => {
//                 GeometryFlag {primitive: MeshPrimitive::Cylinder {
//                     radius: *radius as f32,
//                     length: *length as f32,
//                 }}
//             },
//             urdf_rs::Geometry::Capsule { radius, length } => {
//                 GeometryFlag {primitive: MeshPrimitive::Capsule {
//                     radius: *radius as f32,
//                     length: *length as f32,
//                 }}
//             }
//             urdf_rs::Geometry::Sphere { radius } => GeometryFlag {primitive: MeshPrimitive::Sphere {
//                 radius: *radius as f32,
//             }},
//             urdf_rs::Geometry::Mesh { filename, scale } => {
//                 GeometryFile {
//                     path: filename.clone(),
//                 }
//             }
//         }
//     }
// }

impl From<Vec<Visual>> for FileCheckPicker<GeometryFlag, GeometryFile> {
    fn from(value: Vec<Visual>) -> Self {
        if let Some(visual) = value.first() {
            let urdf_geometry = &visual.geometry;

            let flag_geometry = match urdf_geometry {
                urdf_rs::Geometry::Box { size } => FileCheckPicker::PureComponent(
                    GeometryFlag { 
                        primitive:  MeshPrimitive::Box {
                            size: (*size).map(|f| f as f32),
                        }
                    }
                ),
                urdf_rs::Geometry::Cylinder { radius, length } => FileCheckPicker::PureComponent(
                    GeometryFlag {
                        primitive: MeshPrimitive::Cylinder {
                            radius: *radius as f32,
                            length: *length as f32,
                        }
                    }
                ),
                urdf_rs::Geometry::Capsule { radius, length } => FileCheckPicker::PureComponent(
                    GeometryFlag {
                        primitive: MeshPrimitive::Capsule {
                            radius: *radius as f32,
                            length: *length as f32,
                        }
                    }
                ),
                urdf_rs::Geometry::Sphere { radius } => FileCheckPicker::PureComponent(
                    GeometryFlag {
                        primitive: MeshPrimitive::Sphere {
                            radius: *radius as f32,
                        }
                    }
                ),
                urdf_rs::Geometry::Mesh { filename, .. } => FileCheckPicker::PathComponent(
                    GeometryFile {
                        path: filename.clone(),
                    }
                )
            };
            return flag_geometry
        } else {
            Self::default()
        }
    }
}

// impl<'a> From<Vec<Visual>> for FileCheckItem<'a, GeometryFlag, GeometryFile> {
//     fn from(value: Vec<Visual>) -> Self {
//         // take only the first one for expidency
//         if let Some(visual) = value.first() {
//             let urdf_geometry = visual.geometry;

//             let flag_geometry = match urdf_geometry {
//                 urdf_rs::Geometry::Box { size } => 
//                     FileCheckItem {
//                         component: &GeometryFlag { 
//                             primitive:  MeshPrimitive::Box {
//                                 size: (*size).map(|f| f as f32),
//                             }
//                         },
//                         component_file: None

//                     },            
//                 urdf_rs::Geometry::Cylinder { radius, length } => 
//                     FileCheckItem {
//                         component: &GeometryFlag {
//                             primitive: MeshPrimitive::Cylinder {
//                                 radius: radius as f32,
//                                 length: length as f32,
//                         }},
//                         component_file: None
//                     }
//                 ,
//                 urdf_rs::Geometry::Capsule { radius, length } => {
//                     FileCheckItem {
//                         component: &GeometryFlag {primitive: MeshPrimitive::Capsule {
//                             radius: radius as f32,
//                             length: length as f32,
//                         }},
//                         component_file: None
//                     }

//                 },
//                 urdf_rs::Geometry::Sphere { radius } => 
//                     FileCheckItem {
//                         component: &GeometryFlag {
//                             primitive: MeshPrimitive::Sphere {
//                                 radius: radius as f32,
//                             }
//                         }, 
//                         component_file: None
//                     }
//                 ,
//                 urdf_rs::Geometry::Mesh { filename, scale } => 
//                     FileCheckItem { 
//                         component: &GeometryFlag::default(),
//                         component_file: Some(&GeometryFile {
//                             path: filename.clone(),
//                         })
//                     }

                
//             };
//             flag_geometry
//             } else {
//                 FileCheckItem {
//                     component: &GeometryFlag::default(),
//                     component_file: None
//                 }
//             }
//         }
//     }


impl Unwrap<&GeometryFlag> for Mesh {
    fn unwrap(value: &GeometryFlag) -> Result<Self, String>{
        match value.primitive {
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
                Ok(shape::Cylinder{radius: radius, height: length, ..default()}.into())
            },
            MeshPrimitive::Capsule { radius, length } => {
                Ok(shape::Capsule{radius: radius, depth: length, ..default()}.into())
            },
            MeshPrimitive::Sphere { radius } => {
                Ok(shape::Capsule{radius: radius, depth: 0.0, ..default()}.into())
            },
        }

    }
}
