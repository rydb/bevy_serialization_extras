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

/// bevy prefab meshes
#[derive(Clone, Reflect, PartialEq)]
pub enum MeshPrefab {
    Cuboid(Cuboid),
    Cylinder(Cylinder),
    Capsule(Capsule3d),
    Sphere(Sphere),
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

impl FromWrapper<MeshFlag3d> for Mesh3d {
    fn from_wrapper(
        value: &MeshFlag3d,
        asset_server: &Res<AssetServer>,
        assets: &mut ResMut<Assets<Self::AssetKind>>,
    ) -> Self {
        let asset = match value {
            MeshFlag3d::AssetPath(path) => asset_server.load(path),
            MeshFlag3d::Procedural(_) => {
                warn!("MeshFlag3d <--> Mesh conversion not implemented in bevy_serialization_core. Using fallback mesh.");
                assets.add(Cuboid::from_size(Vec3::new(0.1, 0.1, 0.1)))
            }
            MeshFlag3d::Prefab(prefab) => match prefab {
                MeshPrefab::Cuboid(cuboid) => assets.add(*cuboid),
                MeshPrefab::Cylinder(cylinder) => assets.add(*cylinder),
                MeshPrefab::Capsule(capsule3d) => assets.add(*capsule3d),
                MeshPrefab::Sphere(sphere) => assets.add(*sphere),
            },
        };
        Mesh3d(asset)
    }
}

impl FromAsset<Mesh3d> for MeshFlag3d {
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
