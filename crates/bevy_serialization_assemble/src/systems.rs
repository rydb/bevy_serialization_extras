use crate::components::{RequestAssetStructure, RollDown, RollDownIded};
use crate::prelude::*;
use crate::resources::{AssetSpawnRequestQueue, RequestFrom};
use crate::traits::{FromStructure, InnerTarget, IntoHashMap};
use bevy_asset::prelude::*;
use bevy_core::Name;
use bevy_ecs::component::{ComponentId, Components};
use bevy_ecs::{prelude::*, query::QueryData};
use bevy_hierarchy::{BuildChildren, Children};
use bevy_log::prelude::*;
use std::collections::VecDeque;
use std::fmt::Debug;

/// proxy system for checking load status of assets for component hooks.
pub fn run_asset_status_checkers(
    asset_systems: Res<AssetCheckers>, mut commands: Commands
) {
    for (_, system) in asset_systems.0.iter() {
        // run systems for each asset type
        commands.run_system(*system);
    }
}

pub fn run_rolldown_checkers(
    rolldown_systems: Res<RollDownCheckers>,
    mut commands: Commands
) {
    for (_, system) in rolldown_systems.0.iter() {
        // run systems for each asset type
        commands.run_system(*system);
    }
}

pub fn check_roll_down<T: Clone + Component>(
    initialized_children: Res<InitializedStagers>,
    rolldowns: Query<(Entity, &Name, &RollDownIded<T>)>,
    test: Query<EntityRef>,
    decendents: Query<&Children>,
    mut commands: Commands,
    components: &Components,
){
    for (e, name, rolldown) in &rolldowns {
        // let Some(ids) = initialized_children.0.get(&e) else {
        //     return
        // };
        let Ok(children) = decendents.get(e) else {
            return
        };
        // for child in children {
            
        // }

        for child in children {
            let Ok(e_ref) = test.get(*child) else {
                return
            };
            let check_list = e_ref.archetype().components().collect::<Vec<_>>();
            // println!("Name: {:#}, Filter ids: {:#?} registered: {:#?}", 
            //     name, 
            //     check_list.iter().map(|n| components.get_name(*n).unwrap_or("???")).collect::<Vec<_>>(), 
            //     rolldown.1
            // );

            //if components.any(|n| rolldown.1.contains(n));
            if rolldown.1.iter().any(|n| check_list.contains(n)) {
                commands.entity(*child).insert(rolldown.0.clone());
                commands.entity(e).remove::<RollDownIded<T>>();
                println!("finished rolling down");
            }
        }
        // println!("Filter ids: {:#?} registered: {:#?}", ids, rolldown.1);
        // // for (child, components) in ids.iter()
        // // .filter(|(id, components)| rolldown.1
        // // .iter()
        // // .any(|n| components.contains(n))) {
        // //     for child in children {
        // //         println!("rolling down");
        // //         commands.entity(*child).insert(rolldown.0.clone());
        // //     }
        // //     println!("finished rolling down");
        // //     commands.entity(e).remove::<RollDown<T>>();
        // }
    }
    
}

pub fn initialize_asset_structure<T>(
    //events: EventReader<AssetEvent<T::Inner>>,
    asset_server: Res<AssetServer>,
    requests: Query<(Entity, &RequestAssetStructure<T>)>,
    assets: Res<Assets<T::Inner>>,
    mut commands: Commands,

) 
    where
        T: Clone + From<T::Inner> + InnerTarget + FromStructure + Send + Sync + 'static,
        T::Inner: Asset + Clone
{
    //println!("checking initialize_asset structures...");
    for (e, request) in &requests {
        //println!("checking load status for... {:#}", e);
        let handle = match request {
            RequestAssetStructure::Handle(handle) => {handle},
            _ => {
                warn!("no handle??");
                return;
            }
        };
        if asset_server.is_loaded(handle) {
            let Some(asset) = assets.get(handle) else {
                warn!("handle for Asset<T::inner> reports being loaded by asset not available?");
                return;
            };
            println!("Asset loaded for {:#}", e);
            // upgrading handle to asset
            commands.entity(e).remove::<RequestAssetStructure<T>>();
            commands.entity(e).insert(RequestAssetStructure::Asset(T::from(asset.clone())));
        } else {
            let status = asset_server.load_state(handle);
            println!("Asset unloaded: REASON: {:#?}", status);
        }
    }
}

//FIXME: implement this properly. Once an asset builder that could use this exists.
pub fn serialize_structures_as_assets<ThingSet, AssetType>(//thing_query: Query<ThingSet>,
    //asset_server: Res<AssetServer>,
    //mut assets: ResMut<Assets<AssetType>>,
)
where
    ThingSet: QueryData,
    AssetType: Asset + for<'w, 's> IntoHashMap<Query<'w, 's, ThingSet>> + Clone,
{
    // let assets_list: HashMap<String, AssetType> = IntoHashMap::into_hashmap(thing_query);
    // //println!("assets list is {:#?}", assets_list.keys());
    // for (name, uncached_asset) in assets_list.iter() {
    //     asset_server.add(uncached_asset.clone());
    //     //LazyDeserialize::deserialize(uncached_asset.clone(), asset_handle.path());
    // }
}

pub fn deserialize_assets_as_structures<TargetAsset>(
    thing_assets: Res<Assets<TargetAsset>>,
    mut asset_spawn_requests: ResMut<AssetSpawnRequestQueue<TargetAsset>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) where
    TargetAsset: Asset + Clone + FromStructure,
{
    let mut failed_requests = VecDeque::new();
    while asset_spawn_requests.requests.len() != 0 {
        if let Some(request) = asset_spawn_requests.requests.pop_front() {
            match request.source.clone() {
                RequestFrom::AssetServerPath(path) => {
                    trace!("processing request from path: {:#?}", path.clone());
                    let asset_handle = asset_server.load(path);

                    //passes off request to the front of the queue for the next update as the asset is likely to not have loaded yet until next update.
                    let mut unready_asset_request = request;
                    //turns asset request into assset id as now former "file" path is now a part of the Res<Assets<T>>
                    unready_asset_request.source = RequestFrom::AssetHandle(asset_handle);
                    failed_requests.push_front(unready_asset_request);
                }
                RequestFrom::AssetHandle(handle) => {
                    trace!("processing request from assetid {:#?}", handle);
                    trace!("failed load attempts: {:#?}", request.failed_load_attempts);
                    if let Some(asset) = thing_assets.get(&handle) {
                        let components = FromStructure::components(asset.clone());
                        
                        match components {
                            crate::traits::Structure::Root(bundle) => {
                                commands.spawn(
                                    bundle
                                );   
                            },
                            crate::traits::Structure::Children(children, split) => {
                                
                                let root = commands.spawn_empty().id();
                                for bundle in children {
                                    let child = commands.spawn(bundle).id();
                                    commands.entity(root).add_child(child);
                        
                                }
                            },
                        }
                    } else {
                        let mut failed_request = request;
                        failed_request.failed_load_attempts += 1;
                        failed_requests.push_back(failed_request);
                    }
                }
            }
        }
    }
    // re-add failed requests to asset_spawn_requests, as there could be a chance the asset just hasn't loaded yet.
    asset_spawn_requests.requests.append(&mut failed_requests);
    // for (asset_id, asset) in thing_assets.iter() {
    //     //FromStructure::into_structures(&mut commands, asset, asset_load_request.requests.clone());
    // }
}
