use std::{any::type_name, ops::Deref};

use bevy_asset::{Asset, Assets};
use bevy_ecs::{component::{ComponentHooks, ComponentId, StorageType}, prelude::*, system::SystemId};
use bevy_log::warn;
use bevy_reflect::Reflect;
use bevy_utils::HashMap;
use derive_more::derive::From;

use crate::{prelude::{WrapAssetDeserializers, WrapAssetSerializers, WrapCompDeserializers, WrapCompSerializers}, systems::{deserialize_asset_for, deserialize_for, serialize_for, try_serialize_asset_for}, traits::{AssetKind, AssetWrapper, ComponentWrapper}};



pub type AssetType<T> = <<T as AssetWrapper>::AssetTarget as AssetKind>::AssetKind;


/// Wrapper component around an Comp(Handle<Asset<T>>)
#[derive(Reflect, From, Clone)]
pub struct WrapAsset<T: Reflect + AssetWrapper + Clone>(pub T);

impl<T: Reflect + AssetWrapper + Clone> Component for WrapAsset<T> {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    
    fn register_component_hooks(_hooks: &mut ComponentHooks) {
        //type ComponentAssetWrapper<T> = <T as AssetWrapper>::AssetTarget;

        _hooks.on_add(|mut world, e, id| {
            let comp = {
                match world.entity(e).get::<Self>() {
                    Some(val) => val.clone(),
                    None => {
                        warn!("could not get {:#?} on: {:#}", type_name::<Self>(), e);
                        return
                    },
                }
            };
            // Get the asset server for the assetkind this wrapper refers to.
            let handle = {
                if let Some(mut assets) = world.get_resource_mut::<Assets<AssetType<T>>>() {
                    let asset = AssetType::<T>::from(comp.0);
                    assets.add(asset)
                } else {
                    warn!("no Assets<T> found for {:#}", type_name::<Assets<AssetType<T>>>());
                    return
                }
            };


            let componentized_asset = T::AssetTarget::from(handle);
            world.commands().entity(e).insert(componentized_asset);

            if world.get_resource::<WrapAssetSerializers>().unwrap().0.contains_key(&id) == false {
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


            //let handle = assets.add(T::AssetTarget::from(comp.0));
            //let handle = T::Target::from(comp.0.clone());
        });
    }
}

#[derive(Clone, From)]
pub struct WrapComp<T: Reflect + ComponentWrapper + Clone>(pub T);

impl<T: Reflect + ComponentWrapper + Clone> Component for WrapComp<T>{
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
            let target = T::Target::from(comp.0.clone());
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