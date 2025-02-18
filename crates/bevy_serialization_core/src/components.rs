use std::any::type_name;

use bevy_ecs::{component::{ComponentHooks, StorageType}, prelude::*};
use bevy_log::warn;
use bevy_reflect::Reflect;
use derive_more::derive::From;


pub trait FlagTarget {
    type FlagTarget: Component;
}

/// The componentized version of a wrapper for another component.
#[derive(Reflect, From)]
pub struct Flag<T: Reflect + FlagTarget + Clone>(pub T);

impl<T: Reflect + FlagTarget + Clone> Component for Flag<T>
    where
        T::FlagTarget: From<T>
{
    const STORAGE_TYPE: StorageType = StorageType::Table;

    /// Called when registering this component, allowing mutable access to its [`ComponentHooks`].
    fn register_component_hooks(_hooks: &mut ComponentHooks) {
        _hooks.on_add(|mut world, e, id| {
            let comp = {
                match world.entity(e).get::<Self>() {
                    Some(val) => val,
                    None => {
                        warn!("could not get {:#?} on: {:#}", type_name::<Self>(), e);
                        return
                    },
                }
            };
            let flag_target = T::FlagTarget::from(comp.0.clone());

            world.commands().entity(e).insert(flag_target);
            
        });
    }
}