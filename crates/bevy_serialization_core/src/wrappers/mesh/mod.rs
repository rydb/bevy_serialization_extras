use bevy_asset::{AssetServer, Assets, Handle};
use derive_more::derive::From;
use log::warn;
//use bevy::prelude::*;
//use urdf_rs::Visual;
use crate::traits::*;

use bevy_ecs::prelude::*;
use bevy_math::prelude::*;
use bevy_reflect::prelude::*;
use bevy_render::prelude::*;

//TODO: Until:
// https://github.com/KhronosGroup/glTF-External-Reference
// becomes an extension supported by gltf-rs, or the spec is merged into gltf. gltf serialization cannot be supported in this library.
// pub mod gltf;

/// bevy prefab meshes
#[derive(Clone, Reflect, Debug, PartialEq, From)]
pub enum MeshPrefab {
    Cuboid(Cuboid),
    Cylinder(Cylinder),
    Capsule(Capsule3d),
    Sphere(Sphere),
    Cone(Cone),
    /// Fallback for unimplemented shapes. Should lead to fallback variant mesh.
    Unimplemented,
}

impl Default for MeshPrefab {
    fn default() -> Self {
        Self::Cuboid(Cuboid::from_length(0.1))
    }
}


#[derive(Component, Reflect, Clone, PartialEq)]
#[reflect(Component)]
pub enum MeshFlag3d {
    /// asset path to a model from bevy.
    AssetPath(String),
    //Prefab
    // procedural geometry loaded from bevy
    //TODO:
    Procedural(MeshWrapper),
    Prefab(MeshPrefab),
}

impl Default for MeshFlag3d {
    fn default() -> Self {
        Self::Procedural(MeshWrapper::default())
    }
}

pub const FALLBACK_MESH: Cuboid = Cuboid {
    half_size: Vec3 {
        x: 0.1,
        y: 0.1,
        z: 0.1,
    }
};

impl FromWrapper<MeshFlag3d> for Mesh3d {
    fn from_wrapper(
        value: &MeshFlag3d,
        asset_server: &Res<AssetServer>,
        assets: &mut ResMut<Assets<Self::AssetType>>,
    ) -> Self {
        let asset = match value {
            MeshFlag3d::AssetPath(path) => asset_server.load(path),
            MeshFlag3d::Procedural(_) => {
                warn!("MeshFlag3d <--> Mesh conversion not implemented in bevy_serialization_core. Using fallback mesh.");
                assets.add(FALLBACK_MESH)
            }
            MeshFlag3d::Prefab(prefab) => match prefab {
                MeshPrefab::Cuboid(cuboid) => assets.add(*cuboid),
                MeshPrefab::Cylinder(cylinder) => assets.add(*cylinder),
                MeshPrefab::Capsule(capsule3d) => assets.add(*capsule3d),
                MeshPrefab::Sphere(sphere) => assets.add(*sphere),
                MeshPrefab::Unimplemented => {
                    warn!("Attempted to convert unimplemented MeshPrefab kind to Mesh3d. Defaulting to fallback shape.");
                    assets.add(FALLBACK_MESH)
                },
                MeshPrefab::Cone(cone) => assets.add(*cone),
            },
            //MeshFlag3d::Handle(handle) => handle.clone(),
        };
        Mesh3d(asset)
    }
}

impl FromAsset<Mesh3d> for MeshFlag3d {
    fn from_asset(value: &Mesh3d, _: &ResMut<Assets<<Mesh3d as AssetHandleComponent>::AssetType>>) -> Self {
        match value.0.path() {
            Some(path) => Self::AssetPath(path.to_string()),
            None => Self::Procedural(MeshWrapper),
        }
    }
}

impl AssetHandleComponent for Mesh3d {
    type AssetType = Mesh;
}
/// THIS IS A DUMMY
///
/// TODO: IMPLEMENT PROPERLY.
#[derive(Clone, Reflect, Default, PartialEq)]
pub struct MeshWrapper;
