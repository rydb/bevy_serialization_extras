use std::{collections::HashMap, ops::Deref};
use bevy_ecs::prelude::*;
use bevy_asset::prelude::*;

/// trait that explains how to take struct and unwrap it into a bevy thing.
/// Like [`From`], but returns either the Target to be unwrapped or a filepath to thing.
pub trait Unwrap<T>: Sized {
    fn unwrap(value: T) -> Result<Self, String>;
}

pub trait FromStructure
where
    Self: Sized + Asset,
{
    fn into_entities(commands: &mut Commands, value: Self, spawn_request: AssetSpawnRequest<Self>);
}

pub trait IntoHashMap<T>
where
    Self: Sized,
{
    fn into_hashmap(value: T) -> HashMap<String, Self>;
}


use crate::resources::AssetSpawnRequest;
use thiserror::Error;
use urdf_rs::UrdfError;

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum LoadError {
    //FIXME: figure out how to convert urdf into a generic error. This lib should not need to import urdf_rs for a single error!!!
    #[error("Failed load urdf")]
    Io(#[from] UrdfError),
    // #[error("Failed to parse urdf")]
    // SaveError,
}

/// deserialize trait that works by offloading deserialization to desired format's deserializer
pub trait LazyDeserialize
where
    Self: Sized,
{
    fn deserialize(absolute_path: String) -> Result<Self, LoadError>;
}

pub trait LazySerialize
where
    Self: Sized,
{
    fn serialize(absolute_path: String) -> Result<Self, LoadError>;
}

///trait that denotes that the struct is likely paired with other structs to create a structure(E.G: urdf)
pub trait Structure<T> {
    /// returns the name of the structure this struct refers to.
    fn structure(value: T) -> String;
}

// component on a query that is checked for changes
//FIXME: make this work with a set of components, or better, change to use a "component iter" to have this work for all components in query
pub trait ChangeChecked {
    type ChangeCheckedComp: Component;
}

pub trait AsBundle<T: Bundle> {
    fn into_bundle(self) -> T;
}

/// denotes that this struct unfolds into something else. Usually means that the struct is "object oriented", and can be unfolded into an ECS compliant variant.
pub trait Unfold<T> {
    fn unfolded(value: T) -> Self;
}

pub trait FromWrapper<T>
    where
        Self: AssetKind + Deref<Target = Handle<Self::AssetKind>>,
{
    fn from_wrapper(value: &T, asset_server: &Res<AssetServer>, assets: &mut ResMut<Assets<Self::AssetKind>>) -> Self;
}

pub trait FromAsset<T> 
    where
        T: AssetKind + Deref<Target = Handle<T::AssetKind>>,
        // Self: From<&'a T::AssetKind>
{
    fn from_asset(value: &T, assets: &ResMut<Assets<T::AssetKind>>) -> Self;

}

// pub trait FromAsset<T>
//     where
//         Self: Component + AssetKind
// {
//     fn from(value: &Self, asset_server: ResMut<Assets<Self::AssetKind>>) {
        
//     }
// }

pub trait AssetKind {
    type AssetKind: Asset; 
}
