//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_serialization_extras::plugins::SerializationPlugin;
use bevy_ui_extras::systems::visualize_right_sidepanel_for;
use moonshine_save::save::Save;
use bevy_editor_extras::plugins::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_serialization_extras::bundles::model::ModelBundle;
use bevy_egui::EguiContext;

const SAVE_FOLDER: &str = "examples";

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SerializationPlugin)
        .add_plugins(SelecterPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .add_systems(Update, (visualize_right_sidepanel_for::<Save>, save_file_selection))
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands.spawn(
        (
        ModelBundle {
            mesh: shape::Cube {size: 1.0}.into(),
            material: Color::GREEN.into(),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        //PhysicsBundle::default(),
        Save,
        //MakeSelectableBundle::default(),
));
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

pub fn save_file_selection(
    world: &mut World,

) {
    if let Ok(egui_context_check) = world
        .query_filtered::<&mut EguiContext, &PrimaryWindow>()
        .get_single(world) 
    {
        let menu_name = "Select a sample save to load";
        // if let Some(asset_server)  = world.get_resource::<AssetServer>() {
        //     asset_server.
        // } 

        let mut egui_context = egui_context_check.clone();
        

        egui::TopBottomPanel::bottom(menu_name)
        .show(egui_context.get_mut(), |ui| {
                //ui.heading(menu_name);
                ui.button("save_1")
            }
    
    );
    }
}