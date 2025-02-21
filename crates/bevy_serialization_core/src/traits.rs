use bevy_asset::prelude::*;
use bevy_ecs::prelude::*;
use bevy_reflect::{FromReflect, GetTypeRegistration, Reflect, Typed};
use std::ops::Deref;

/// trait that explains how to take struct and unwrap it into a bevy thing.
/// Like [`From`], but returns either the Target to be unwrapped or a filepath to thing.
pub trait Unwrap<T>: Sized {
    fn unwrap(value: T) -> Result<Self, String>;
}
pub trait ComponentWrapper 
    where
        Self: Reflect + Clone + for <'a> From<&'a Self::Target>,
        Self::Target: Clone + for <'a> From<&'a Self>
{
    type Target: Component + Clone;
}


pub type AssetType<T> = <<T as AssetWrapper>::WrapperTarget as AssetHandleComponent>::AssetType;

pub trait AssetWrapper
    where
        Self: PartialEq + Reflect + FromReflect + Typed + GetTypeRegistration + for <'a> From<&'a AssetType<Self>>,
        Self::WrapperTarget: Deref<Target = Handle<AssetType<Self>>> + From<Handle<AssetType<Self>>> + AssetHandleComponent,
        AssetType<Self>: for <'a> From<&'a Self>
{
    type WrapperTarget: Component + Deref;
}

// component on a query that is checked for changes
//FIXME: make this work with a set of components, or better, change to use a "component iter" to have this work for all components in query
pub trait ChangeChecked {
    type ChangeCheckedComp: Component;
}

/// conversion from ComponentWrapper -> Component(Handle<Asset>)
pub trait FromWrapper<T>
where
    Self: AssetHandleComponent + Deref<Target = Handle<Self::AssetType>>,
{
    fn from_wrapper(
        value: &T,
        asset_server: &Res<AssetServer>,
        assets: &mut ResMut<Assets<Self::AssetType>>,
    ) -> Self;
}

/// conversion from Component(Handle<Asset>) -> ComponentWrapper
pub trait FromAsset<T>
where
    T: AssetHandleComponent + Deref<Target = Handle<T::AssetType>>,
{
    fn from_asset(value: &T, assets: &ResMut<Assets<T::AssetType>>) -> Self;
}
/// a component that holds a handle to an asset. Nessecary to peel inner T without adding an extra generic to impls.
pub trait AssetHandleComponent {
    type AssetType: Asset;
}
