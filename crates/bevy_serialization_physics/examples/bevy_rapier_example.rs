use bevy::prelude::*;
use bevy_egui::EguiContext;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_raycast::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_serialization_core::{plugins::SerializationPlugin, prelude::SerializationBasePlugin};
use bevy_serialization_physics::prelude::{link::{JointAxesMaskWrapper, JointFlag}, SerializationPhysicsPlugin};

use bevy::prelude::Vec3;
use bevy_ui_extras::systems::visualize_components_for;
use bevy_window::PrimaryWindow;
use bitvec::{field::BitField, order::Msb0, view::BitView};
use egui::{text::LayoutJob, ScrollArea, TextFormat, Ui};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

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
    .add_plugins(SerializationPhysicsPlugin)
    .add_plugins(SerializationBasePlugin)
    //.insert_resource()
    //.add_plugins(SerializationPlugin)
    .register_type::<Selectable>()
    .init_resource::<SelectedMotorAxis>()
    .init_resource::<PhysicsUtilitySelection>()
    .add_systems(Update, visualize_components_for::<Selected>(bevy_ui_extras::Display::Window))
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
const DASHES: usize = 5;
const CUBE_COUNT: usize = 2;
const ORIGIN: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const NUM: usize = 1;


#[derive(Component, Reflect)]
pub struct Selectable;

#[derive(Resource, Default, Clone, Copy, PartialEq)]
pub struct SelectedMotorAxis {
    pub axis: MotorAxis,
}

#[derive(Component, Default, Reflect)]
pub struct Selected;

#[derive(Default, EnumIter, Clone, Copy, Display, PartialEq)]
pub enum MotorAxis {
    X = 0,
    Y = 1,
    Z = 2,
    #[default]
    ANGX = 3,
    ANGY = 4,
    ANGZ = 5,
}
#[derive(Default, EnumIter, Display)]
pub enum PhysicsUtilityType {
    #[default]
    Joints,
}

#[derive(Resource, Default)]
pub struct PhysicsUtilitySelection {
    pub selected: PhysicsUtilityType,
}

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

pub fn rapier_joint_info_ui(
    mut rapier_joint_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut rapier_joints: Query<&ImpulseJoint, With<Selected>>,
) {
    for mut context in rapier_joint_window.iter_mut() {
        egui::Window::new("Rapier Joint Info textbox")
            //.frame(DEBUG_FRAME_STYLE)
            .show(context.get_mut(), |ui| {
                for joint in rapier_joints.iter_mut() {
                    ScrollArea::vertical()
                        //.max_height(window_size.unwrap_or(Rect{min: Pos2::default(), max: Pos2::default()}).height())
                        .max_height(500.0)
                        //.id_source(i.to_string() + "_joint")
                        .show(ui, |ui| {
                            let joint_as_string = format!("{:#?}", joint);
                            let job =
                                LayoutJob::single_section(joint_as_string, TextFormat::default());
                            if ui.button("Copy to clipboard").clicked() {
                                ui.output_mut(|o| o.copied_text = String::from(job.text.clone()));
                            }
                            ui.label(job.clone());
                        });
                }
            });
    }
}



pub fn motor_controller_ui(
    mut selected_joints: Query<(Entity, &mut JointFlag), With<Selected>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut contexts: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut motor_axis: ResMut<SelectedMotorAxis>,
) {
    let negative_accel_key = KeyCode::Minus;
    let positive_accel_key = KeyCode::Equal;

    let negative_damping_key = KeyCode::BracketLeft;
    let positive_damping_key = KeyCode::BracketRight;

    let motor_index = motor_axis.axis as usize;

    let window_name = "motor controller";

    //let mut selected_axis = motor_axis.into_inner().clone();

    for mut context in contexts.iter_mut() {
        egui::Window::new(window_name)
            //.frame(DEBUG_FRAME_STYLE)
            .show(context.get_mut(), |ui| {
                ui.label("Controls");
                ui.label("-".repeat(DASHES));
                ui.label(format!(
                    "positive acceleration: [{:#?}] key",
                    positive_accel_key
                ));
                ui.label(format!(
                    "negative acceleration: [{:#?}] key",
                    negative_accel_key
                ));
                ui.label("-".repeat(DASHES));

                for (e, joint) in selected_joints.iter() {
                    //ui.label("-".repeat(DASHES));
                    ui.label(format!("{:#?}, to {:#?} info", e, joint.parent_id));

                    ui.label("motor info:");
                    //ui.selectable_value(&mut motor_index.index, curent_value, motor_index_text);

                    ui.horizontal(|ui| {
                        for axis in MotorAxis::iter() {
                            ui.selectable_value(&mut motor_axis.axis, axis, axis.to_string());
                        }
                    });
                    ui.label(format!("{:#?}", joint.motors[motor_index]));
                    // for motor in joint.motors.iter() {
                    //     ui.label(format!("{:#?}",motor));
                    // }
                    ui.label("-".repeat(DASHES));
                }
            });
    }
    if keyboard.pressed(negative_accel_key) {
        for (_, mut joint) in selected_joints.iter_mut() {
            joint.motors[motor_index].target_vel += -1.0;
        }
    }
    if keyboard.pressed(positive_accel_key) {
        for (_, mut joint) in selected_joints.iter_mut() {
            joint.motors[motor_index].target_vel += 1.0;
        }
    }

    if keyboard.pressed(negative_damping_key) {
        for (_, mut joint) in selected_joints.iter_mut() {
            joint.motors[motor_index].damping += -1.0;
        }
    }
    if keyboard.pressed(positive_damping_key) {
        for (_, mut joint) in selected_joints.iter_mut() {
            joint.motors[motor_index].damping += 1.0;
        }
    }
}

// A collection of utilities that make debugging physics easier
pub fn physics_utilities_ui(
    mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut utility_selection: ResMut<PhysicsUtilitySelection>,

    mut selected_joints: Query<(Entity, &mut JointFlag), With<Selected>>,
) {
    for mut context in primary_window.iter_mut() {
        egui::Window::new("Physics Utilities")
            //.title_bar(false)
            //.frame(DEBUG_FRAME_STYLE)
            .show(context.get_mut(), |ui| {
                // lay out the ui widget selection menu
                ui.horizontal(|ui| {
                    for utility in PhysicsUtilityType::iter() {
                        if ui.button(utility.to_string()).clicked() {
                            utility_selection.selected = utility;
                        }
                    }
                });

                match utility_selection.selected {
                    PhysicsUtilityType::Joints => {
                        for (e, mut joint) in selected_joints.iter_mut() {
                            ui.label(format!("{:#?}, to {:#?} info", e, joint.parent_id));
                            ui.label("-".repeat(DASHES));
                            //ui.label()

                            ui.label("limit axis bits");

                            ui.horizontal(|ui: &mut Ui| {
                                let mut limit_axis_bits = joint.limit_axes.bits().clone();
                                let limit_axis_bitvec = limit_axis_bits.view_bits_mut::<Msb0>();

                                for mut bit in limit_axis_bitvec.iter_mut() {
                                    //let mut bit_value = bit;

                                    ui.checkbox(&mut bit, "");
                                }
                                let new_joint_mask = JointAxesMaskWrapper::from_bits_truncate(
                                    limit_axis_bitvec.load_le(),
                                );
                                // stops component from being registered as changed if nothing is happening to it
                                if joint.limit_axes != new_joint_mask {
                                    joint.limit_axes = new_joint_mask;
                                }
                            });

                            ui.label("locked axis bits");
                            ui.horizontal(|ui| {
                                let mut locked_axis_bits = joint.locked_axes.bits().clone();
                                let limit_axis_bitvec = locked_axis_bits.view_bits_mut::<Msb0>();

                                for mut bit in limit_axis_bitvec.iter_mut() {
                                    //let mut bit_value = bit;

                                    ui.checkbox(&mut bit, "");
                                }
                                let new_joint_mask = JointAxesMaskWrapper::from_bits_truncate(
                                    limit_axis_bitvec.load_le(),
                                );

                                if joint.locked_axes != new_joint_mask {
                                    joint.locked_axes = new_joint_mask;
                                }
                            });
                            ui.label("-".repeat(DASHES));
                        }
                    }
                }
            });
    }
}