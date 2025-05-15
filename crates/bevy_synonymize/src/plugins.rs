use std::{any::{type_name, TypeId}, marker::PhantomData};

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_asset::prelude::*;
use bevy_log::warn;
use bevy_pbr::StandardMaterial;
use crate::{prelude::{material::{Material3dFlag, MeshMaterial3dRepr}, mesh::Mesh3dFlag, InitializedSynonyms}, traits::{AssetSynonymTarget, SynonymPaths}};
use crate::{systems::{desynonymize_assset, desynonymize, synonymize, try_synonymize_asset}, traits::{AssetState, ComponentSynonym}};



/// plugin for converitng between synonymous components. 
pub struct SynonymizeComponent<T: ComponentSynonym> {
    thing: PhantomData<fn() -> T>,
}

impl<T: ComponentSynonym> Default for SynonymizeComponent<T> {
    fn default() -> Self {
        Self {
            thing: Default::default(),
        }
    }
}

impl<T: ComponentSynonym> Plugin for SynonymizeComponent<T> {
    fn build(&self, app: &mut App) {
        //TODO: Move this to new crate
        //skip_serializing::<T::SynonymTarget>(app);
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
        //         let target = T::SynonymTarget::from(&comp);
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
pub struct SynonymizeAsset<T: AssetSynonymTarget + 'static> {
    thing: PhantomData<fn() -> T>,
}

impl<T: AssetSynonymTarget> Plugin for SynonymizeAsset<T> {
    fn build(&self, app: &mut App) {
        //TODO: Move this to new crate
        //skip_serializing::<T::SynonymTarget>(app);

        let synonym_id = TypeId::of::<T::Synonym>();
        let initializing_repr = type_name::<T>().to_string();
        if let Some(mut initialized_synonyms) = app.world_mut().get_resource_mut::<InitializedSynonyms>() {
            if let Some(first_instance) = initialized_synonyms.get(&synonym_id) {
                panic!("multi-initialization found for {:#?}. this is not allowed. First instance {:#}, Second instance {:#}", type_name::<T::Synonym>(), first_instance, initializing_repr)
            }
            initialized_synonyms.insert(synonym_id, initializing_repr);
        } else {
            let mut new_map = InitializedSynonyms::default();
            new_map.insert(synonym_id, initializing_repr);
            app.world_mut().insert_resource(new_map);
        };
        
        app.add_systems(
            PreUpdate,
            (try_synonymize_asset::<T>, desynonymize_assset::<T>).chain(),
        );

        app.register_type::<T::Synonym>()
            .world_mut()
            .register_component_hooks::<T::Synonym>()
            .on_add(|mut world, hook_context| {
                let comp = {
                    match world.entity(hook_context.entity).get::<T::Synonym>() {
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
                                    type_name::<Assets<T::AssetType>>()
                                );
                                return;
                            };
                            let asset = T::from_synonym(pure);
                            assets.add(asset)
                        }
                    }
                };

                let componentized_asset = T::Target::from(handle);
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
        app
        .add_plugins(SynonymizeAsset::<MeshMaterial3dRepr<StandardMaterial>>::default())
        ;
    }
}