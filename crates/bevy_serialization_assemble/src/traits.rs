use bevy_asset::{Asset, AssetLoader, saver::AssetSaver};
use bevy_ecs::{
    prelude::*,
    system::{SystemParam, SystemParamItem},
};
use std::{any::TypeId, collections::HashSet, ops::Deref};

/// The trait for assembling a structure into its root asset.
///
/// I.E: (Mesh, Material, Name) -> Assemble(FormatWrapper(Format)) -> model.format
pub trait Assemble
where
    Self: Sized
        + AssembleParms
        + Send
        + Sync
        +
        //LazySerialize +
        Deref<Target: Asset + Sized>,
{
    type Saver: Send
        + Default
        + AssetSaver<Asset = Self::Target>
        + AssetSaver<Settings = Self::Settings>;
    type Loader: Send + Default + AssetLoader<Asset = Self::Target>;
    type Settings: Send + Default;
    fn assemble(selected: HashSet<Entity>, value: SystemParamItem<Self::Params>) -> Self::Target;
}

pub trait AssembleParms {
    /// params to fetch world data to assemble(put queries/resource/etc.. like a traditional bevy system in here)
    type Params: SystemParam;
}

#[derive(Clone, Debug, Default)]
pub struct DisassembleSettings {
    pub split: bool,
}

#[derive(Clone, Debug, Default)]
pub struct DisassembleSettings {
    pub split: bool,
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
    // type Settings: Send + Sync + Clone;
    fn components(value: Self, settings: DisassembleSettings) -> Structure<impl Bundle>;
    // type Settings: Send + Sync + Clone;
    fn components(value: Self, settings: DisassembleSettings) -> Structure<impl Bundle>;
}

/// Weather to split children off into seperate entities or have them as children to a parent.
#[derive(Debug, Clone, Default)]
pub struct Split {
    pub split: bool,
    // when spliting, weather to have split parts inheriet transform from their former parent.
    // this is nessecary if using transform propagation.
    pub inheriet_transform: bool
}

#[derive(Clone, Copy, Debug)]
pub struct PullDown(pub (crate) TypeId);


impl PullDown {
    pub fn id<T: Component>() -> Self {
        Self (TypeId::of::<T>())
    }
}



#[derive(Debug, Clone, Copy, Default)]
pub struct Split(pub bool);

pub enum Structure<T> {
    Root(T),
    Children(Vec<T>, Split),
}
