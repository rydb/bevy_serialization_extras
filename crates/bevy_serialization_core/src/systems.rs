use crate::{
    resources::{AssetSpawnRequestQueue, RequestFrom},
    traits::*,
};
// use bevy::{
//     ecs::query::{QueryData, WorldQuery},
//     prelude::*,
// };
use core::fmt::Debug;
use std::collections::VecDeque;

use bevy_ecs::{prelude::*, query::{QueryData, WorldQuery}};
use bevy_asset::prelude::*;
use bevy_render::prelude::*;

//FIXME: implement this properly. Once an asset builder that could use this exists.
pub fn serialize_structures_as_assets<ThingSet, AssetType>(
    //thing_query: Query<ThingSet>,
    //asset_server: Res<AssetServer>,
    //mut assets: ResMut<Assets<AssetType>>,
) where
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

// pub fn serialize_structures_as_resource<ThingSet, ThingResource> (
//     thing_query: Query<ThingSet>,
//     things_resource: ResMut<ThingResource>,
//     mut commands: Commands,
// )

//     where
//         ThingSet: WorldQuery,
//         ThingResource: Resource + for<'w, 's> From<Query<'w, 's, ThingSet>>,
// {
//     commands.insert_resource(
//         ThingResource::from(thing_query)
//     )
// }

pub fn deserialize_assets_as_structures<ThingAsset>(
    thing_assets: Res<Assets<ThingAsset>>,
    mut asset_spawn_requests: ResMut<AssetSpawnRequestQueue<ThingAsset>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) where
    ThingAsset: Asset + Clone + FromStructure + LazyDeserialize,
{
    let mut failed_requests = VecDeque::new();
    while asset_spawn_requests.requests.len() != 0 {
        if let Some(request) = asset_spawn_requests.requests.pop_front() {
            match request.source.clone() {
                RequestFrom::AssetServerPath(path) => {
                    log::trace!("processing request from path: {:#?}", path.clone());
                    let asset_handle = asset_server.load(path);

                    //passes off request to the front of the queue for the next update as the asset is likely to not have loaded yet until next update.
                    let mut unready_asset_request = request;
                    //turns asset request into assset id as now former "file" path is now a part of the Res<Assets<T>>
                    unready_asset_request.source = RequestFrom::AssetHandle(asset_handle);
                    failed_requests.push_front(unready_asset_request);
                }
                RequestFrom::AssetHandle(handle) => {
                    log::trace!("processing request from assetid {:#?}", handle);
                    log::trace!("failed load attempts: {:#?}", request.failed_load_attempts);
                    if let Some(asset) = thing_assets.get(&handle) {
                        FromStructure::into_entities(&mut commands, asset.clone(), request);
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

/// takes a component, and spawns a serializable copy of it on its entity
pub fn serialize_for<Thing, WrapperThing>(
    thing_query: Query<(Entity, &Thing), Without<WrapperThing>>,
    mut commands: Commands,
) where
    Thing: Component,
    WrapperThing: Component + for<'a> From<&'a Thing>,
{
    for (e, f) in thing_query.iter() {
        // entity may not exist when inserting component if entity is deleted in the same frame as this.
        // this checks to make sure it exists to prevent a crash.
        commands.entity(e).try_insert(WrapperThing::from(f));
    }
}

/// Takes a query based interpretation of thing(`thing` that is composted of several components), and decomposes it into a single component
pub fn deserialize_as_one<T, U>(
    mut commands: Commands,
    structure_query: Query<(Entity, T), Or<(Without<U>, Changed<T::ChangeCheckedComp>)>>,
) where
    T: QueryData + ChangeChecked,
    U: Component
        + Debug
        + for<'a, 'b> From<&'b <<T as QueryData>::ReadOnly as WorldQuery>::Item<'a>>,
{
    for (e, thing_query) in structure_query.into_iter() {
        let unwrapped_thing = U::from(&thing_query);
        //FIXME: This gets run very frequently, will need to figure out why that is
        log::trace!("On, {:?}, inserting {:?}", e, unwrapped_thing);
        commands.entity(e).try_insert(unwrapped_thing);
    }
}

/// takes an asset handle, and spawns a serializable copy of it on its entity
pub fn try_serialize_asset_for<Thing, WrapperThing>(
    things: ResMut<Assets<Thing>>,
    things_query: Query<(Entity, &Handle<Thing>), Or<(Changed<Handle<Thing>>, Without<WrapperThing>)>>,
    wrapper_things_query: Query<&WrapperThing>,
    mut commands: Commands,
)
// -> bool
where
    Thing: Asset,
    WrapperThing: Component + for<'a> From<&'a Thing> + PartialEq,
{
    for (e, thing_handle) in things_query.iter() {
        
        let Some(thing) = things.get(thing_handle) else {return};
        let new_wrapper_thing = WrapperThing::from(thing);
        let old_wrapper_thing = wrapper_things_query.get(e).unwrap_or(&new_wrapper_thing);

        // don't re-insert the same component
        if &new_wrapper_thing != old_wrapper_thing {
            log::trace!("changing Wrapperthing to match changed asset for {:#?}", e);
            commands.entity(e).try_insert(new_wrapper_thing);
        }
    }
    //return true;
}
/// takes a wrapper component, and deserializes it back into its unserializable asset handle varaint
pub fn deserialize_asset_for<WrapperThing, Thing>(
    mut things: ResMut<Assets<Thing>>,
    wrapper_thing_query: Query<
        (Entity, &WrapperThing),
        Or<(Changed<WrapperThing>, Without<Handle<Thing>>)>,
    >,
    things_query: Query<&Handle<Thing>>,
    mut commands: Commands,
) where
    WrapperThing: Component + for<'a> From<&'a Thing> + PartialEq,
    Thing: Asset + for<'a> From<&'a WrapperThing>,
{
    for (e, wrapper_thing) in wrapper_thing_query.iter() {
        log::trace!("converting wrapper thing {:#?}", e);


        let new_thing = Thing::from(wrapper_thing);

        // dont re-insert duplicates. This is to prevent change deadlock from insert causing a [`Changed`] chain
        let should_try_insert = {
            if let Ok(old_thing_handle) = things_query.get(e) {
                if let Some(old_thing) = things.get(old_thing_handle) {
                    let old_thing_as_wrapper = WrapperThing::from(old_thing);            
                    if wrapper_thing != &old_thing_as_wrapper {
                        true
                    } else {
                        false
                    }
                } else {true}
            } else {true}
        };
        if should_try_insert {
            let thing_handle = things.add(new_thing);
            commands.entity(e).try_insert(thing_handle);

        }

    }
}

/// takes a wrapper component, and attempts to deserialize it into its asset handle through [`Unwrap`]
/// this is components that rely on file paths.
pub fn deserialize_wrapper_for<WrapperThing, Thing>(
    mut things: ResMut<Assets<Thing>>,
    wrapper_thing_query: Query<
        (Entity, &WrapperThing),
        Or<(Without<Handle<Thing>>, Changed<WrapperThing>)>,
    >,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) where
    WrapperThing: Component,
    Thing: Asset + for<'a> Unwrap<&'a WrapperThing>,
{
    for (e, wrapper_thing) in wrapper_thing_query.iter() {
        let thing_fetch_attempt = Thing::unwrap(wrapper_thing);

        match thing_fetch_attempt {
            Ok(thing) => {
                let thing_handle = things.add(thing);
                commands.entity(e).try_insert(thing_handle);
            }
            Err(file_path) => {
                let thing_handle: Handle<Thing> = asset_server.load(file_path);
                commands.entity(e).try_insert(thing_handle);
            }
        }
    }
}

/// deserializes a wrapper component into its unserializable component variant.
pub fn deserialize_for<WrapperThing, Thing>(
    wrapper_thing_query: Query<
        (Entity, &WrapperThing),
        Or<(Without<Thing>, Changed<WrapperThing>)>,
    >,
    mut commands: Commands,
) where
    Thing: Component + for<'a> From<&'a WrapperThing>,
    WrapperThing: Component,
{
    for (e, f) in wrapper_thing_query.iter() {
        commands.entity(e).try_insert(Thing::from(f));
    }
}

/// adds computed visability to componnets that don't have it. this should probably be removed
/// at some point...
pub fn add_inherieted_visibility(
    computed_visiblity_query: Query<Entity, Without<InheritedVisibility>>,
    mut commands: Commands,
) {
    for e in computed_visiblity_query.iter() {
        commands
            .entity(e)
            .try_insert(InheritedVisibility::default());
    }
}

pub fn add_view_visibility(
    computed_visiblity_query: Query<Entity, Without<ViewVisibility>>,
    mut commands: Commands,
) {
    for e in computed_visiblity_query.iter() {
        commands.entity(e).try_insert(ViewVisibility::default());
    }
}
