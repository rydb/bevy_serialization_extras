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
use crate::traits::Wrapper;


use moonshine_save::save::*;
use super::components::*;
use std::any::TypeId;
use std::path::PathBuf;
use std::collections::HashMap;


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

/// collect entities with `ModelFlag` that don't have meshes, and spawn their meshes.  
pub fn spawn_models(
    unspawned_models_query: Query<(Entity, &ModelFlag, &Transform), Without<Handle<Mesh>>>,
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assset_server: Res<AssetServer>,
    //transform_query: Query<&Transform>,
) {
    for (e, model, trans) in unspawned_models_query.iter() {
        println!("spawning model");
        let mesh_check: Option<Mesh> = match model.geometry.clone() {
            Geometry::Primitive(variant) => Some(variant.into()), 
            Geometry::Mesh { filename, .. } => {
                println!("attempting to load mesh: {:#?}", filename);
                meshes.get(&assset_server.load(filename))}.cloned()
        }; 
        if let Some(mesh) = mesh_check {
            let mesh_handle = meshes.add(mesh);

            let material_handle = materials.add(model.material.clone());
            //(*model).serialize()
            commands.entity(e)
            .insert(
                (
                PbrBundle {
                    mesh: mesh_handle,
                    material: material_handle,
                    transform: *trans,
                    ..default()
                }, // add mesh
                MakeSelectableBundle::default(), // makes model selectable 
                Unload, // marks entity to unload on deserialize
            )
            )

       
            ;
            // match model.physics {
            //     Physics::Dynamic => {
            //         commands.entity(e).insert(PhysicsBundle::default());
            //     },
            //     Physics::Fixed => {
            //         commands.entity(e).insert(
            //             PhysicsBundle {
            //             rigid_body: RigidBody::Fixed,
            //             ..default() 
            //         }
            //         );
            //     }
            // }
        } else {
            println!("load attempt failed for this mesh, re-attempting next system call");
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
