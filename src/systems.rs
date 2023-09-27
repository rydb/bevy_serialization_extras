use bevy::prelude::*;
use moonshine_save::prelude::Unload;

use bevy_rapier3d::prelude::RigidBody;
//use crate::body::robot::components::PhysicsBundle;
use bevy::ecs::query::ReadOnlyWorldQuery;
use super::components::Geometry;
use bevy_component_extras::components::*;
//use crate::urdf::urdf_loader::BevyRobot;
use crate::physics::components::PhysicsBundle;


use moonshine_save::save::*;
use super::components::*;
use std::path::PathBuf;

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

        let mesh_check: Option<Mesh> = match model.geometry.clone() {
            Geometry::Primitive(variant) => Some(variant.into()), 
            Geometry::Mesh { filename, .. } => {
                println!("attempting to load mesh: {:#?}", filename);
                meshes.get(&assset_server.load(filename))}.cloned()
        }; 
        if let Some(mesh) = mesh_check {
            let mesh_handle = meshes.add(mesh);

            let material_handle = materials.add(model.material.clone());
            
            // if let Ok(trans) = transform_query.get(e) {

            // }
            // add all components a deserialized model needs to be useful. 
            
            
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
            match model.physics {
                Physics::Dynamic => {
                    commands.entity(e).insert(PhysicsBundle::default());
                },
                Physics::Fixed => {
                    commands.entity(e).insert(
                        PhysicsBundle {
                        rigid_body: RigidBody::Fixed,
                        ..default() 
                    }
                    );
                }
            }
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
    
    //let mut world_copy = *world;
    // query for all components with special serializtion behaviour
    //let model_mesh_query = world.query_filtered::<Entity, With<Handle<Mesh>>>();
    
    let mut builder = DynamicSceneBuilder::from_world(world);
    
    // for e in builder.extract_entities(model_mesh_query.iter(&world)) {

    // }
    //this is a problematic component that needs to be manually added to everything due to ComputedVisiblityFlags not implementing reflect.
    builder.deny::<ComputedVisibility>(); 
    // builder.allow::<ModelFlag>();
    // builder.allow::<Serializable>();

    builder.extract_entities(serializable_querry.iter());
    let scene = builder.build();
    Saved { scene }
}

//pub fn save_models()

// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .add_plugins(WorldInspectorPlugin::new())
//         .add_plugins(SerializationPlugin)
//         .add_systems(Startup, setup)
//         .register_type::<FavoriteNumber>()
//         .register_type::<ModelVariant>()
//         //.register_type::<Option<Entity>>()
//         .run();
// }

// /// set up a simple 3D scene
// fn setup(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     commands.spawn(
//     PbrBundle {
//         mesh: meshes.add(shape::Plane::from_size(5.0).into()),
//         material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
//         ..default()
//     });
//     // mark cube to spawn
//     // commands.spawn(
//     // (

//     //     Save,
//     //     FavoriteNumber{favorite_number: 10},
//     //     ModelVariant::Cube,
//     // ));
//     // // cube
//     // commands.spawn(PbrBundle {
//     //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
//     //     material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
//     //     transform: Transform::from_xyz(0.0, 0.5, 0.0),
//     //     ..default()
//     // });
//     // light
//     commands.spawn(PointLightBundle {
//         point_light: PointLight {
//             intensity: 1500.0,
//             shadows_enabled: true,
//             ..default()
//         },
//         transform: Transform::from_xyz(4.0, 8.0, 4.0),
//         ..default()
//     });
//     // camera
//     commands.spawn(Camera3dBundle {
//         transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
//         ..default()
//     });
// }

// pub fn check_for_save_keypress(
//     keys: Res<Input<KeyCode>>,
// ) -> bool{
//     if keys.just_pressed(KeyCode::AltRight) {
//         return true
//     } else {
//         return false
//     }
// }

// pub fn check_for_load_keypress(
//     keys: Res<Input<KeyCode>>,
// ) -> bool{
//     if keys.just_pressed(KeyCode::AltLeft) {
//         return true
//     } else {
//         return false
//     }
// }

// pub fn spawn_unspawned_models(
//     unspawned_models_querry: Query<(Entity, &ModelVariant), Without<Handle<Mesh>>>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     mut commands: Commands,
// ) {
//     for (unspawned_model, variant) in unspawned_models_querry.iter() {
//         match *variant {
//             ModelVariant::Cube => {
//                 commands.entity(unspawned_model).insert(

//                         PbrBundle {
//                                 mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
//                                 material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
//                                 transform: Transform::from_xyz(0.0, 0.5, 0.0),
//                                 ..default()
//                             }
//                 );

//             }
//         }
//     }
// }

// // pub fn list_components(
// //     model_querry: Query<(Entity), With<Handle<Mesh>>>,
// // ){
// //     for model in model_querry.iter() {
// //         model.serializable()
// //     }
// // }


// // pub fn save_into_file(
// //     path: impl Into<PathBuf>,

// // ) -> SavePipeline  {
// //     println!("saving scene");
// //     save::<With<Save>>
// //     .pipe(into_file(path.into()))
// //     .pipe(finish)
// //     .in_set(SaveSet::Save)
    
// // }