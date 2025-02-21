use std::{any::type_name, ops::Deref};

use bevy_asset::{Asset, Assets};
use bevy_ecs::{component::{ComponentHooks, ComponentId, StorageType}, prelude::*, system::SystemId};
use bevy_log::warn;
use bevy_reflect::{FromReflect, GetTypeRegistration, Reflect, Typed};
use bevy_utils::HashMap;
use derive_more::derive::From;

use crate::{prelude::{WrapAssetDeserializers, WrapAssetSerializers, WrapCompDeserializers, WrapCompSerializers}, systems::{deserialize_asset_for, deserialize_for, serialize_for, try_serialize_asset_for}, traits::{AssetHandleComponent, AssetType, AssetWrapper, ComponentWrapper}};





/// Wrapper component around an Comp(Handle<Asset<T>>)
#[derive(Reflect, From, Clone)]
pub struct WrapAsset<T: AssetWrapper>(pub T);

impl<T: AssetWrapper> Component for WrapAsset<T> {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    
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

            let handle = {
                let asset = AssetType::<T>::from(&comp.0);

                let Some(mut assets) = world.get_resource_mut::<Assets<AssetType<T>>>() else {
                    warn!("no mut Assets<T> found for {:#}", type_name::<Assets<AssetType<T>>>());
                    return;
                };
                assets.add(asset)
            };


            let componentized_asset = T::WrapperTarget::from(handle);
            world.commands().entity(e).insert(componentized_asset);

            if world.get_resource::<WrapAssetSerializers>().unwrap().0.contains_key(&id) == false {
                let registry = world.resource_mut::<AppTypeRegistry>();
                registry.write().register::<Self>();
                let system_id = {
                    world.commands().register_system(try_serialize_asset_for::<T>)
                };
                let mut checkers = world.get_resource_mut::<WrapCompSerializers>().unwrap();

                checkers.0.insert(id, system_id);
            }
            if world.get_resource::<WrapAssetDeserializers>().unwrap().0.contains_key(&id) == false {
                let system_id = {
                    world.commands().register_system(deserialize_asset_for::<T>)
                };
                let mut checkers = world.get_resource_mut::<WrapAssetDeserializers>().unwrap();

                checkers.0.insert(id, system_id);
            }


            //let handle = assets.add(T::Reflect::from(comp.0));
            //let handle = T::Target::from(comp.0.clone());
        });
    }
}

#[derive(Clone, From)]
pub struct WrapComp<T: Reflect + ComponentWrapper + Clone>(pub T);

impl<T: Reflect + ComponentWrapper> Component for WrapComp<T>{
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
            let target = T::Target::from(&comp.0);
            world.commands().entity(e).insert(target);

            if world.get_resource::<WrapCompSerializers>().unwrap().0.contains_key(&id) == false {
                let system_id = {
                    world.commands().register_system(serialize_for::<T>)
                };
                let mut checkers = world.get_resource_mut::<WrapCompSerializers>().unwrap();

                checkers.0.insert(id, system_id);
            }
            if world.get_resource::<WrapCompDeserializers>().unwrap().0.contains_key(&id) == false {
                let system_id = {
                    world.commands().register_system(deserialize_for::<T>)
                };
                let mut checkers = world.get_resource_mut::<WrapCompDeserializers>().unwrap();

                checkers.0.insert(id, system_id);
            }
            
        });
    }
}