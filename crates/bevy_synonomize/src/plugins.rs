use std::{any::{type_name, TypeId}, marker::PhantomData};

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_asset::prelude::*;
use bevy_log::warn;
use crate::prelude::AssetType;
use crate::{systems::{deserialize_asset_for, deserialize_for, serialize_for, try_serialize_asset_for}, traits::{AssetState, AssetWrapper, ComponentWrapper}};



/// plugin for serialization for WrapperComponent -> Component, Component -> WrapperComponent
pub struct SerializeComponentFor<T: ComponentWrapper> {
    thing: PhantomData<fn() -> T>,
}

impl<T: ComponentWrapper> Default for SerializeComponentFor<T> {
    fn default() -> Self {
        Self {
            thing: Default::default(),
        }
    }
}

impl<T: ComponentWrapper> Plugin for SerializeComponentFor<T> {
    fn build(&self, app: &mut App) {
        //TODO: Move this to new crate
        //skip_serializing::<T::WrapperTarget>(app);
        app.world_mut();
        // .register_component_hooks::<T>().on_insert(|mut world, e, id| {
        //         let comp = {
        //             match world.entity(e).get::<T>() {
        //                 Some(val) => val,
        //                 None => {
        //                     warn!("could not get {:#?} on: {:#}", type_name::<Self>(), e);
        //                     return
        //                 },
        //             }
        //         };
        //         let target = T::WrapperTarget::from(&comp);
        //         world.commands().entity(e).insert(target);
        //     });

        app.register_type::<T>().add_systems(
            PreUpdate,
            (serialize_for::<T>, deserialize_for::<T>).chain(),
        );
    }
}


/// plugin for serialization for WrapperComponent -> Asset, Asset -> WrapperComponent
#[derive(Default)]
pub struct SerializeAssetFor<T: AssetWrapper> {
    thing: PhantomData<fn() -> T>,
}

impl<T: AssetWrapper> Plugin for SerializeAssetFor<T> {
    fn build(&self, app: &mut App) {
        //TODO: Move this to new crate
        //skip_serializing::<T::WrapperTarget>(app);

        app.add_systems(
            PreUpdate,
            (try_serialize_asset_for::<T>, deserialize_asset_for::<T>).chain(),
        );

        app.register_type::<T>()
            .world_mut()
            .register_component_hooks::<T>()
            .on_add(|mut world, hook_context| {
                let comp = {
                    match world.entity(hook_context.entity).get::<T>() {
                        Some(val) => val,
                        None => {
                            warn!(
                                "could not get {:#?} on: {:#}",
                                type_name::<Self>(),
                                hook_context.entity
                            );
                            return;
                        }
                    }
                };

                let handle = {
                    match comp.asset_state() {
                        AssetState::Path(path) => {
                            let Some(asset_server) = world.get_resource::<AssetServer>() else {
                                warn!("could not get asset server?");
                                return;
                            };
                            asset_server.load(path)
                        }
                        AssetState::Pure(pure) => {
                            let Some(assets) = world.get_resource::<AssetServer>() else {
                                warn!(
                                    "no mut Assets<T> found for {:#}",
                                    type_name::<Assets<AssetType<T>>>()
                                );
                                return;
                            };
                            let asset = AssetType::<T>::from(pure);
                            assets.add(asset)
                        }
                    }
                };

                let componentized_asset = T::WrapperTarget::from(handle);
                world
                    .commands()
                    .entity(hook_context.entity)
                    .insert(componentized_asset);
            });
    }
}