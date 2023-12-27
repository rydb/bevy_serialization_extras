//! A simple 3D scene with light shining over a cube sitting on a plane.

use std::path::PathBuf;

use bevy::{prelude::*, window::PrimaryWindow, render::mesh::shape::Cube};
use bevy_serialization_extras::{plugins::SerializationPlugin, resources::{SaveRequest, LoadRequest, AssetSpawnRequest, AssetSpawnRequestQueue}, bundles::physics::{PhysicsBundle, PhysicsFlagBundle}, loaders::urdf_loader::Urdf, wrappers::link::{Linkage, LinkQuery}};
use bevy_ui_extras::systems::visualize_right_sidepanel_for;
use egui::{TextEdit, text::LayoutJob, TextFormat, ScrollArea};
use moonshine_save::save::Save;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_serialization_extras::bundles::model::ModelBundle;
use bevy_egui::EguiContext;
use bevy_rapier3d::{plugin::{RapierPhysicsPlugin, NoUserData}, render::RapierDebugRenderPlugin, dynamics::{ImpulseJoint, RigidBody, PrismaticJointBuilder, RapierImpulseJointHandle}};

use bevy_serialization_extras::ui::*;


fn main() {

    App::new()

    .insert_resource(SetSaveFile{name: "red".to_owned()})
    .insert_resource(UrdfHandles::default())
    .add_plugins(DefaultPlugins.set(WindowPlugin {exit_condition: bevy::window::ExitCondition::OnPrimaryClosed, ..Default::default()}))
        .add_plugins(SerializationPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        //.add_plugins(SelecterPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, queue_urdf_load_requests)
        //.add_systems(Update, load_urdfs_handles_into_registry)
        .add_systems(Startup, setup)
        //.add_systems(Update, (visualize_right_sidepanel_for::<Save>, save_file_selection))
        .add_systems(Update, manage_serialization_ui)
        .add_systems(Update, display_rapier_joint_info)
        //.add_systems(Update, print_links)
        .run();
}

pub fn minimal_viable_joint(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let joint = PrismaticJointBuilder::new(Vec3::X)
    .local_anchor1(Vec3::new(0.0, 0.0, 1.0))
    .local_anchor2(Vec3::new(0.0, 0.0, -3.0))
    .motor_velocity(1.0, 0.5);

    let cube1 = commands.spawn( (

        PbrBundle {
            mesh: meshes.add(shape::Cube::new(0.2).into()),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            material: materials.add(Color::BLUE.into()),
            ..default()
        },
        RigidBody::Dynamic,   

    )
    )
    .id();
    commands.entity(cube1).insert(
            ImpulseJoint::new(cube1, joint),
    );
    let cube2 = commands.spawn(
        (
        PbrBundle {
            mesh: meshes.add(shape::Cube::new(0.2).into()),
            transform: Transform::from_xyz(0.0, 1.0, 0.5),
            material: materials.add(Color::WHITE.into()),
            ..default()
        },
        RigidBody::Dynamic,
        )
        
    ).id();
}

// #[derive(Component)]
// pub struct RapierJointWindow;

pub fn display_rapier_joint_info(
    mut rapier_joint_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    rapier_joints: Query<&ImpulseJoint>,
) {
    for mut context in rapier_joint_window.iter_mut() { 
        egui::Window::new("Rapier Joint Info textbox")
        .show(context.get_mut(), |ui|{
            //println!("number of joints {:#?}", rapier_joints.iter().len());

            for joint in rapier_joints.iter() {
                ScrollArea::vertical().show(
                    ui, |ui| {
                        let joint_as_string = format!("{:#?}", joint);
                        let job = LayoutJob::single_section(
                            joint_as_string,
                            TextFormat::default()
                        );
                        ui.label(job);
                    }
                );

            }
        })
        ;
    }
}

// pub fn print_rapier_joint_info(
//     rapier_joints: Query<&ImpulseJoint>, 
// ) {
//     for joint in rapier_joints.iter() {
//         println!("joint: {:#?}", joint)
//     }
// }

#[derive(Resource, Default)]
pub struct UrdfHandles {
    pub handle_vec: Vec<Handle<Urdf>>
}

pub fn queue_urdf_load_requests(
    mut urdf_load_requests: ResMut<AssetSpawnRequestQueue<Urdf>>
) {
    // urdf_load_requests.requests.push_front(
    //     AssetSpawnRequest {
    //          source: "urdfs/example_bot.xml".to_owned().into(), 
    //          position: Transform::from_xyz(0.0, 1.0, 0.0), 
    //          ..Default::default()
    //     }
    // )

    urdf_load_requests.requests.push_front(
        AssetSpawnRequest {
             source: "urdfs/tutorial_bot.xml".to_owned().into(), 
             position: Transform::from_xyz(0.0, 1.0, 0.0), 
             ..Default::default()
        }
    )
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
            ..default()
        },
        PhysicsFlagBundle::default()
        )
    );
    
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
