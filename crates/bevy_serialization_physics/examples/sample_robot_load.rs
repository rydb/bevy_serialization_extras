//! A simple 3D scene with light shining over a cube sitting on a plane.

use std::path::PathBuf;

use bevy::{prelude::*, window::PrimaryWindow, render::mesh::shape::Cube};
use bevy_serialization_core::{plugins::SerializationPlugin, resources::{AssetSpawnRequestQueue, AssetSpawnRequest}};
use bevy_serialization_physics::{ui::{physics_widgets_window, CachedUrdf}, loaders::urdf_loader::Urdf, bundles::physics::PhysicsBundle, plugins::PhysicsSerializationPlugin};
use egui::{TextEdit, text::LayoutJob, TextFormat, ScrollArea};
use moonshine_save::save::Save;
use bevy_egui::EguiContext;
use bevy_rapier3d::{plugin::{RapierPhysicsPlugin, NoUserData}, render::RapierDebugRenderPlugin, dynamics::{ImpulseJoint, RigidBody, PrismaticJointBuilder, RapierImpulseJointHandle}, geometry::Collider};
use bitvec::{prelude::*, view::BitView};
use bevy_camera_extras::plugins::DefaultCameraPlugin;

use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {

    App::new()

    .insert_resource(SetSaveFile{name: "red".to_owned()})
    .insert_resource(UrdfHandles::default())
    .add_plugins(DefaultPlugins.set(WindowPlugin {exit_condition: bevy::window::ExitCondition::OnPrimaryClosed, ..Default::default()}))
        
        // serialization plugins
        .add_plugins(SerializationPlugin)
        .add_plugins(PhysicsSerializationPlugin)
        // rapier physics plugins
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        
        
        .add_plugins(DefaultCameraPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, queue_urdf_load_requests)
        .add_systems(Startup, setup)
        .add_systems(Update, physics_widgets_window)
        .run();
}




#[derive(Resource, Default)]
pub struct UrdfHandles {
    pub handle_vec: Vec<Handle<Urdf>>,

}

pub fn queue_urdf_load_requests(
    mut urdf_load_requests: ResMut<AssetSpawnRequestQueue<Urdf>>,
    mut cached_urdf: ResMut<CachedUrdf>,
    mut asset_server: Res<AssetServer>,

) {
    let load_urdf_path = "urdf_tutorial/urdfs/tutorial_bot.xml";
    //let load_urdf_path = "urdf_tutorial/urdfs/issue_test.xml";
    //let load_urdf_path = "urdf_tutorial/urdfs/full_urdf_tutorial_bot.xml";
    cached_urdf.urdf = asset_server.load(load_urdf_path);
    // urdf_load_requests.requests.push_front(
    //     AssetSpawnRequest {
    //          source: "urdfs/example_bot.xml".to_owned().into(), 
    //          position: Transform::from_xyz(0.0, 1.0, 0.0), 
    //          ..Default::default()
    //     }
    // )

    urdf_load_requests.requests.push_front(
        AssetSpawnRequest {
             source: load_urdf_path.to_owned().into(), 
             position: Transform::from_xyz(0.0, 1.0, 0.0), 
             ..Default::default()
        }
    );
    // urdf_load_requests.requests.push_front(
    //     AssetSpawnRequest {
    //          source: "urdf_tutorial/urdfs/model_load_test.xml".to_owned().into(), 
    //          position: Transform::from_xyz(0.0, 1.0, 0.0), 
    //          ..Default::default()
    //     }
    // )
    // urdf_load_requests.requests.push_front(
    //     AssetSpawnRequest {
    //          source: "urdf_tutorial/urdfs/full_urdf_tutorial_bot.xml".to_owned().into(), 
    //          position: Transform::from_xyz(0.0, 1.0, 0.0), 
    //          ..Default::default()
    //     }
    // );
    
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cached_urdf: ResMut<CachedUrdf>
    //mut urdfs: ResMut<Urdfs>
) {
    // for (l, i) in materials.iter() {

    // }
    // plane
    commands.spawn(
    (
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(5.0).into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::from_xyz(0.0, -1.0, 0.0),
            ..default()
        },
        PhysicsBundle::default()
        )
    );
    
    // cube
    // commands.spawn(
    //     (
    //         PbrBundle {
    //             mesh: meshes.add(shape::Cube {size: 1.0}.into()),
    //             material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    //             ..default()
    //         },
    //         PhysicsBundle {
    //             rigid_body: RigidBody::Dynamic,
    //             ..default()
    //         }
    //     )
    // );

    // light
    commands.spawn(
        (
        PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    },
    Save
    )
);
    // camera
    commands.spawn(
    (Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    },
    Save
));
}


#[derive(Resource, Default)]
pub struct SetSaveFile {
    pub name: String,
}
