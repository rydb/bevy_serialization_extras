use bevy::{reflect::{GetTypeRegistration, TypeRegistration}, ecs::{bundle::Bundle, component::Component}, utils::thiserror};

/// trait that explains how to take struct and unwrap it into a bevy thing. 
/// Like [`From`], but returns either the Thing to be unwrapped or a filepath to thing.
pub trait Unwrap<T>: Sized {
    fn unwrap(value: T) -> Result<Self, String>;
}

/// trait that denotes that enum/struct/etc.. can fetch all of the type registrations needed of itself.
///
/// this is placeholder to fill the gap of recursive type registration,
/// See: https://github.com/bevyengine/bevy/issues/4154
pub trait ManagedTypeRegistration: GetTypeRegistration {
    /// takes all fields of this enum/struc/etc.., and returns a vec with their type registrations.
    fn get_all_type_registrations() -> Vec<TypeRegistration>;
}

use thiserror::Error;
use urdf_rs::UrdfError;

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum LoadError {
    #[error("Failed load urdf")]
    Io(#[from] UrdfError),

    // #[error("Failed to parse urdf")]
    // SaveError,
}

/// deserialize trait that works by offloading deserialization to desired format's deserializer
pub trait LazyDeserialize
    where
        Self: Sized
{
    fn deserialize(absolute_path: String) -> Result<Self, LoadError>;
}

///trait that denotes that the struct is likely paired with other structs to create a structure(E.G: urdf)
pub trait Structure<T> {
    /// returns the name of the structure this struct refers to. 
    fn structure(value: T) -> String;
}

// component on a query that is checked for changes
//#TODO make this work with a set of components, or better, change to use a "component iter" to have this work for all components in query
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
