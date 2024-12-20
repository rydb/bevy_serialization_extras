

use bevy_asset::prelude::*;
use log::warn;
//use bevy::prelude::*;
//use urdf_rs::Visual;
use crate::traits::*;

use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;
use bevy_render::prelude::*;
use bevy_math::prelude::*;


pub mod obj;

//TODO: Until: 
// https://github.com/KhronosGroup/glTF-External-Reference
// becomes an extension supported by gltf-rs, or the spec is merged into gltf. gltf serialization cannot be supported in this library.
// pub mod gltf;

// #[derive(Component, Default, Reflect, Clone)]
// #[reflect(Component)]
// pub struct GeometryFlag {
//     pub primitive: MeshPrimitive,
//     // matrix to "flip" the shape by. Not every format expresses orientation the same as bevy, so their positions/transforms are multiplied by their factor
//     // (here) to match bevy's orientation.
//     pub orientation_matrix: [Vec3; 3],
// }

#[derive(Component, Reflect, Clone, PartialEq)]
#[reflect(Component)]
pub enum GeometryFlag {
    /// asset path to a model from bevy.
    AssetPath(String),
    //Prefab
    // procedural geometry loaded from bevy
    //TODO:
    Procedural(MeshWrapper),
}

impl Default for GeometryFlag {
    fn default() -> Self {
        Self::Procedural(MeshWrapper::default())
    }
}

impl FromWrapper<GeometryFlag> for Mesh3d {
    fn from_wrapper(value: &GeometryFlag, asset_server: &Res<AssetServer>, assets: &mut ResMut<Assets<Self::AssetKind>>) -> Self {
        match value {
            GeometryFlag::AssetPath(path) => {
                let asset: Handle<Self::AssetKind> = asset_server.load(path);
                Mesh3d(asset)
            },
            GeometryFlag::Procedural(_) => {
                warn!("GeometryFlag <--> Mesh conversion not implemented in bevy_serialization_core. Using fallback mesh.");

                let asset = assets.add(
                    Cuboid::from_size(Vec3::new(0.1, 0.1, 0.1))
                );
                Mesh3d(asset)
            },
        }
    }
}

impl FromAsset<Mesh3d> for GeometryFlag {
    fn from_asset(value: &Mesh3d, _: &ResMut<Assets<<Mesh3d as AssetKind>::AssetKind>>) -> Self {
        match value.0.path() {
            Some(path) => {
                Self::AssetPath(path.to_string())
            },
            None => {
                Self::Procedural(MeshWrapper)
            },
        }
    }
}

impl AssetKind for Mesh3d {
    type AssetKind = Mesh;
}
/// THIS IS A DUMMY
/// 
/// TODO: IMPLEMENT PROPERLY.
#[derive(Clone, Reflect, Default, PartialEq)]
pub struct MeshWrapper;

// #[derive(Default, Component, Reflect, Clone)]
// #[reflect(Component)]
// pub struct GeometryFile {
//     pub source: String,
// }

// impl From<Cuboid> for GeometryFlag {
//     fn from(value: Cuboid) -> Self {
//         Self {
//             primitive: MeshPrimitive::Cuboid {
//                 size: value.size().into(),
//             },
//             orientation_matrix: IDENTITY_MATRIX,
//         }
//     }
// }

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

// impl Unwrap<&GeometryFlag> for Mesh {
//     fn unwrap(value: &GeometryFlag) -> Result<Self, String> {
//         let rotation_matrix = Matrix3::new(
//             i[0].x, i[0].y, i[0].z, i[1].x, i[1].y, i[1].z, i[2].x, i[2].y, i[2].z,
//         );

//     }
// }

// impl Unwrap<&GeometryFile> for Mesh {
//     fn unwrap(value: &GeometryFile) -> Result<Self, String> {
//         return Err(value.source.clone());
//     }
// }
