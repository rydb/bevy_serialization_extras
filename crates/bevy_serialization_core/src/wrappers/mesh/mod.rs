use derive_more::derive::From;
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
#[derive(Reflect, Clone, Copy, Debug, PartialEq, From)]
pub enum MeshPrefab {
    Cuboid(Cuboid),
    Cylinder(Cylinder),
    Capsule(Capsule3d),
    Sphere(Sphere),
    Cone(Cone),
    /// Fallback for unimplemented shapes. Should lead to fallback variant mesh.
    Unimplemented(&'static str),
}

impl From<&MeshWrapper> for Mesh {
    fn from(value: &MeshWrapper) -> Self {
        match value {
            MeshWrapper::Prefab(mesh_prefab) => match *mesh_prefab {
                MeshPrefab::Cuboid(cuboid) => cuboid.into(),
                MeshPrefab::Cylinder(cylinder) => cylinder.into(),
                MeshPrefab::Capsule(capsule3d) => capsule3d.into(),
                MeshPrefab::Sphere(sphere) => sphere.into(),
                MeshPrefab::Cone(cone) => cone.into(),
                MeshPrefab::Unimplemented(kind) => FALLBACK_MESH.into(),
            },
            MeshWrapper::Procedural => FALLBACK_MESH.into(),
        }
    }
}

//TODO: Implement properly when mesh -> mesh file conversion exists to use this.
impl From<&Mesh> for MeshWrapper {
    fn from(_value: &Mesh) -> Self {
        Self::Procedural
    }
}



impl Default for MeshPrefab {
    fn default() -> Self {
        Self::Cuboid(Cuboid::from_length(0.1))
    }
}


#[derive(Component, Reflect, PartialEq, From)]
#[reflect(Component)]
pub enum Mesh3dFlag {
    /// asset path to a model from bevy.
    //Prefab
    // procedural geometry loaded from bevy
    //TODO:
    Path(String),
    //Procedural(ProceduralMeshWrapper),
    Pure(MeshWrapper),
}

#[derive(Component, Reflect, PartialEq, From)]
pub enum MeshWrapper {
    Prefab(MeshPrefab),
    Procedural,
}

impl AssetWrapper for Mesh3dFlag {
    type WrapperTarget = Mesh3d;
    
    type PureVariant = MeshWrapper;
    
    fn asset_state(&self) -> AssetState<Self::PureVariant, String> {
        todo!()
    }
}

/// TODO: Implement this a bevy <-> mesh converter for this library exists.
/// 
#[derive(Reflect, Clone, PartialEq)]
pub(crate) struct ProceduralMeshWrapper;

impl Default for Mesh3dFlag {
    fn default() -> Self {
        Self::Pure(MeshWrapper::Procedural)
    }
}

pub const FALLBACK_MESH: Cuboid = Cuboid {
    half_size: Vec3 {
        x: 0.1,
        y: 0.1,
        z: 0.1,
    }
};

impl AssetHandleComponent for Mesh3d {
    type AssetType = Mesh;
}