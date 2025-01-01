use bevy_color::{Color, LinearRgba};
use bevy_serialization_core::prelude::{
    material::{MaterialFlag3d, MaterialWrapper},
    mesh::{MeshFlag3d, MeshPrefab},
};
use derive_more::From;
use glam::Vec3;
use nalgebra::Vector3;
use urdf_rs::Visual;

use bevy_math::prelude::*;

#[derive(From, Clone)]
pub struct VisualWrapper(Visual);

impl From<&VisualWrapper> for MaterialFlag3d {
    fn from(value: &VisualWrapper) -> Self {
        if let Some(material) = &value.0.material {
            if let Some(color) = &material.color {
                let rgba = color.rgba.0;
                Self::Wrapper(MaterialWrapper {
                    color: Color::LinearRgba(LinearRgba {
                        red: rgba[0] as f32,
                        green: rgba[1] as f32,
                        blue: rgba[2] as f32,
                        alpha: rgba[3] as f32,
                    }),
                })
            } else {
                Self::default()
            }
        } else {
            Self::default()
        }
    }
}

impl From<&VisualWrapper> for MeshFlag3d {
    fn from(value: &VisualWrapper) -> Self {
        let visual = &value.0;

        let urdf_geometry = &visual.geometry;

        let flag_geometry = match urdf_geometry {
            urdf_rs::Geometry::Box { size } => {
                let bevy_size = /*urdf_rotation_flip * */ Vector3::new(size[0], size[1], size[2]);
                Self::Prefab(MeshPrefab::Cuboid(Cuboid {
                    half_size: Vec3::new(
                        bevy_size[0] as f32,
                        bevy_size[1] as f32,
                        bevy_size[2] as f32,
                    ),
                }))
            }
            urdf_rs::Geometry::Cylinder { radius, length } => {
                //TODO: double check that this is correct
                Self::Prefab(MeshPrefab::Cylinder(Cylinder {
                    radius: *radius as f32,
                    half_height: *length as f32,
                }))
            }
            urdf_rs::Geometry::Capsule { radius, length } => {
                //TODO: double check that this is correct
                Self::Prefab(MeshPrefab::Capsule(Capsule3d {
                    radius: *radius as f32,
                    half_length: *length as f32,
                }))
            }
            urdf_rs::Geometry::Sphere { radius } => Self::Prefab(MeshPrefab::Sphere(Sphere {
                radius: *radius as f32,
            })),
            urdf_rs::Geometry::Mesh { filename, .. } => Self::AssetPath(filename.to_owned()),
        };
        flag_geometry
    }
}
