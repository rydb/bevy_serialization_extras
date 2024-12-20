use bevy_asset::prelude::*;
use log::warn;
//use bevy::prelude::*;
//use urdf_rs::Visual;
use crate::traits::*;

use bevy_ecs::prelude::*;
use bevy_math::prelude::*;
use bevy_reflect::prelude::*;
use bevy_render::prelude::*;

pub mod obj;

//TODO: Until:
// https://github.com/KhronosGroup/glTF-External-Reference
// becomes an extension supported by gltf-rs, or the spec is merged into gltf. gltf serialization cannot be supported in this library.
// pub mod gltf;

#[derive(Component, Reflect, Clone, PartialEq)]
#[reflect(Component)]
pub enum MeshFlag {
    /// asset path to a model from bevy.
    AssetPath(String),
    //Prefab
    // procedural geometry loaded from bevy
    //TODO:
    Procedural(MeshWrapper),
}

impl Default for MeshFlag {
    fn default() -> Self {
        Self::Procedural(MeshWrapper::default())
    }
}

impl FromWrapper<MeshFlag> for Mesh3d {
    fn from_wrapper(
        value: &MeshFlag,
        asset_server: &Res<AssetServer>,
        assets: &mut ResMut<Assets<Self::AssetKind>>,
    ) -> Self {
        match value {
            MeshFlag::AssetPath(path) => {
                let asset: Handle<Self::AssetKind> = asset_server.load(path);
                Mesh3d(asset)
            }
            MeshFlag::Procedural(_) => {
                warn!("MeshFlag <--> Mesh conversion not implemented in bevy_serialization_core. Using fallback mesh.");

                let asset = assets.add(Cuboid::from_size(Vec3::new(0.1, 0.1, 0.1)));
                Mesh3d(asset)
            }
        }
    }
}

impl FromAsset<Mesh3d> for MeshFlag {
    fn from_asset(value: &Mesh3d, _: &ResMut<Assets<<Mesh3d as AssetKind>::AssetKind>>) -> Self {
        match value.0.path() {
            Some(path) => Self::AssetPath(path.to_string()),
            None => Self::Procedural(MeshWrapper),
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

// impl From<Plane3d> for MeshFlag {
//     fn from(value: Plane3d) -> Self {
//         Self {
//             primitive: MeshPrimitive::Cuboid {
//                 size: [value.size, 1.0, value.size],
//             },
//             orientation_matrix: IDENTITY_MATRIX,
//         }
//     }
// }
