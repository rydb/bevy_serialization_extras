use bevy_asset::{saver::AssetSaver, Asset, AssetLoader};
use bevy_ecs::{
    prelude::*,
    system::{SystemParam, SystemParamItem},
};
use std::{collections::HashSet, ops::Deref};

/// The trait for assembling a structure into its root asset.
///
/// I.E: (Mesh, Material, Name) -> Assemble(FormatWrapper(Format)) -> model.format
pub trait Assemble
where
    Self: Sized + AssembleParms + Send + Sync
     + 
     //LazySerialize + 
     Deref<Target: Asset + Sized>,
{
    type Saver: Send + Default + AssetSaver<Asset = Self::Target> + AssetSaver<Settings = Self::Settings>;
    type Loader: Send + Default + AssetLoader<Asset = Self::Target>;
    type Settings: Send + Default;
    fn assemble(selected: HashSet<Entity>, value: SystemParamItem<Self::Params>) -> Self::Target;
}

pub trait AssembleParms {
    /// params to fetch world data to assemble(put queries/resource/etc.. like a traditional bevy system in here)
    type Params: SystemParam;
}

/// The trait for Disassembling structures into either:
///
/// A) its sub components
///
/// B) its children
///
/// I.E: model.format -> Disassemble(FormatWrapper(Format)) -> (Mesh, Material, Name)
pub trait Disassemble
where
    Self: Clone + Send + Sync + Deref + 'static,
{
    fn components(value: Self) -> Structure<impl Bundle>;
}

/// Weather to split children off into seperate entities or have them as children to a parent.
pub struct Split(pub bool);

pub enum Structure<T> {
    Root(T),
    Children(Vec<T>, Split),
}

pub trait LazySerialize
where
    Self: Sized,
{
    fn serialize(&self, name: String, folder_path: String) -> Result<(), anyhow::Error>;
}