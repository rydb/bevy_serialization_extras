use bevy_asset::Asset;
use bevy_color::prelude::*;
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::component::Component;
use bevy_ecs::prelude::ReflectComponent;
use bevy_pbr::prelude::*;
use bevy_reflect::prelude::*;
use bevy_utils::prelude::*;
use bytemuck::TransparentWrapper;
use derive_more::derive::From;

use crate::traits::{AssetState, AssetSynonymTarget, SelfPath, SelfPure, SynonymPaths, SynonymPure};

/// serializable wrapper for mesh materials
#[derive(Component, Reflect, Clone, PartialEq, From)]
#[reflect(Component)]
pub enum Material3dFlag {
    Pure(MaterialWrapper),
    Path(String),
}

impl SynonymPaths for Material3dFlag {
    type Pure = MaterialWrapper;

    type Path = String;
    
    fn asset_state(&self) -> AssetState<SelfPure<Self>, SelfPath<Self>> {
        match self {
            Material3dFlag::Pure(material_wrapper) => AssetState::Pure(material_wrapper),
            Material3dFlag::Path(path) => AssetState::Path(path),
        }
    }
}

#[derive(Clone, From, PartialEq, Reflect)]
pub enum MaterialWrapper {
    Color(Color),
}

#[derive(From, Clone, Deref, DerefMut, Default, TransparentWrapper)]
#[repr(transparent)]
pub struct MeshMaterial3dRepr<T: Material>(MeshMaterial3d<T>);

impl AssetSynonymTarget for MeshMaterial3dRepr<StandardMaterial> {
    type Synonym = Material3dFlag;
    type AssetType = StandardMaterial;

    fn from_synonym(value: &SynonymPure<Self>) -> Self::AssetType {
        match value {
            MaterialWrapper::Color(color) => Self::AssetType {
                base_color: *color,
                ..default()
            },
        }
    }
    
    fn from_asset(value: &Self::AssetType) -> SynonymPure<Self> {
        SynonymPure::<Self>::Color(value.base_color)
    }
}

impl Default for Material3dFlag {
    fn default() -> Self {
        Material3dFlag::Pure(Color::default().into())
    }
}
