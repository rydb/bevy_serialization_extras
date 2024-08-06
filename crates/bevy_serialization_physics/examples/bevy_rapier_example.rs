use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_raycast::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_serialization_core::plugins::SerializationPlugin;
use bevy_serialization_physics::{
    plugins::PhysicsSerializationPlugin,
    ui::{
        motor_controller_ui, physics_utilities_ui, rapier_joint_info_ui, PhysicsUtilitySelection,
        Selectable, Selected, SelectedMotorAxis,
    },
};

use bevy::prelude::Vec3;
use bevy_ui_extras::systems::visualize_window_for;

//RapierImpulseJointHandle

fn main() {
    App::new()
    //raycasting
    .add_plugins(DefaultPlugins.set(bevy_mod_raycast::low_latency_window_plugin()))
    .add_plugins(CursorRayPlugin)
    //.add_plugins(RaycastPluginState)
    .add_plugins((
        RapierPhysicsPlugin::<NoUserData>::default(),
        RapierDebugRenderPlugin::default(),
    ))
    .add_plugins(WorldInspectorPlugin::new())
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
        transform: Transform::from_xyz(ORIGIN.x - 5.0, ORIGIN.y, ORIGIN.z)
            .looking_at(Vec3::new(13.0, 1.0, 1.0), Vec3::Y),
        ..Default::default()
    });
}

const CUBE_COUNT: usize = 2;
const ORIGIN: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const NUM: usize = 1;

fn create_revolute_joints(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //let rad = 0.4;
    let shift = 2.0;

    let mut curr_parent = commands
        .spawn((
            //TransformBundle::from(Transform::from_xyz(origin.x, origin.y, 0.0)),
            RigidBody::Fixed,
            AsyncCollider::default(),
            PbrBundle {
                mesh: meshes.add(Cuboid::new(0.5, 0.5, 0.5)),
                transform: Transform::from_xyz(ORIGIN.x, ORIGIN.y, 0.0),
                material: materials.add(Color::Srgba(Srgba::BLUE)),
                //transform: Transform::from_xyz(15.0, 3.0, 30.0),
                ..default()
            }, 
        ))
        .id();

    for i in 0..NUM {
        // Create four bodies.
        let z = ORIGIN.z + i as f32 * shift * 2.0 + shift;
        let positions = [
            Vec3::new(ORIGIN.x, ORIGIN.y, z),
            Vec3::new(ORIGIN.x + shift, ORIGIN.y, z),
            Vec3::new(ORIGIN.x + shift, ORIGIN.y, z + shift),
            Vec3::new(ORIGIN.x, ORIGIN.y, z + shift),
        ];

        let mut handles = [curr_parent; CUBE_COUNT];
        for k in 0..CUBE_COUNT {
            handles[k] = commands
                .spawn((
                    RigidBody::Dynamic,
                    AsyncCollider::default(),
                    PbrBundle {
                        mesh: meshes.add(Cuboid::new(0.5, 0.5, 0.5)),
                        transform: Transform::from_translation(positions[k]),
                        material: materials.add(Color::Srgba(Srgba::BLUE)),
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
            RevoluteJointBuilder::new(x).local_anchor2(Vec3::new(-shift, 0.0, 0.0)), 
        ];

        commands
            .entity(handles[0])
            .insert(ImpulseJoint::new(curr_parent, revs[0]))
            .insert(Selectable);
        commands
            .entity(handles[1])
            .insert(ImpulseJoint::new(handles[0], revs[1]))
            .insert(Selectable);
        curr_parent = *handles.last().unwrap();
    }
}

//FIXME: This should be in its own crate, but for speed sake, this is here for now.
pub fn selector_raycast(
    cursor_ray: Res<CursorRay>,
    mut raycast: Raycast,
    mouse_press: Res<ButtonInput<MouseButton>>,
    mut selectables: Query<Entity, (With<Selectable>, With<Transform>)>,
    selected: Query<(Entity, &Selected)>,
    mut commands: Commands,
) {
    if let Some(cursor_ray) = **cursor_ray {
        let hits = raycast.cast_ray(cursor_ray, &default());
        for (e, _) in hits.iter() {
            if mouse_press.just_pressed(MouseButton::Left) {
                if let Ok(e) = selectables.get_mut(e.clone()) {
                    match selected.get(e) {
                        Ok(..) => commands.entity(e).remove::<Selected>(),
                        Err(..) => commands.entity(e).insert(Selected),
                    };
                }
            }
        }
    }
}
