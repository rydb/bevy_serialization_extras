use bevy::reflect::{GetTypeRegistration, TypeRegistration};

/// trait that explains how to take struct and unwrap it into a bevy thing. 
/// Like [`From`], but returns either the Thing to be unwrapped or a filepath to thing.
pub trait Unwrap<T>: Sized {
    fn unwrap(value: T) -> Result<Self, String>;
}

/// trait that denotes that enum/struct/etc.. can fetch all of the type registrations of it self
/// !!!(TODO) there should be a way to ensure test cases for this are satisfied? !!!
pub trait ManagedTypeRegistration: GetTypeRegistration {
    /// takes all fields of this enum/struc/etc.., and returns a vec with their type registrations.
    fn get_all_type_registrations() -> Vec<TypeRegistration>;
}