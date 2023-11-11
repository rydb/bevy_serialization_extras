use bevy::{reflect::{GetTypeRegistration, TypeRegistration}, prelude::{World, Entity}, ecs::query::WorldQuery};

/// trait that explains how to take struct and unwrap it into a bevy thing. 
/// Like [`From`], but returns either the Thing to be unwrapped or a filepath to thing.
pub trait Unwrap<T>: Sized {
    fn unwrap(value: T) -> Result<Self, String>;
}

/// trait that denotes that enum/struct/etc.. can fetch all of the type registrations of it self
///
/// !!!(TODO) there should be a way to ensure test cases for this are satisfied !!!
pub trait ManagedTypeRegistration: GetTypeRegistration {
    /// takes all fields of this enum/struc/etc.., and returns a vec with their type registrations.
    fn get_all_type_registrations() -> Vec<TypeRegistration>;
}

///trait that denotes that the struct is likely paired with other structs to create a structure(E.G: urdf)
pub trait Structure {
    /// returns the name of the structure this struct refers to. 
    fn structure(self) -> String;
}

// returns a list of filters from the given T 
// pub trait CollectFromQuery<T> {
//     fn filter_list(value: T) -> Vec<With>
// }

// pub trait BoundEntiy {

// }

/// A trait to be applied to bound queries to fetch the current entity of a specific query iteration.
pub trait AssociatedEntity<T> {
    fn associated_entity(value: T) -> Entity;
}

/// creates the struct via components that reference the same structure, but are individually distributed.
pub trait FromStructure<T: WorldQuery>: Sized {
    fn from_world(world: &World) -> Self;
}


// pub trait CollectFromQuery<T> {
//     fn return_vecs() -> Vec<Vec<_>>;
// }