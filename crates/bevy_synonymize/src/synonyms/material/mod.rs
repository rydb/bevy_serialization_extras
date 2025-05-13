use bevy_asset::Asset;
use bevy_color::prelude::*;
use bevy_ecs::component::Component;
use bevy_ecs::prelude::ReflectComponent;
use bevy_pbr::prelude::*;
use bevy_reflect::prelude::*;
use bevy_utils::prelude::*;
use derive_more::derive::From;

use crate::traits::{AssetHandleComponent, AssetState, AssetSynonym};

/// serializable wrapper for mesh materials
#[derive(Component, Reflect, Clone, PartialEq, From)]
#[reflect(Component)]
pub enum Material3dFlag {
    Pure(MaterialWrapper),
    Path(String),
}

#[derive(Clone, From, PartialEq, Reflect)]
pub enum MaterialWrapper {
    Color(Color),
}

impl AssetSynonym for Material3dFlag {
    type SynonymTarget = MeshMaterial3d<StandardMaterial>;
    type PureVariant = MaterialWrapper;

    fn asset_state(&self) -> AssetState<Self::PureVariant, String> {
        match self {
            Material3dFlag::Pure(material_wrapper) => AssetState::Pure(material_wrapper),
            Material3dFlag::Path(path) => AssetState::Path(path),
        }
    }
}
// impl<'a, 'b> Into<AssetState<'b, <Material3dFlag as AssetSynonym>::PureVariant, <Material3dFlag as AssetSynonym>::PathVariant>> for &'a Material3dFlag

// impl IntoAssetState for Material3dFlag {
//     fn asset_state(&self) -> AssetState<Self::PureVariant,  Self::PathVariant> {
//         match self {
//             Material3dFlag::Pure(material_wrapper) => AssetState::Pure(material_wrapper),
//             Material3dFlag::Path(path) => AssetState::Path(path),
//         }
//     }
// }

// impl<'a, 'b> From<&'a Material3dFlag> for AssetState<'b, <Material3dFlag as AssetSynonym>::PureVariant, <Material3dFlag as AssetSynonym>::PathVariant>
//     where
//         'a: 'b
// {
//     fn from(value: &'a Material3dFlag) -> Self {
//         match value {
//             Material3dFlag::Pure(material_wrapper) => AssetState::Pure(material_wrapper),
//             Material3dFlag::Path(path) => AssetState::Path(path),
//         }
//     }
// }

impl From<&MaterialWrapper> for StandardMaterial {
    fn from(value: &MaterialWrapper) -> Self {
        match value {
            MaterialWrapper::Color(color) => Self {
                base_color: *color,
                ..default()
            },
        }
    }
}

impl From<&StandardMaterial> for MaterialWrapper {
    fn from(value: &StandardMaterial) -> Self {
        Self::Color(value.base_color)
    }
}

// impl FromAssetState<&Material3dFlag> for StandardMaterial {
//     fn from_asset_state(value: &Material3dFlag) -> AssetState<Self> {
//         match value {
//             Material3dFlag::Pure(color) => AssetState::Pure(
//                         Self {
//                             base_color: *color,
//                             ..default()
//                         }
//                     ),
//             //Material3dFlag::Path(path) => todo!(),
//         }
//     }
// }

// impl FromAssetState<&StandardMaterial> for Material3dFlag {
//     fn from_asset_state(value: &StandardMaterial) -> AssetState<Self> {
//         todo!()
//     }
// }

// impl From<&Material3dFlag> for AssetState<StandardMaterial> {
//     fn from(value: &Material3dFlag) -> Self {
//         match value {
//             Material3dFlag::Color(color) => AssetState::Pure(
//                 Self {
//                     base_color: *color,
//                     ..default()
//                 }
//             ),
//         }
//         // match value {
//         //     Material3dFlag::Color(color) => Self {
//         //         base_color: *color,
//         //         ..default()
//         //     },
//         // }
//     }
// }

impl From<&StandardMaterial> for Material3dFlag {
    fn from(value: &StandardMaterial) -> Self {
        Self::Pure(value.base_color.into())
    }
}

impl<T: Asset + Material> AssetHandleComponent for MeshMaterial3d<T> {
    type AssetType = T;
}

impl Default for Material3dFlag {
    fn default() -> Self {
        Material3dFlag::Pure(Color::default().into())
    }
}
