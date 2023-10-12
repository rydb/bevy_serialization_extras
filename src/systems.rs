use bevy::reflect::TypeUuid;
use bevy::prelude::*;

use crate::traits::*;

use moonshine_save::save::*;
use std::any::TypeId;
use std::collections::HashMap;
use bevy::asset::Asset;


pub fn serialize_for<Thing, WrapperThing>(
    thing_query: Query<(Entity, &Thing)>,
    mut commands: Commands,
)
    where
        Thing: Component,
        WrapperThing: Component + for<'a> From<&'a Thing>  
{
    for (e, f) in thing_query.iter() {
        commands.entity(e).insert(

            WrapperThing::from(f)
        );
    }
}

pub fn try_serialize_asset_for<Thing, WrapperThing> (
    things: ResMut<Assets<Thing>>,
    thing_query: Query<(Entity, &Handle<Thing>)>,
    mut commands: Commands,
)// -> bool
    where
        Thing: Asset,
        WrapperThing: Component + for<'a> From<&'a Thing> 
{
    for (e, thing_handle) in thing_query.iter() {
        match things.get(thing_handle) {
            Some(thing) => {
                commands.entity(e).insert(
                    WrapperThing::from(thing)
                );
                //return true
            },
            None => {
                //return false
            }
        }
    }
    //return true;
}

pub fn deserialize_wrapper_for<WrapperThing, Thing> (
    mut things: ResMut<Assets<Thing>>,
    wrapper_thing_query: Query<(Entity, &WrapperThing), Without<Handle<Thing>>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) 
    where
        WrapperThing: Component,
        Thing: Asset + TypeUuid + for<'a> Unwrap<&'a WrapperThing>,
{
    for (e, wrapper_thing) in wrapper_thing_query.iter() {
        let thing_fetch_attempt = Thing::unwrap(wrapper_thing);
    
        match thing_fetch_attempt {
            Ok(thing) => {
               let thing_handle = things.add(thing);
                commands.entity(e).insert(
                    thing_handle
                );
            }
            Err(file_path) => {
                let thing_handle: Handle<Thing> = asset_server.load(file_path);
                commands.entity(e).insert(
                    thing_handle
                );
            }
        }
    }
}

pub fn deserialize_asset_for<WrapperThing, Thing> (
    mut things: ResMut<Assets<Thing>>,
    wrapper_thing_query: Query<(Entity, &WrapperThing), Without<Handle<Thing>>>,
    mut commands: Commands,
) 
    where
        WrapperThing: Component,
        Thing: Asset + for<'a> From<&'a WrapperThing>,
{
    for (e, wrapper_thing) in wrapper_thing_query.iter() {
        let thing = Thing::from(wrapper_thing);
        let thing_handle = things.add(thing);

        commands.entity(e).insert(
            thing_handle
        );
    }
}

pub fn deserialize_for<WrapperThing, Thing>(
    wrapper_thing_query: Query<(Entity, &WrapperThing), Without<Thing>>,
    mut commands: Commands,
) 
    where
        Thing: Component + for<'a> From<&'a WrapperThing>,
        WrapperThing: Component  
{
    for (e, f) in wrapper_thing_query.iter() {
        commands.entity(e).insert(
            Thing::from(f)
        );
    }
}


/// collects all components in the world, and cross references them with the type registry. If a component is not in the type registry, label it in
/// the 'serializable check" window
pub fn list_unserializable(
    world: &mut World,
){
    let mut enetities_to_save = world.query_filtered::<Entity, With<Save>>();
    

    let type_registry = world.resource::<AppTypeRegistry>();

    let mut saved_component_types = HashMap::new();
    for e in enetities_to_save.iter(&world) {
        for component in world.entity(e).archetype().components() {

            let comp_info = world.components().get_info(component).unwrap();
            saved_component_types.insert(comp_info.type_id().unwrap(), comp_info.name().to_owned());
        }
    }

    let registered_types = type_registry.read().iter()
    .map(|id| {
        let type_id = id.type_id();

        return (type_id, id.type_name().to_owned())
    })
    .collect::<HashMap<TypeId, String>>();
    
    println!("Listing imports required for adding unregistered types to type registry: ");
    for item in saved_component_types.keys() {
        if registered_types.contains_key(item) == false {
            println!("use {:#}", saved_component_types[item])
        }
    }
    println!("listing .register_type::<T>'s for unregistered types. copy and paste this into app  ");
    for item in saved_component_types.keys() {
        //println!("listing component types marked to save {:#?}", saved_component_types[item]);
        if registered_types.contains_key(item) == false {
            println!(".register_type::<{:#}>()", 
            saved_component_types[item].split("::")
            .collect::<Vec<_>>()
            .last()
            .unwrap())
        }
    }
}



pub fn check_for_save_keypress(
    keys: Res<Input<KeyCode>>,
) -> bool{
    if keys.just_pressed(KeyCode::AltRight) {
        return true
    } else {
        return false
    }
}

pub fn check_for_load_keypress(
    keys: Res<Input<KeyCode>>,
) -> bool{
    if keys.just_pressed(KeyCode::AltLeft) {
        return true
    } else {
        return false
    }
}

pub fn add_computed_visiblity(
    computed_visiblity_query: Query<Entity, Without<ComputedVisibility>>,
    mut commands: Commands,
    
) {
    for e in computed_visiblity_query.iter() {
        commands.entity(e).insert(ComputedVisibility::default());
    }
}

