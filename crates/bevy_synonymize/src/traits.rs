use bevy_asset::prelude::*;
use bevy_ecs::prelude::*;
use bevy_reflect::{FromReflect, GetTypeRegistration, Reflect, Typed};
use bytemuck::TransparentWrapper;
use std::ops::Deref;
pub trait ComponentSynonym
where
    Self: Component
        + Reflect
        + FromReflect
        + Typed
        + GetTypeRegistration
        + for<'a> From<&'a Self::SynonymTarget>,
    Self::SynonymTarget: Clone + for<'a> From<&'a Self>,
{
    type SynonymTarget: Component + Clone;
}

// pub type AssetType<T> = <<T as AssetSynonym>::SynonymTarget as AssetHandleComponent>::AssetType;

// pub type AssetTypeNew<T> = <<T as Deref>::Target as Deref>::Target;
pub enum AssetState<'a, Pure, Path: Into<String>> {
    Pure(&'a Pure),
    Path(&'a Path),
}

// pub trait AssetSynonym
// where
//     Self: Component
//         + Reflect
//         + FromReflect
//         + Typed
//         + GetTypeRegistration
//         + From<String>
//         + From<Self::PureVariant>,
//     Self::SynonymTarget: Deref<Target = Handle<AssetType<Self>>>
//         + From<Handle<AssetType<Self>>>
//         + AssetHandleComponent,
//     AssetType<Self>: for<'a> From<&'a Self::PureVariant>,
//     Self::PureVariant: for<'a> From<&'a AssetType<Self>>,
// {
//     type PureVariant;
//     type SynonymTarget: Component + Deref;

//     fn asset_state(&self) -> AssetState<Self::PureVariant, String>;
// }

/// convienience alias
pub type SelfPure<T> = <T as SynonymPaths>::Pure;

/// convienience alias
pub type SelfPath<T> = <T as SynonymPaths>::Path;

pub trait SynonymPaths {
    type Pure;
    type Path;

    fn asset_state(&self) -> AssetState<SelfPure<Self>, String>;
}

pub type SynonymPath<T> = <<T as AssetSynonymTarget>::Synonym as SynonymPaths>::Path; 
pub type SynonymPure<T> = <<T as AssetSynonymTarget>::Synonym as SynonymPaths>::Pure;

pub type SynonymTarget<T> = <T as Deref>::Target;

/// trait for newtype impl around Synonym <-> Target conversions.
pub trait AssetSynonymTarget 
    where
        Self: Deref<Target: From<Handle<Self::AssetType>> + Sized + Component + Deref<Target = Handle<Self::AssetType>>> + TransparentWrapper<Self::Target>,
        Self::Target: Deref,
{
    type Synonym: Reflect + FromReflect + GetTypeRegistration + From<String> + From<SynonymPure<Self>> + Component + SynonymPaths;
    type AssetType: Asset;

    fn from_synonym(value: &SynonymPure<Self>) -> Self::AssetType;
    
    fn from_asset(value: &Self::AssetType) -> SynonymPure<Self>;
}

// component on a query that is checked for changes
//FIXME: make this work with a set of components, or better, change to use a "component iter" to have this work for all components in query
pub trait ChangeChecked {
    type ChangeCheckedComp: Component;
}