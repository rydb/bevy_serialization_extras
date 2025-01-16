use bevy_asset::prelude::*;
use bevy_ecs::prelude::*;
use std::{collections::HashMap, fmt::Display};
pub trait IntoHashMap<T>
where
    Self: Sized,
{
    fn into_hashmap(value: T, world: &World ) -> HashMap<String, Self>;
}

///trait that denotes that the struct is likely paired with other structs to create a structure(E.G: urdf)
pub trait Structure<T> {
    /// returns the name of the structure this struct refers to.
    fn structure(value: T) -> String;
}

// pub struct SubStructure<T: Component>(pub T);

pub enum ApplyTarget {
    Root,
    Child,
}
/// conversion from asset -> world entities with components
pub trait FromStructure{
    
    fn into_entities(commands: &mut Commands, root: Entity, value: Self);
    //TODO: re-attempt returning a bundle instead of adding components manually if optional components in bundle get stabalized(mabye 0.16?).
    //that is only blocker of improving the correctness of this method.
    //-> Vec<impl Bundle>;
}

/// deserialize trait that works by offloading deserialization to desired format's deserializer
pub trait LazyDeserialize
where
    Self: Sized,
{
    fn deserialize(absolute_path: String, world: &World) -> Result<Self, LoadError>;
}

pub trait LazySerialize
where
    Self: Sized,
{
    fn serialize(absolute_path: String) -> Result<Self, LoadError>;
}

use thiserror::Error;

use crate::resources::AssetSpawnRequest;

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
