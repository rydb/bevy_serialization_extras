use bevy::{prelude::*, reflect::TypeData};
use moonshine_save::prelude::Unload;

use bevy::ecs::component::ComponentId;
use bevy_rapier3d::prelude::RigidBody;
//use crate::body::robot::components::PhysicsBundle;
use bevy::ecs::query::ReadOnlyWorldQuery;
use super::components::Geometry;
use bevy_component_extras::components::*;
//use crate::urdf::urdf_loader::BevyRobot;
use crate::physics::components::PhysicsBundle;
use crate::traits::*;


use moonshine_save::save::*;
use super::components::*;
use std::any::TypeId;
use std::path::PathBuf;
use std::collections::HashMap;



///Takes a component that is ECSSerializable, and manages serialization for it.
pub fn serialize_for<T: ECSSerialize>(
    world: &mut World,
) {
    T::serialize(world);
}

pub fn deserialize_for<T: ECSDeserialize>(
    world: &mut World,
) {
   T::deserialize(world)
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

    for item in saved_component_types.keys() {
        if registered_types.contains_key(item) == false {
            println!("NOT IN TYPE REGISTRY {:#?}", saved_component_types[item])
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

pub fn save_into_file(path: impl Into<PathBuf>) -> SavePipeline {
    
    save::<With<Save>>
        .pipe(into_file(path.into()))
        .pipe(finish)
        .in_set(SaveSet::Save)
}


/// as a hot fix to deal with ComputedVisiblityFlags not being accessible by the type registry. This system manually adds ComputedVisibility to all Entities
/// which don't have one
pub fn add_computed_visiblity(
    computed_visiblity_query: Query<Entity, Without<ComputedVisibility>>,
    mut commands: Commands,
    
) {
    for e in computed_visiblity_query.iter() {
        commands.entity(e).insert(ComputedVisibility::default());
    }
}

pub fn save<Filter: ReadOnlyWorldQuery>(
    world: &World,
    serializable_querry: Query<Entity, Filter>,
) -> Saved {

    let mut builder = DynamicSceneBuilder::from_world(world);
    //this is a problematic component that needs to be manually added to everything due to ComputedVisiblityFlags not implementing reflect.
    builder.deny::<ComputedVisibility>(); 
    //you cant load meshes from handles, you need a wrapper component to save the geometry of the mesh
    builder.deny::<Handle<Mesh>>();
    // builder.allow::<ModelFlag>();
    // builder.allow::<Serializable>();

    builder.extract_entities(serializable_querry.iter());
    let scene = builder.build();
    Saved { scene }
}
