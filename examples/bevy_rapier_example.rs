use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use bevy_serialization_extras::plugins::SerializationPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            0xF9 as f32 / 255.0,
            0xF9 as f32 / 255.0,
            0xFF as f32 / 255.0,
        )))
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        .add_plugins(SerializationPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, (setup_graphics, setup_physics))
        .run();
}

pub fn setup_graphics(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(10.0, 3.0, 0.0)
            .looking_at(Vec3::new(0.0, 3.0, 0.0), Vec3::Y),
        ..Default::default()
    });
}

pub fn setup_physics(mut commands: Commands) {
    let ground_size = 5.0;
    let ground_height = 0.1;

    commands.spawn((
        TransformBundle::from(Transform::from_xyz(0.0, -ground_height, 0.0)),
        Collider::cuboid(ground_size, ground_height, ground_size),
    ));

    commands.spawn((
        TransformBundle::from(Transform::from_xyz(0.0, 3.0, 0.0)),
        RigidBody::Dynamic,
        LockedAxes::TRANSLATION_LOCKED
            | LockedAxes::ROTATION_LOCKED_Y
            | LockedAxes::ROTATION_LOCKED_Z,
        Collider::cuboid(0.2, 0.6, 2.0),
    ));

    commands.spawn((
        TransformBundle::from(
            Transform::from_xyz(0.0, 5.0, 0.0).with_rotation(Quat::from_rotation_x(1.0)),
        ),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Collider::cuboid(0.6, 0.4, 0.4),
    ));
}