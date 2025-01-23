use crate::components::RequestAssetStructure;
use crate::prelude::*;
use crate::resources::{AssetSpawnRequestQueue, RequestFrom};
use crate::traits::{FromStructure, InnerTarget, IntoHashMap};
use bevy_asset::prelude::*;
use bevy_ecs::{prelude::*, query::QueryData};
use bevy_hierarchy::BuildChildren;
use bevy_log::prelude::*;
use std::collections::VecDeque;

/// proxy system for checking load status of assets for component hooks.
pub fn run_asset_status_checkers(
    asset_systems: Res<AssetCheckers>, mut commands: Commands
) {
    for (_, system) in asset_systems.0.iter() {
        // run systems for each asset type
        commands.run_system(*system);
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
        let handle = match request {
            RequestAssetStructure::Handle(handle) => {handle},
            _ => {
                //warn!("no handle??");
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
                            crate::traits::Structure::Children(children) => {
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
