use std::{any::{Any, TypeId}, collections::HashMap, marker::PhantomData};

use crate::{prelude::{AssetCheckers, InitializedStagers, RollDownCheckers}, systems::{check_roll_down, initialize_asset_structure}, traits::{FromStructure, InnerTarget, Structure}};
use bevy_asset::prelude::*;
use bevy_ecs::{component::{ComponentHooks, ComponentId, StorageType}, prelude::*, world::DeferredWorld};
use bevy_log::warn;
use bevy_hierarchy::prelude::*;
use bevy_reflect::DynamicTypePath;
use bevy_serialization_core::prelude::mesh::{MeshFlag3d, MeshWrapper};
use bevy_state::commands;




/// Take inner new_type and add components to this components entity from [`FromStructure`]
#[derive(Clone)]
pub struct RequestStructure<T: FromStructure + Sync + Send + Clone + 'static>(pub T);

impl<T: FromStructure + Sync + Send + Clone + 'static> Component for RequestStructure<T> {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    fn register_component_hooks(_hooks: &mut ComponentHooks) {
        _hooks.on_add(|mut world, e, id| {
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
                Structure::Children(bundles, split) => {
                    let mut children = Vec::new();
                    for bundle in bundles {
                        
                        let child = world.commands().spawn(bundle).id();
                        if !split.0 {
                            world.commands().entity(e).add_child(child);

                        }
                        children.push(child);
                    }
                    {
                        let mut initialized_stagers = world.get_resource_mut::<InitializedStagers>().unwrap();

                        let previous_result = initialized_stagers.0.get_mut(&e);

                        match previous_result {
                            Some(res) => res.push((id, children)),
                            None => {
                                initialized_stagers.0.insert(e, vec![(id, children)]);
                            },
                        }
                        //initialized_stagers.0.insert(e, v)
                        // let mut initialized_children = world.get_resource_mut::<InitializedChildren>().unwrap();
                        // if let Some(old_val) = initialized_children.0.insert(e, children) {
                        //     warn!("old value replaced for InitializedChildren, Refactor this to work with multiple  RequestStructur::Children to prevent bugs");
                        // }
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
                Structure::Children(bundles, split) => {
                    let mut children = Vec::new();
                    for bundle in bundles {
                        let child = world.commands().spawn(bundle).id();
                        if !split.0 {
                            world.commands().entity(e).add_child(child);

                        }
                        children.push(child);
            
                    }
                    {
                        let mut initialized_stagers = world.get_resource_mut::<InitializedStagers>().unwrap();

                        let previous_result = initialized_stagers.0.get_mut(&e);

                        match previous_result {
                            Some(res) => res.push((id, children)),
                            None => {
                                initialized_stagers.0.insert(e, vec![(id, children)]);
                            },
                        }
                        // let mut initialized_children = world.get_resource_mut::<InitializedChildren>().unwrap();
                        // if let Some(old_val) = initialized_children.0.insert(e, children) {
                        //     warn!("old value replaced for InitializedChildren, Refactor this to work with multiple  RequestStructur::Children to prevent bugs");
                        // }
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

/// staging component for resolving one component from another.
/// useful for bundles where the context for what something is has to be resolved later
#[derive(Clone)]
pub enum Resolve<T, U> {
    One(T),
    Other(U)
}

impl<T: Component + Clone, U: Component + Clone> Component for Resolve<T, U> {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    fn register_component_hooks(_hooks: &mut bevy_ecs::component::ComponentHooks) {
        _hooks.on_add(|mut world, e, id| {
            let comp = {
                match world.entity(e).get::<Self>() {
                    Some(val) => val.clone(),
                    None => {
                        warn!("could not get resolve on: {:#}", e);
                        return
                    },
                }
            };
            match comp {
                Resolve::One(one) => {world.commands().entity(e).insert(one);},
                Resolve::Other(other) => {world.commands().entity(e).insert(other);},
            }
            world.commands().entity(e).remove::<Self>();
        });
    }
}

/// staging component to roll down component to all children. 
#[derive(Clone)]
pub struct RollDown<T: Component>(
    pub T
    //, TypeId
);

impl<T: Component + Clone> Component for RollDown<T> {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;
    
    fn register_component_hooks(_hooks: &mut ComponentHooks) {
        _hooks.on_add(|mut world, e, id| {
            let rolldown_checkers = world.get_resource_mut::<RollDownCheckers>().unwrap();
            if !rolldown_checkers.0.contains_key(&id) {
                warn!("Adding rolldown system for {:#?}", id);
                let system_id = {
                    world.commands().register_system(check_roll_down::<T>)
                };
                let mut asset_checkers = world.get_resource_mut::<RollDownCheckers>().unwrap();

                asset_checkers.0.insert(id, system_id);
                return
            }
            let comp = {
                match world.entity(e).get::<Self>() {
                    Some(val) => val.clone(),
                    None => {
                        warn!("could not get RollDown<T> on: {:#}", e);
                        return
                    },
                }
            };
            let children = {
                let Some(children) = world.entity(e).get::<Children>() else {
                    warn!("No children for {:#}, skipping Rolldown<T>", e);
                    return
                };
                children.to_vec()
            };
            for child in children.iter() {
                //let ids = world.register_component()
                //warn!("rolling down to {:#}", child);
                world.commands().entity(child.clone()).insert(comp.0.clone());
            }

            world.commands().entity(e).remove::<Self>();
        });
    }
}

/// staging component to split off given component from its parent. 
pub struct SplitOff<T: Component + Clone>(pub T);

impl<T: Component + Clone> Component for SplitOff<T> {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    fn register_component_hooks(_hooks: &mut ComponentHooks) {
        _hooks.on_add(|mut world, e, id| {
            let comp = {
                match world.entity(e).get::<Self>() {
                    Some(val) => val.0.clone(),
                    None => {
                        warn!("could not get RollDown<T> on: {:#}", e);
                        return
                    },
                    
                }
            };

            world.commands().spawn(
                comp
            );
            world.commands().entity(e).remove::<Self>();
        });
    }
}