//! A simple 3D scene with light shining over a cube sitting on a plane.

use std::{any::TypeId, collections::HashSet};

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_asset::io::{
    AssetSource,
    file::{FileAssetReader, FileAssetWriter},
};
use bevy_camera_extras::{CameraController, CameraExtrasPlugin, CameraRestrained};
use bevy_inspector_egui::{
    bevy_egui::EguiContext,
    egui::{self, Align2, Color32, Frame, Margin, Rounding, Shadow, Stroke},
};
use bevy_rapier3d::{plugin::RapierPhysicsPlugin, render::RapierDebugRenderPlugin};
use bevy_serialization_assemble::{
    JointRequest, SaveSuccess, components::RequestAssetStructure, prelude::*,
};
use bevy_serialization_core::prelude::*;
use bevy_serialization_physics::prelude::*;
use bevy_ui_extras::{UiExtrasDebug, visualize_components_for};
use moonshine_save::save::Save;

use strum_macros::{Display, EnumIter};

pub const SAVES: &str = "saves";
pub const ROBOT: &str = "diff_bot";

// const SAVES_LOCATION: String = "crates/bevy_serialization_assemble/"..concat!(SAVES);
// const SAVES_LOCATION: &'static str = formatcp!("{:#}://" );

fn main() {
    App::new()
        .add_plugins(AppSourcesPlugin::CRATE)
        .add_plugins(AssetSourcesUrdfPlugin {
            //TODO: This should be unified under `ROOT`
            assets_folder_local_path: "../../assets".to_owned(),
        })
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                exit_condition: bevy::window::ExitCondition::OnPrimaryClosed,
                ..Default::default()
            }), //.set(bevy_mod_raycast::low_latency_window_plugin())
        )
        .insert_state(InitializationStage::Select)
        //.add_schedule(Schedule::new(AssetCheckSchedule))
        .insert_resource(SetSaveFile {
            name: "blue".to_owned(),
        })
        .insert_resource(UtilitySelection::default())
        .add_plugins(RapierPhysicsPlugin::<()>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        // // serialization plugins
        .add_plugins(SerializationPlugin)
        .add_plugins(SerializationAssembleBasePlugin)
        .add_plugins(SerializationPhysicsPlugin)
        .add_plugins(SerializationBasePlugin)
        .add_plugins(UrdfSerializationPlugin)
        // // rapier physics plugins
        .add_plugins(UiExtrasDebug {
            menu_mode: bevy_ui_extras::states::DebugMenuState::Explain,
            ..default()
        })
        .add_plugins(CameraExtrasPlugin {
            cursor_grabbed_by_default: true,
            ..default()
        })
        //.add_systems(Update, urdf_widgets_ui)
        .add_systems(
            Update,
            visualize_components_for::<Name>(bevy_ui_extras::Display::Side(
                bevy_ui_extras::Side::Right,
            )),
        )
        // // Demo systems
        .register_type::<Wheel>()
        .add_systems(Startup, setup)
        .add_systems(Update, control_robot)
        .add_systems(Update, bind_left_and_right_wheel)
        // .add_systems(Update, freeze_spawned_robots)
        .add_systems(
            Update,
            select_robot.run_if(in_state(InitializationStage::Select)),
        )
        .add_systems(
            Update,
            save_selected_robot.run_if(in_state(InitializationStage::Save)),
        )
        .add_systems(
            Update,
            load_saved_robot.run_if(in_state(InitializationStage::LoadSaved)),
        )
        // .register_type::<WasFrozen>()
        // .register_type::<Selected>()
        .run();
}

#[derive(States, Debug, PartialEq, PartialOrd, Hash, Eq, Clone)]
pub enum InitializationStage {
    Select,
    Save,
    LoadSaved,
}

#[derive(Component)]
pub struct BasePlate;

#[derive(Component, Clone, Reflect)]
pub struct Selected;

#[derive(Component, Reflect, Display)]
pub enum Wheel {
    Left,
    Right,
    Passive,
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(
            Vec3::new(0.0, 1.0, 0.0),
            Vec2::new(50.0, 50.0),
        ))),
        MeshMaterial3d(materials.add(Color::LinearRgba(LinearRgba::new(0.3, 0.5, 0.3, 1.0)))),
        Transform::from_xyz(0.0, -1.0, 0.0),
        RigidBodyFlag::Fixed,
        RequestCollider::Convex,
        Name::new("plane"),
        BasePlate,
    ));
    // let robot = "diff_bot.xml";
    // Robot
    commands.spawn((
        RequestAssetStructure::<UrdfWrapper>::Path(
            "root://model_pkg/urdf/".to_owned() + ROBOT + ".xml",
        ),
        Transform::from_xyz(-2.0, 0.0, 0.0),
    ));

    // light
    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
        Save,
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        CameraController {
            restrained: CameraRestrained(false),
            camera_mode: bevy_camera_extras::CameraMode::Free,
        },
    ));
}

pub fn select_robot(
    parts: Query<
        Entity,
        (
            With<MassFlag>,
            With<ColliderFlag>,
            With<Mesh3d>,
            With<RigidBodyFlag>,
            Without<BasePlate>,
            Without<Selected>,
        ),
    >,
    mut commands: Commands,
    mut initialization_stage: ResMut<NextState<InitializationStage>>,
) {
    if parts.iter().len() > 0 {
        for part in &parts {
            commands.entity(part).insert(Selected);
        }
        initialization_stage.set(InitializationStage::Save)
    }
}

pub fn save_selected_robot(
    selected: Query<Entity, With<Selected>>,
    mut assemble_requests: ResMut<AssembleRequests<UrdfWrapper>>,
    mut initialization_stage: ResMut<NextState<InitializationStage>>,
    uninitialized_joints: Query<&JointRequest>,
) {
    if selected.iter().len() > 0 {
        // do not attempt to save the robot while joints are uninitialized or it won't save correctly!
        if uninitialized_joints.iter().len() <= 0 {
            let entities = &mut selected.iter().collect::<HashSet<_>>();

            println!("selected entities: {:#?}", entities);
            let request = AssembleRequest::<UrdfWrapper>::new(
                ROBOT.into(),
                SAVES.to_string(),
                entities.clone(),
            );
            assemble_requests.0.push(request);
            initialization_stage.set(InitializationStage::LoadSaved)
        }
    }
}

pub fn load_saved_robot(mut commands: Commands, mut event_reader: EventReader<SaveSuccess>) {
    for event in event_reader.read() {
        if event.asset_type_id == TypeId::of::<Urdf>() {
            println!("Loading saved robot: {:#}", event.file_name);

            // Robot2
            commands.spawn((
                RequestAssetStructure::<UrdfWrapper>::Path(
                    "saves://".to_owned() + &event.file_name + ".xml",
                ),
                //RequestAssetStructure::<UrdfWrapper>::Path(SAVES.to_owned() + "" + ROBOT + ".xml"),
                Transform::from_xyz(2.0, 0.0, 0.0),
            ));
        }
    }
}

#[derive(Resource, Default)]
pub struct SetSaveFile {
    pub name: String,
}

pub const DEBUG_FRAME_STYLE: Frame = Frame {
    inner_margin: Margin {
        left: 0.0,
        right: 0.0,
        top: 0.0,
        bottom: 0.0,
    },
    outer_margin: Margin {
        left: 0.0,
        right: 0.0,
        top: 0.0,
        bottom: 0.0,
    },
    rounding: Rounding {
        nw: 0.0,
        ne: 0.0,
        sw: 0.0,
        se: 0.0,
    },
    shadow: Shadow::NONE,
    fill: egui::Color32::from_rgba_premultiplied(15, 15, 15, 128),
    stroke: Stroke {
        width: 1.0,
        color: Color32::BLACK,
    },
};

#[derive(Default, EnumIter, Display)]
pub enum UtilityType {
    #[default]
    UrdfInfo,
}
#[derive(Resource, Default)]
pub struct UtilitySelection {
    pub selected: UtilityType,
}

pub const ROOT: &str = "root";

/// Whether this is a crate or `main.rs`.
pub enum AppSourcesPlugin {
    CRATE,
    MAIN,
}

impl Plugin for AppSourcesPlugin {
    fn build(&self, app: &mut App) {
        let executor_location = match *self {
            Self::CRATE => "../../",
            Self::MAIN => "./",
        };
        app.register_asset_source(
            ROOT,
            AssetSource::build().with_reader(move || {
                Box::new(FileAssetReader::new(
                    executor_location.to_owned() + "assets",
                ))
            }),
        );
        app.register_asset_source(
            SAVES,
            AssetSource::build()
                .with_reader(move || Box::new(FileAssetReader::new(SAVES)))
                .with_writer(move |create_root| {
                    Some(Box::new(FileAssetWriter::new(SAVES, create_root)))
                }),
        );
    }
}

pub fn control_robot(
    mut rigid_body_flag: Query<&mut RigidBodyFlag, Without<BasePlate>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut wheels: Query<(&mut JointFlag, &Wheel)>,
) {
    let target_speed = 20.0;

    let turn_speed_multiplier = 0.5;
    let leftward_key = KeyCode::ArrowLeft;
    let rightward_key = KeyCode::ArrowRight;
    let forward_key = KeyCode::ArrowUp;
    let backward_key = KeyCode::ArrowDown;

    let freeze_key = KeyCode::KeyP;
    let unfreeze_key = KeyCode::KeyO;

    for mut context in primary_window.iter_mut() {
        egui::Window::new("robot controls")
            .frame(DEBUG_FRAME_STYLE)
            .anchor(Align2::LEFT_BOTTOM, [0.0, 0.0])
            .show(context.get_mut(), |ui| {
                ui.label(format!("Freeze key: {:#?}", freeze_key));
                ui.label(format!("unfreeze key {:#?}", unfreeze_key));
                ui.label("-------------------------");
                ui.label("");
                ui.label("wheel controls")
            });
    }
    for (mut joint, wheel) in wheels.iter_mut() {
        for axis in joint.joint.motors.iter_mut() {
            if keys.pressed(forward_key) {
                axis.target_vel = target_speed
            } else if keys.pressed(backward_key) {
                axis.target_vel = -target_speed
            } else {
                axis.target_vel = 0.0
            }
        }
        match wheel {
            Wheel::Left => {
                for axis in joint.joint.motors.iter_mut() {
                    if keys.pressed(leftward_key) {
                        axis.target_vel = -target_speed * turn_speed_multiplier;
                    }
                    if keys.pressed(rightward_key) {
                        axis.target_vel = target_speed * turn_speed_multiplier;
                    }
                }
            }
            Wheel::Right => {
                for axis in joint.joint.motors.iter_mut() {
                    if keys.pressed(leftward_key) {
                        axis.target_vel = target_speed * turn_speed_multiplier;
                    }
                    if keys.pressed(rightward_key) {
                        axis.target_vel = -target_speed * turn_speed_multiplier;
                    }
                }
            }
            Wheel::Passive => {}
        }
    }

    if keys.pressed(freeze_key) {
        for mut rigidbody in rigid_body_flag.iter_mut() {
            *rigidbody = RigidBodyFlag::Fixed;
        }
    }
    if keys.pressed(unfreeze_key) {
        for mut rigidbody in rigid_body_flag.iter_mut() {
            *rigidbody = RigidBodyFlag::Dynamic;
        }
    }
}

/// find what is "probably" the left and right wheel, and give them a marker.
pub fn bind_left_and_right_wheel(
    robots: Query<(Entity, &Name), (With<JointFlag>, Without<Wheel>)>,
    mut commands: Commands,
) {
    for (e, name) in robots.iter() {
        let name_str = name.to_string().to_lowercase();

        let split_up = name_str.split("_").collect::<Vec<&str>>();
        //println!("binding wheel");
        if split_up.contains(&Wheel::Left.to_string().to_lowercase().as_str()) {
            commands.entity(e).insert(Wheel::Left);
        } else if split_up.contains(&Wheel::Right.to_string().to_lowercase().as_str()) {
            commands.entity(e).insert(Wheel::Right);
        } else {
            commands.entity(e).insert(Wheel::Passive);
        }
    }
}

#[derive(Component, Reflect)]
pub struct WasFrozen;

//FIXME: physics bodies fly out of control when spawned, this freezes them for the user to unpause until thats fixed.
pub fn freeze_spawned_robots(
    mut robots: Query<(Entity, &mut RigidBodyFlag), Without<WasFrozen>>,
    mut commands: Commands,
) {
    for (e, mut body) in robots.iter_mut() {
        *body = RigidBodyFlag::Fixed;
        commands.entity(e).insert(WasFrozen);
    }
}
