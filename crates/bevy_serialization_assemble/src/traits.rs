use bevy_ecs::{prelude::*, system::{SystemParam, SystemParamItem}};
use std::ops::Deref;

/// The trait for assembling a structure into its root asset.
/// 
/// I.E: (Mesh, Material, Name) -> Assemble(FormatWrapper(Format)) -> model.format
pub trait Assemble
where
    Self: Sized + AssembleParms,
{
    fn assemble(selected: Vec<Entity>, value: SystemParamItem<Self::Params>) -> Self;
}

pub trait AssembleParms {
    /// params to fetch world data to assemble(put queries/resource/etc.. like a traditional bevy system in here)
     type Params: SystemParam;
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
        Self: Clone + Send + Sync + Deref + 'static,
{
    fn components(value: Self) -> Structure<impl Bundle>;
}

/// Weather to split children off into seperate entities or have them as children to a parent.
pub struct Split(pub bool);

pub enum Structure<T> {
    Root(T),
    Children(Vec<T>, Split)
}

// /// deserialize trait that works by offloading deserialization to desired format's deserializer
// pub trait LazyDeserialize
// where
//     Self: Sized,
// {
//     fn deserialize(absolute_path: String, world: &World) -> Result<Self, LoadError>;
// }

pub trait LazySerialize
where
    Self: Sized,
{
    fn serialize(&self, name: String) -> Result<(), anyhow::Error>;
}


// #[non_exhaustive]
// #[derive(Error, Debug)]
// pub enum LoadError {
//     Error(String),
// }


// /// Errors in saving assets.
// #[non_exhaustive]
// #[derive(Error, Debug)]
// pub enum SaveError {
//     File(#[from] std::io::Error),
//     Other(#[from] Box<dyn std::error::Error>)
// }


// impl Display for SaveError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let res = match self {
//             SaveError::Other(err) => write!(f, "Error: {:#}", err),
//             SaveError::File(error) => write!(f, "Error: {:#}", error),
//         };
//         res
//     }
// }

