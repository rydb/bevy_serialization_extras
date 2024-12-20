use bevy_asset::{Asset, Assets};
//use bevy::prelude::*;
use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;
use bevy_color::prelude::*;
use bevy_pbr::prelude::*;
use bevy_utils::prelude::*;

use crate::traits::{AssetKind, FromAsset, FromWrapper};

// #[derive(Component, Reflect, Clone, Default, PartialEq)]
// #[reflect(Component)]
// pub struct MaterialFlag {
//     pub color: Color,
// }


#[derive(Default, Reflect, Clone, PartialEq)]
pub struct MaterialWrapper {
    pub color: Color,
}

#[derive(Component, Reflect, Clone, PartialEq)]
#[reflect(Component)]
pub enum MaterialFlag {
    Wrapper(MaterialWrapper),
    AssetPath(String),
}

impl Default for MaterialFlag {
    fn default() -> Self {
        Self::Wrapper(MaterialWrapper::default())
    }
}

// impl Default for MaterialWrapper {
//     fn default() -> Self {
//         Color::default().into()
//     }
// }

// #[derive(Component, Reflect, Clone, Default)]
// #[reflect(Component)]
// pub struct MaterialFile {
//     pub path: String,
// }

// impl From<&MaterialFlag> for StandardMaterial


impl FromWrapper<MaterialFlag> for MeshMaterial3d<StandardMaterial> {
    fn from_wrapper(value: &MaterialFlag, asset_server: &Res<bevy_asset::AssetServer>, assets: &mut ResMut<bevy_asset::Assets<Self::AssetKind>>) -> Self {
        match value {
            MaterialFlag::Wrapper(material_wrapper) => {
                let mat = StandardMaterial {
                    base_color: material_wrapper.color,
                    ..default()
                };
                let mat_handle = assets.add(mat);
                MeshMaterial3d(mat_handle)
            },
            MaterialFlag::AssetPath(path) => {
                let mat_handle = asset_server.load(path);
                MeshMaterial3d(mat_handle)
            },
        }
    }
}


impl FromAsset<MeshMaterial3d<StandardMaterial>> for MaterialFlag {
    fn from_asset(value: &MeshMaterial3d<StandardMaterial>, assets: &ResMut<Assets<StandardMaterial>>) -> Self {
            match value.0.path() {
                Some(path) => Self::AssetPath(path.to_string()),
                None => {
                    let asset = assets.get(value.id());
                    if let Some(asset) = asset {
                        Self::Wrapper(MaterialWrapper::from(asset))
                    } else {
                        Self::Wrapper(MaterialWrapper::default())
                    }
                }
                
            }
    }
}


// impl<T: Material> From<MeshMaterial3d<T>> for MaterialFlag {
//     fn from(value: MeshMaterial3d<T>) -> Self {
//         match value.0.path() {
//             Some(path) => Self::AssetPath(path.to_string())
//         }
//     }
// }

impl<T: Asset + Material> AssetKind for MeshMaterial3d<T> {
    type AssetKind = T;
}

// impl<T> From<&MaterialWrapper> for T {
//     fn from(value: &MaterialWrapper) -> Self {
//         Self::from(value)
        
//         // Self {
//         //     base_color: value.color,
//         //     ..default()
//         // }
//     }
// }

// impl<T> From<&T> for MaterialWrapper {
//     fn from(value: &T) -> Self {
//         Self::from(value)
//     }
// }

impl From<&StandardMaterial> for MaterialWrapper {
    fn from(value: &StandardMaterial) -> Self {
        Self {
            color: value.base_color,
            //..default()
        }
    }
}

impl From<Color> for MaterialWrapper {
    fn from(value: Color) -> Self {
        Self {
            color: value,

        }
    }
}
