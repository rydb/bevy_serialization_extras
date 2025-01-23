use std::marker::PhantomData;

use crate::{prelude::AssetCheckers, systems::initialize_asset_structure, traits::{FromStructure, InnerTarget, Structure}};
use bevy_asset::prelude::*;
use bevy_ecs::{component::{ComponentHooks, ComponentId, StorageType}, prelude::*, world::DeferredWorld};
use bevy_log::warn;
use bevy_hierarchy::prelude::*;


/// Take inner new_type and add components to this components entity from [`FromStructure`]
pub struct RequestStructure<T>(pub T);

impl<T: FromStructure + Sync + Send + Clone + 'static> Component for RequestStructure<T> {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    fn register_component_hooks(_hooks: &mut ComponentHooks) {
        _hooks.on_add(|mut world, e, _| {
            let comp = {
                let comp = match world.entity(e).get::<Self>() {
                    Some(val) => val,
                    None => {
                        warn!("could not get FromStructure on: {:#}", e);
                        return
                    },
                };
                comp.0.clone()
            };

            match FromStructure::components(comp) {
                Structure::Root(bundle) => {
                    world.commands().entity(e).insert(bundle);
                },
                Structure::Children(children) => {
                    for bundle in children {
                        let child = world.commands().spawn(bundle).id();
                        world.commands().entity(e).add_child(child);
            
                    }
                    world.commands().entity(e).remove::<Self>();
                },
            }
            world.commands().entity(e).remove::<Self>();
        });
    }
}

#[derive(Clone, Debug)]
// #[reflect(no_field_bounds)]
pub enum RequestAssetStructure<T> 
    where
        T: From<T::Inner> + InnerTarget,
        T::Inner: Asset
{
    Path(String),
    Handle(Handle<T::Inner>),
    Asset(T)
}

impl<T> Component for RequestAssetStructure<T>
    where
        T: Clone + From<T::Inner> + InnerTarget + FromStructure+ Send + Sync + 'static,
        T::Inner: Asset + Clone
{
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    fn register_component_hooks(_hooks: &mut ComponentHooks) {
        _hooks.on_add(|mut world, e, id| {
            let asset = {
                let path = match world.entity(e).get::<Self>() {
                    Some(val) => val.clone(),
                    None => {
                        warn!("could not get RequestAssetStructure on: {:#}", e);
                        return
                    },
                };
                let asset = match path {
                    RequestAssetStructure::Path(path) => {
                        let handle = world.load_asset(path);
                        //upgrade path to asset handle.
                        world.commands().entity(e).remove::<Self>();
                        world.commands().entity(e).insert(Self::Handle(handle));
                        return;
                    },
                    RequestAssetStructure::Handle(_) => {
                        
                        let add_system = 
                        {
                            let asset_checkers = world.get_resource_mut::<AssetCheckers>().unwrap();
                            //let type_path = T::Inner::type_path();

                            //let comp_id = T::
                            if asset_checkers.0.contains_key(&id) {
                                //asset_checkers.0.insert(id, type_path.to_string());
                                false
                            } else {
                                true
                            }
                        };

                        if add_system {
                            let system_id = {
                                world.commands().register_system(initialize_asset_structure::<T>)
                            };
                            let mut asset_checkers = world.get_resource_mut::<AssetCheckers>().unwrap();

                            asset_checkers.0.insert(id, system_id);
                        }
                        return;
                    },
                    RequestAssetStructure::Asset(asset) => {
                        //world.commands().entity(e)
                        println!("got asset");
                        asset
                    }
                };
                asset
            };
            println!("populating structure for {:#}", e);
            
            
            match FromStructure::components(asset) {
                Structure::Root(bundle) => {
                    world.commands().entity(e).insert(bundle);
                },
                Structure::Children(children) => {
                    for bundle in children {
                        let child = world.commands().spawn(bundle).id();
                        world.commands().entity(e).add_child(child);
            
                    }
                    world.commands().entity(e).remove::<Self>();
                },
            }
            world.commands().entity(e).remove::<Self>();
        });
    }
}

/// Staging component for optional components. Is split open into inner component if it exists.
pub struct Maybe<T: Component>(pub Option<T>);


/// A hook that runs whenever [`Maybe`] is added to an entity.
///
/// Generates a [`MaybeCommand`].
fn maybe_hook<B: Component>(mut world: DeferredWorld<'_>, entity: Entity, _component_id: ComponentId) {
    // Component hooks can't perform structural changes, so we need to rely on commands.
    world.commands().queue(MaybeCommand {
        entity,
        _phantom: PhantomData::<B>,
    });
}

struct MaybeCommand<B> {
    entity: Entity,
    _phantom: PhantomData<B>,
}

impl<B: Component> Command for MaybeCommand<B> {
    fn apply(self, world: &mut World) {
        let Ok(mut entity_mut) = world.get_entity_mut(self.entity) else {
            #[cfg(debug_assertions)]
            panic!("Entity with Maybe component not found");

            #[cfg(not(debug_assertions))]
            return;
        };

        let Some(maybe_component) = entity_mut.take::<Maybe<B>>() else {
            #[cfg(debug_assertions)]
            panic!("Maybe component not found");

            #[cfg(not(debug_assertions))]
            return;
        };

        if let Some(component) = maybe_component.0 {
            entity_mut.insert(component);
        }
    }
}

impl<T: Component> Component for Maybe<T> {
    const STORAGE_TYPE: bevy_ecs::component::StorageType = StorageType::SparseSet;

    fn register_component_hooks(_hooks: &mut bevy_ecs::component::ComponentHooks) {
        _hooks.on_add(maybe_hook::<T>);
    }
}