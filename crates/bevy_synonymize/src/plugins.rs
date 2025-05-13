use std::{any::{type_name, TypeId}, marker::PhantomData};

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_asset::prelude::*;
use bevy_log::warn;
use crate::prelude::{material::Material3dFlag, mesh::Mesh3dFlag, AssetType};
use crate::{systems::{desynonymize_assset, desynonymize, synonymize, try_synonymize_asset}, traits::{AssetState, AssetWrapper, ComponentWrapper}};



/// plugin for converitng between synonymous components. 
pub struct SynonymizeComponent<T: ComponentWrapper> {
    thing: PhantomData<fn() -> T>,
}

impl<T: ComponentWrapper> Default for SynonymizeComponent<T> {
    fn default() -> Self {
        Self {
            thing: Default::default(),
        }
    }
}

impl<T: ComponentWrapper> Plugin for SynonymizeComponent<T> {
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
            (synonymize::<T>, desynonymize::<T>).chain(),
        );
    }
}


/// plugin for converting between synonymous asset component newtypes.
#[derive(Default)]
pub struct SynonymizeAsset<T: AssetWrapper> {
    thing: PhantomData<fn() -> T>,
}

impl<T: AssetWrapper> Plugin for SynonymizeAsset<T> {
    fn build(&self, app: &mut App) {
        //TODO: Move this to new crate
        //skip_serializing::<T::WrapperTarget>(app);

        app.add_systems(
            PreUpdate,
            (try_synonymize_asset::<T>, desynonymize_assset::<T>).chain(),
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

/// base Synonymizations for this library.
pub struct SynonymizeBasePlugin;

impl Plugin for SynonymizeBasePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SynonymizeAsset::<Material3dFlag>::default())
            .add_plugins(SynonymizeAsset::<Mesh3dFlag>::default());
    }
}