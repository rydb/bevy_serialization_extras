use bevy_ecs::prelude::*;
use std::{collections::HashMap, fmt::Display};
pub trait IntoHashMap<T>
where
    Self: Sized,
{
    fn into_hashmap(value: T, world: &World ) -> HashMap<String, Self>;
}

pub trait FromStructure {
    fn components(value: Self) -> Structure<impl Bundle>;
}

pub enum Structure<T> {
    Root(T),
    Children(Vec<T>)
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

/// newtype around asset. 
pub trait InnerTarget {
    type Inner;
}

