use bevy_derive::Deref;
use bytemuck::TransparentWrapper;
use derive_more::derive::From;
use crate::traits::*;

use bevy_ecs::prelude::*;
use bevy_math::prelude::*;
use bevy_reflect::prelude::*;
use bevy_render::prelude::*;

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
                MeshPrefab::Unimplemented(_kind) => FALLBACK_MESH.into(),
            },
            MeshWrapper::Procedural(mesh) => mesh.clone(),
        }
    }
}

//TODO: Implement properly when mesh -> mesh file conversion exists to use this.
impl From<&Mesh> for MeshWrapper {
    fn from(value: &Mesh) -> Self {
        Self::Procedural(value.clone())
    }
}

impl Default for MeshPrefab {
    fn default() -> Self {
        Self::Cuboid(Cuboid::from_length(0.1))
    }
}

#[derive(Component, Reflect, From)]
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

impl SynonymPaths for Mesh3dFlag {
    type Pure = MeshWrapper;

    type Path = String;

    fn asset_state(&self) -> AssetState<SelfPure<Self>, SelfPath<Self>> {
        match self {
            Self::Pure(material_wrapper) => AssetState::Pure(material_wrapper),
            Self::Path(path) => AssetState::Path(path),
        }
    }
}

#[derive(Reflect, From)]
pub enum MeshWrapper {
    Prefab(MeshPrefab),
    Procedural(Mesh),
}

#[derive(TransparentWrapper, Deref)]
#[repr(transparent)]
pub struct Mesh3dRepr(Mesh3d);

// impl AssetSynonymTarget for Mesh3dRepr {
//     type Synonym = Mesh3dFlag;

//     type AssetType = Mesh;

//     fn from_synonym(value: &SynonymPure<Self>) -> Self::AssetType {
//         match value {
//             MeshWrapper::Prefab(mesh_prefab) => mesh_prefab.into(),
//             MeshWrapper::Procedural(mesh) => mesh,
//         }
//     }

//     fn from_asset(value: &Self::AssetType) -> SynonymPure<Self> {
//         todo!()
//     }
// }

// impl AssetSynonym for Mesh3dFlag {
//     type SynonymTarget = Mesh3d;

//     type PureVariant = MeshWrapper;

//     fn asset_state(&self) -> AssetState<Self::PureVariant, String> {
//         match self {
//             Self::Pure(material_wrapper) => AssetState::Pure(material_wrapper),
//             Self::Path(path) => AssetState::Path(path),
//         }
//     }
// }

/// TODO: Implement this a bevy <-> mesh converter for this library exists.
///
#[derive(Reflect, Clone, PartialEq)]
pub(crate) struct ProceduralMeshWrapper;

impl Default for Mesh3dFlag {
    fn default() -> Self {
        Self::Pure(MeshWrapper::Procedural(FALLBACK_MESH.into()))
    }
}

pub const FALLBACK_MESH: Cuboid = Cuboid {
    half_size: Vec3 {
        x: 0.1,
        y: 0.1,
        z: 0.1,
    },
};
