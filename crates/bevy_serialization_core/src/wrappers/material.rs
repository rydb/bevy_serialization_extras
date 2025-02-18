use bevy_asset::{Asset, Assets};
//use bevy::prelude::*;
use bevy_color::prelude::*;
use bevy_ecs::prelude::*;
use bevy_pbr::prelude::*;
use bevy_reflect::prelude::*;
use bevy_utils::prelude::*;
use derive_more::derive::From;

use crate::traits::{AssetKind, FromAsset, FromWrapper};

// /// General wrapper to represent materials.
// /// TODO: give this a full implementation.
// #[derive(Default, Reflect, Clone, PartialEq)]
// pub struct MaterialWrapper {
//     pub color: Color,
// }

/// Serializable version of MeshMaterial3d
#[derive(Reflect, Clone, PartialEq, From)]
pub enum MaterialWrapper{
    Color(Color),
    AssetPath(String),
}

impl Default for MaterialWrapper {
    fn default() -> Self {
        Color::default().into()
    }
}

impl FromWrapper<MaterialWrapper> for MeshMaterial3d<StandardMaterial> {
    fn from_wrapper(
        value: &MaterialWrapper,
        asset_server: &Res<bevy_asset::AssetServer>,
        assets: &mut ResMut<bevy_asset::Assets<Self::AssetKind>>,
    ) -> Self {
        match value {
            MaterialWrapper::Color(material_wrapper) => {
                let mat = StandardMaterial {
                    base_color: material_wrapper.clone(),
                    ..default()
                };
                let mat_handle = assets.add(mat);
                MeshMaterial3d(mat_handle)
            }
            MaterialWrapper::AssetPath(path) => {
                let mat_handle = asset_server.load(path);
                MeshMaterial3d(mat_handle)
            }
        }
    }
}

impl FromAsset<MeshMaterial3d<StandardMaterial>> for MaterialWrapper {
    fn from_asset(
        value: &MeshMaterial3d<StandardMaterial>,
        assets: &ResMut<Assets<StandardMaterial>>,
    ) -> Self {
        match value.0.path() {
            Some(path) => Self::AssetPath(path.to_string()),
            None => {
                let asset = assets.get(value.id());
                if let Some(asset) = asset {
                    Self::Color(asset.base_color)
                } else {
                    Self::AssetPath("UNIMPLEMENTED".to_owned())
                }
            }
        }
    }
}

impl<T: Asset + Material> AssetKind for MeshMaterial3d<T> {
    type AssetKind = T;
}

// impl From<&StandardMaterial> for MaterialWrapper {
//     fn from(value: &StandardMaterial) -> Self {
//         Self {
//             color: value.base_color,
//             //..default()
//         }
//     }
// }

// impl From<Color> for MaterialWrapper {
//     fn from(value: Color) -> Self {
//         Self { color: value }
//     }
// }
