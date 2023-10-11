use bevy::{prelude::App, reflect::GetTypeRegistration};

pub trait Unwrap<T>: Sized {
    fn unwrap(value: T) -> Result<Self, String>;
}

/// trait for recursively type registering all fields of a struct into the type registry
pub trait RecursiveTypeRegister<T: GetTypeRegistration> {
    /// takes all fields for this component, and registers them.
    fn register_types_all(app: &mut App) -> App;
}