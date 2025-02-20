use std::{any::type_name, ops::Deref};

use bevy_asset::Asset;
use bevy_ecs::{component::{ComponentHooks, ComponentId, StorageType}, prelude::*, system::SystemId};
use bevy_log::warn;
use bevy_reflect::Reflect;
use bevy_utils::HashMap;
use derive_more::derive::From;

use crate::{systems::serialize_for, traits::ComponentWrapper};


#[derive(Resource, Default)]
pub struct EchoizerSerializers(pub HashMap<ComponentId, SystemId>);
#[derive(Resource, Default)]
pub struct EchoizerDeserializers(pub HashMap<ComponentId, SystemId>);


#[derive(Resource, Default)]
pub struct EchoSerializers(pub HashMap<ComponentId, SystemId>);

#[derive(Resource, Default)]
pub struct EchoDeserializers(pub HashMap<ComponentId, SystemId>);


/// The componentized version of a wrapper for another component.
#[derive(Reflect, From)]
pub struct Echoize<T: Reflect + Asset + Clone>(pub T);

#[derive(Clone)]
pub struct Echo<T: Reflect + ComponentWrapper + Clone>(pub T);

impl<T: Reflect + ComponentWrapper + Clone> Component for Echo<T>{
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
            let echo = T::Echo::from(comp.0.clone());
            world.commands().entity(e).insert(echo);

            if world.get_resource::<EchoSerializers>().unwrap().0.contains_key(&id) == false {
                let system_id = {
                    world.commands().register_system(serialize_for::<T>)
                };
            }
            // match comp.0.clone().retrieve_target() {
            //     FlagKind::Component(comp) => {
            //         todo!()
            //         // if world.get_resource_mut::<AssetCheckers>().unwrap().0.contains_key(&id) == false {
            //         //     let system_id = {
            //         //         world.commands().register_system(initialize_asset_structure::<T>)
            //         //     };
            //         //     let mut asset_checkers = world.get_resource_mut::<AssetCheckers>().unwrap();

            //         //     asset_checkers.0.insert(id, system_id);
            //         // }
            //     },
            //     FlagKind::Asset(pure_or_path) => todo!(),
            // }
            //let x = T::Target::from(comp.0.clone());

            //let flag_target = T::Target::from(comp.0.clone());

            //world.commands().entity(e).insert(flag_target);
            
        });
    }
}