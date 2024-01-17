
use bevy::prelude::*;
use bevy::reflect::GetTypeRegistration;
use nalgebra::{Matrix3, Vector3};
use urdf_rs::Visual;
use crate::asset_source::AssetSource;
use crate::queries::FileCheckPicker;
use crate::traits::{Unwrap, ManagedTypeRegistration};
use crate::wrappers::mesh::shape::Cube;
use strum_macros::EnumIter;
use bevy::render::mesh::shape::Plane;
use bevy::render::mesh::VertexAttributeValues::Float32x3;


#[derive(Component, Default, Reflect, Clone)]
#[reflect(Component)]
pub struct GeometryFlag{
    primitive: MeshPrimitive,
}

#[derive(Default, Component, Reflect, Clone)]
#[reflect(Component)]
pub struct GeometryFile {
    pub source: String
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


impl From<Cube> for GeometryFlag {
    fn from(value: Cube) -> Self {
        Self {
            primitive: MeshPrimitive::Box { size: [value.size, value.size, value.size] }
        }
    }
}

impl From<Plane> for GeometryFlag {
    fn from(value: Plane) -> Self {
        Self {
            primitive: MeshPrimitive::Box { size: [value.size, 1.0, value.size]}
        }
    }
}

impl From<Vec<Visual>> for FileCheckPicker<GeometryFlag, GeometryFile> {

    
    fn from(value: Vec<Visual>) -> Self {

        let urdf_rotation_flip = Matrix3::new(
            0.0, -1.0, 0.0,
            0.0, 0.0, 1.0,
            1.0, 0.0, 0.0,
        );

        if let Some(visual) = value.first() {

            let urdf_geometry = &visual.geometry;

            let flag_geometry = match urdf_geometry {
                urdf_rs::Geometry::Box { size } => { 

                    let bevy_size = /*urdf_rotation_flip * */ Vector3::new(size[0], size[1], size[2]);
                    FileCheckPicker::PureComponent(
                    
                        GeometryFlag { 

                            primitive:  MeshPrimitive::Box {
                                //size: (*size).map(|f| f as f32),
                                size: [bevy_size[0] as f32, bevy_size[1] as f32, bevy_size[2] as f32]
                            }
                        }
                    
                    )
                },
                urdf_rs::Geometry::Cylinder { radius, length } => 
                {
                    //let bevy_size = Vector3::new(radius, length, radius);
                    FileCheckPicker::PureComponent(
                        GeometryFlag {
                            primitive: MeshPrimitive::Cylinder {
                                radius: *radius as f32,
                                length: *length as f32,
                            }
                        }
                    )
                },
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
                urdf_rs::Geometry::Mesh { filename, .. } => {
                    let asset_source = AssetSource::Package(filename.clone());

                    let asset_path = parse_urdf_source(asset_source);
                    FileCheckPicker::PathComponent(
                    //AssetSource::Package(filename.clone());
                    GeometryFile {
                        //source: AssetSource::Package(filename.clone()),
                        source: asset_path
                    }
                )
                }
            };
            return flag_geometry
        } else {
            panic!("Multi-model links are not implemented.");
            //Self::default()
        }
    }
}

fn parse_urdf_source(source: AssetSource) -> String {

    match source {
        AssetSource::Package(pkg) => pkg.strip_prefix("package://").unwrap().to_owned(),
        AssetSource::Placeholder(..) => panic!("Asset source not implemented for this asset source.")
    }

}

impl Unwrap<&GeometryFlag> for Mesh {
    fn unwrap(value: &GeometryFlag) -> Result<Self, String>{
        match value.primitive {
            MeshPrimitive::Box { size } => {
                // return Ok(shape::Box{
                //     min_x: -size[0] * 0.5,
                //     max_x: size[0] * 0.5,
                //     min_y: -size[1] * 0.5,
                //     max_y: size[1] * 0.5,
                //     min_z: -size[2] * 0.5,
                //     max_z: size[2] * 0.5,
                // }.into())
                let mut mesh = Mesh::from(shape::Box{
                    min_x: -size[0] * 0.5,
                    max_x: size[0] * 0.5,
                    min_y: -size[1] * 0.5,
                    max_y: size[1] * 0.5,
                    min_z: -size[2] * 0.5,
                    max_z: size[2] * 0.5,
                });
                // let urdf_rotation_flip = Matrix3::new(
                //     -1.0, 0.0, 0.0,
                //     0.0, 0.0, 1.0,
                //     0.0, 1.0, 0.0,
                // );
                let urdf_rotation_flip = Matrix3::new(
                    0.0, 0.0, -1.0,
                    0.0, 1.0, 0.0,
                    1.0, 0.0, 0.0,
                );
    
                
                if let Some(topology) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
                    match topology {
                        Float32x3(vertex_list) => {
                            for vertex in vertex_list {
                            let triangle = Vector3::new(vertex[0], vertex[1], vertex[2]);

                            let new_triangle = urdf_rotation_flip * triangle;
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
                let mut mesh = Mesh::from(shape::Cylinder{radius: radius, height: length, ..default()});
                let urdf_rotation_flip = Matrix3::new(
                    -1.0, 0.0, 0.0,
                    0.0, 0.0, 1.0,
                    0.0, 1.0, 0.0,
                );
    
                
                if let Some(topology) = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
                    match topology {
                        Float32x3(vertex_list) => {
                            for vertex in vertex_list {
                            let triangle = Vector3::new(vertex[0], vertex[1], vertex[2]);

                            let new_triangle = urdf_rotation_flip * triangle;
                                vertex[0] = new_triangle[0];
                                vertex[1] = new_triangle[1];
                                vertex[2] = new_triangle[2];
                                 
                            }
                        }
                        _ => panic!("{:#?}, is not a support mesh attr type for maping mesh vertex visualizaton tug positions.", topology)
                    }
                }
                Ok(mesh)
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

impl Unwrap<&GeometryFile> for Mesh {
    fn unwrap(value: &GeometryFile) -> Result<Self, String>{
        return Err(value.source.clone())

    }
}

impl ManagedTypeRegistration for GeometryFile {
    fn get_all_type_registrations() -> Vec<bevy::reflect::TypeRegistration> {
        let type_registry = Vec::new();

        type_registry
    }
}