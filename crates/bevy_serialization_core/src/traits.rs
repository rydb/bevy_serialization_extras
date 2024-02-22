use std::collections::HashMap;

// use bevy::{
//     asset::Asset,
//     ecs::{bundle::Bundle, component::Component, system::Commands},
//     reflect::{GetTypeRegistration, TypeRegistration},
//     utils::thiserror,
// };


use bevy_ecs::prelude::*;
use bevy_asset::prelude::*;
use bevy_reflect::{GetTypeRegistration, TypeRegistration};
use bevy_utils::thiserror;


/// trait that explains how to take struct and unwrap it into a bevy thing.
/// Like [`From`], but returns either the Thing to be unwrapped or a filepath to thing.
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

/// trait that denotes that enum/struct/etc.. can fetch all of the type registrations needed of itself.
///
/// this is placeholder to fill the gap of recursive type registration,
/// See: https://github.com/bevyengine/bevy/issues/4154
pub trait ManagedTypeRegistration: GetTypeRegistration {
    /// takes all fields of this enum/struc/etc.., and returns a vec with their type registrations.
    fn get_all_type_registrations() -> Vec<TypeRegistration>;
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
