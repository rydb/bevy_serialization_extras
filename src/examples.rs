//! A simple 3D scene with light shining over a cube sitting on a plane.

use std::{
    io,
    path::{Path, PathBuf},
};
use moonshine_save::{
    prelude::*,
    save::*,
};
    use bevy::prelude::*;

/// marks component as a valid candidate for serialization. 
// #[derive(Component)]
// pub struct Serializable;

const SAVE_PATH: &str = "cube.ron";

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct FavoriteNumber {
    pub favorite_number: u32,
}
/// marks entity as to posessing a model. Depending on the variant, a seperate system should de-serialize the entity this component is attached to
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub enum ModelVariant {
    #[default]
    Cube
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((SavePlugin, LoadPlugin))
        .add_systems(Startup, setup)
        .register_type::<FavoriteNumber>()
        .register_type::<ModelVariant>()
        .register_type::<Option<Entity>>()
        .add_systems(Update, save_into_file(SAVE_PATH).run_if(check_for_save_keypress))
        .add_systems(Update, load_from_file(SAVE_PATH).run_if(check_for_load_keypress))
        .add_systems(Update, spawn_unspawned_models)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(
    PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // mark cube to spawn
    // commands.spawn(
    // (

    //     Save,
    //     FavoriteNumber{favorite_number: 10},
    //     ModelVariant::Cube,
    // ));
    // // cube
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //     transform: Transform::from_xyz(0.0, 0.5, 0.0),
    //     ..default()
    // });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
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

pub fn spawn_unspawned_models(
    unspawned_models_querry: Query<(Entity, &ModelVariant), Without<Handle<Mesh>>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    for (unspawned_model, variant) in unspawned_models_querry.iter() {
        match *variant {
            ModelVariant::Cube => {
                commands.entity(unspawned_model).insert(

                        PbrBundle {
                                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                                transform: Transform::from_xyz(0.0, 0.5, 0.0),
                                ..default()
                            }
                );

            }
        }
    }
}

// pub fn save_into_file(
//     path: impl Into<PathBuf>,

// ) -> SavePipeline  {
//     println!("saving scene");
//     save::<With<Save>>
//     .pipe(into_file(path.into()))
//     .pipe(finish)
//     .in_set(SaveSet::Save)
    
// }
