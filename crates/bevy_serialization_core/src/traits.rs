use bevy_asset::prelude::*;
use bevy_ecs::prelude::*;
use std::{collections::HashMap, fmt::Display, ops::Deref};

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

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum LoadError {
    Error(String),
}

impl Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            LoadError::Error(err) => write!(f, "Error: {:#}", err),
        };
        res
    }
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
/// conversion from ComponentWrapper -> Component(Handle<Asset>)
pub trait FromWrapper<T>
where
    Self: AssetKind + Deref<Target = Handle<Self::AssetKind>>,
{
    fn from_wrapper(
        value: &T,
        asset_server: &Res<AssetServer>,
        assets: &mut ResMut<Assets<Self::AssetKind>>,
    ) -> Self;
}

/// conversion from Component(Handle<Asset>) -> ComponentWrapper
pub trait FromAsset<T>
where
    T: AssetKind + Deref<Target = Handle<T::AssetKind>>,
{
    fn from_asset(value: &T, assets: &ResMut<Assets<T::AssetKind>>) -> Self;
}
pub trait AssetKind {
    type AssetKind: Asset;
}
