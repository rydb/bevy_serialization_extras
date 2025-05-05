use bevy_asset::{meta::Settings, saver::AssetSaver, Asset, AssetLoader, Handle, UntypedHandle};
use bevy_ecs::{
    prelude::*,
    system::{SystemParam, SystemParamItem},
};
use bytemuck::TransparentWrapper;
use ref_cast::RefCast;
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

pub enum Source {
    Asset(UntypedHandle),
    Component
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
    Self: Send + Sync + Deref<Target: Sized> + TransparentWrapper<Self::Target> + 'static,
{
    // type Settings: Send + Sync + Clone;
    fn components(value: &Self, settings: DisassembleSettings, source: Source) -> Structure<impl Bundle, impl Bundle>;
}
pub trait AssetLoadSettings {
    /// Settings for how this asset is loaded
    type LoadSettingsType: Settings + Default;
    // const LOADSETTINGS: Option<&'static Self::LoadSettingsType>;
    fn load_settings() -> Option<Self::LoadSettingsType>;
}

/// Weather to split children off into seperate entities or have them as children to a parent.
#[derive(Debug, Clone, Default)]
pub struct Split {
    pub split: bool,
    // when spliting, weather to have split parts inheriet transform from their former parent.
    // this is nessecary if using transform propagation.
    pub inheriet_transform: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct PullDown(pub TypeId);

impl PullDown {
    pub fn id<T: Component>() -> Self {
        Self(TypeId::of::<T>())
    }
}

// pub enum Structure<T> {
//     Root(T),
//     Children(Vec<T>, Split),
// }

pub struct Structure<T,U> {
    /// components going on the entity this component is attached to.
    pub root: T,
    /// components going onto the sub-components/children of this entity.
    pub children: Vec<U>,
    /// settings for what (if) children are split off from the parent/child hierarchy.
    pub split: Split
}
