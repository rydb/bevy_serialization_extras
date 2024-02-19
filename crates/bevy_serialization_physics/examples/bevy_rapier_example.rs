use bevy::{
    ecs::{
        query::ReadOnlyWorldQuery,
        schedule::{SystemConfig, SystemConfigs},
    },
    prelude::*,
    render::camera::CameraProjection,
    utils::HashMap,
    window::PrimaryWindow,
};
use bevy_egui::EguiContext;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_raycast::{immediate::Raycast, CursorRay, DefaultRaycastingPlugin};
use bevy_rapier3d::prelude::*;
use bevy_serialization_core::plugins::SerializationPlugin;
use bevy_serialization_physics::{
    plugins::PhysicsSerializationPlugin,
    ui::{
        motor_controller_ui, physics_utilities_ui, rapier_joint_info_ui, selector_raycast,
        PhysicsUtilitySelection, Selectable, Selected, SelectedMotorAxis,
    },
    wrappers::link::{JointFlag, JointMotorWrapper},
};
use bevy_ui_extras::{
    stylesheets::DEBUG_FRAME_STYLE,
    systems::{visualize_left_sidepanel_for, visualize_right_sidepanel_for, visualize_window_for},
};
use egui::{text::LayoutJob, Pos2, Rect, ScrollArea, TextFormat, Ui};

use bevy::prelude::Vec3;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

//RapierImpulseJointHandle

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        .add_plugins((DefaultRaycastingPlugin, WorldInspectorPlugin::new()))
        .add_plugins(SerializationPlugin)
        .add_plugins(PhysicsSerializationPlugin)
        //.insert_resource()
        //.add_plugins(SerializationPlugin)
        .register_type::<Selectable>()
        .init_resource::<SelectedMotorAxis>()
        .init_resource::<PhysicsUtilitySelection>()
        .add_systems(Update, visualize_window_for::<Selected>)
        .add_systems(Update, selector_raycast)
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, create_revolute_joints)
        .add_systems(Update, motor_controller_ui)
        .add_systems(Update, physics_utilities_ui)
        .add_systems(Update, rapier_joint_info_ui)
        //.add_systems(PostUpdate, )
        //.add_systems(Update, display_rapier_joint_info)
        .run();
}

pub fn setup_graphics(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(origin.x - 5.0, origin.y, origin.z)
            .looking_at(Vec3::new(13.0, 1.0, 1.0), Vec3::Y),
        ..Default::default()
    });
}

const CUBE_COUNT: usize = 2;
const origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const num: usize = 1;

fn create_revolute_joints(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let rad = 0.4;
    let shift = 2.0;

    let mut curr_parent = commands
        .spawn((
            //TransformBundle::from(Transform::from_xyz(origin.x, origin.y, 0.0)),
            RigidBody::Fixed,
            AsyncCollider::default(),
            PbrBundle {
                mesh: meshes.add(shape::Cube::new(0.5).into()),
                transform: Transform::from_xyz(origin.x, origin.y, 0.0),
                //transform: Transform::from_xyz(15.0, 3.0, 30.0),
                ..default()
            }, //meshes.add(shape::Cube::new(0.5).into())
               //Collider::cuboid(rad, rad, rad),
        ))
        .id();

    for i in 0..num {
        // Create four bodies.
        let z = origin.z + i as f32 * shift * 2.0 + shift;
        let positions = [
            Vec3::new(origin.x, origin.y, z),
            Vec3::new(origin.x + shift, origin.y, z),
            Vec3::new(origin.x + shift, origin.y, z + shift),
            Vec3::new(origin.x, origin.y, z + shift),
        ];

        let mut handles = [curr_parent; CUBE_COUNT];
        for k in 0..CUBE_COUNT {
            handles[k] = commands
                .spawn((
                    RigidBody::Dynamic,
                    AsyncCollider::default(),
                    PbrBundle {
                        mesh: meshes.add(shape::Cube::new(0.5).into()),
                        transform: Transform::from_translation(positions[k]),
                        ..default()
                    }, //Collider::cuboid(rad, rad, rad),
                ))
                .id();
        }

        // Setup four joints.
        let x = Vec3::X;
        let z = Vec3::Z;

        let revs = [
            RevoluteJointBuilder::new(z)
                .local_anchor2(Vec3::new(0.0, 0.0, -shift))
                .motor_velocity(100.0, 20.0),
            RevoluteJointBuilder::new(x).local_anchor2(Vec3::new(-shift, 0.0, 0.0)), //.inse,
                                                                                     // RevoluteJointBuilder::new(z).local_anchor2(Vec3::new(0.0, 0.0, -shift)),
                                                                                     // RevoluteJointBuilder::new(x).local_anchor2(Vec3::new(shift, 0.0, 0.0)),
        ];

        commands
            .entity(handles[0])
            .insert(ImpulseJoint::new(curr_parent, revs[0]))
            .insert(Selectable);
        commands
            .entity(handles[1])
            .insert(ImpulseJoint::new(handles[0], revs[1]))
            .insert(Selectable);
        // commands
        //     .entity(handles[2])
        //     .insert(ImpulseJoint::new(handles[1], revs[2]));
        // commands
        //     .entity(handles[3])
        //     .insert(ImpulseJoint::new(handles[2], revs[3]));

        curr_parent = *handles.last().unwrap();
    }
}
