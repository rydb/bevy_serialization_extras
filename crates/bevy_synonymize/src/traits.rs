use bevy_asset::prelude::*;
use bevy_ecs::prelude::*;
use bevy_reflect::{FromReflect, GetTypeRegistration, Reflect, Typed};
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

pub type AssetType<T> = <<T as AssetSynonym>::SynonymTarget as AssetHandleComponent>::AssetType;

pub enum AssetState<'a, T, U> {
    Pure(&'a T),
    Path(&'a U),
}

pub trait AssetSynonym
where
    Self: Component
        + Reflect
        + FromReflect
        + Typed
        + GetTypeRegistration
        + From<String>
        + From<Self::PureVariant>,
    Self::SynonymTarget: Deref<Target = Handle<AssetType<Self>>>
        + From<Handle<AssetType<Self>>>
        + AssetHandleComponent,
    AssetType<Self>: for<'a> From<&'a Self::PureVariant>,
    Self::PureVariant: for<'a> From<&'a AssetType<Self>>,
{
    type SynonymTarget: Component + Deref;
    type PureVariant;

    fn asset_state(&self) -> AssetState<Self::PureVariant, String>;
}

// component on a query that is checked for changes
//FIXME: make this work with a set of components, or better, change to use a "component iter" to have this work for all components in query
pub trait ChangeChecked {
    type ChangeCheckedComp: Component;
}

/// conversion from ComponentSynonym -> Component(Handle<Asset>)
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

/// conversion from Component(Handle<Asset>) -> ComponentSynonym
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
