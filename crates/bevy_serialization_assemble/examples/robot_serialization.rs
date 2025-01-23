//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_asset::io::{file::FileAssetReader, AssetSource};
use bevy_camera_extras::{CameraController, CameraExtrasPlugin, CameraRestrained};
use bevy_egui::EguiContext;
use bevy_obj::ObjPlugin;
use bevy_serialization_assemble::prelude::*;
use bevy_serialization_core::prelude::*;
use bevy_serialization_physics::prelude::*;
use bevy_ui_extras::{visualize_components_for, UiExtrasDebug};
use egui::{
    text::LayoutJob, Color32, Frame, Margin, Rounding, ScrollArea, Shadow, Stroke, TextFormat,
};
use moonshine_save::save::Save;

use resources::CachedUrdf;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

fn main() {
    App::new()
        .add_plugins(AppSourcesPlugin::CRATE)
        .add_plugins(AssetSourcesUrdfPlugin {
            //TODO: This should be unified under `ROOT`
            assets_folder_local_path: "../../assets".to_owned(),
        })
        //.add_schedule(Schedule::new(AssetCheckSchedule))
        .insert_resource(AssetCheckers::default())
        .insert_resource(SetSaveFile {
            name: "blue".to_owned(),
        })
        .insert_resource(UrdfHandles::default())
        .insert_resource(UtilitySelection::default())
        // asset sources
        // .add_plugins(AssetSourcesUrdfPlugin {
        //     assets_folder_local_path: "assets/".to_owned()
        // })
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                exit_condition: bevy::window::ExitCondition::OnPrimaryClosed,
                ..Default::default()
            }), //.set(bevy_mod_raycast::low_latency_window_plugin())
        )
        .add_plugins(ObjPlugin)
        // serialization plugins
        .add_plugins(SerializationPlugin)
        .add_plugins(SerializationAssembleBasePlugin)
        .add_plugins(SerializationPhysicsPlugin)
        .add_plugins(SerializationBasePlugin)
        .add_plugins(UrdfSerializationPlugin)
        //.add_plugins(GltfSerializationPlugin)
        // rapier physics plugins
        .add_plugins(UiExtrasDebug::default())
        .add_plugins(CameraExtrasPlugin {
            cursor_grabbed_by_default: true,
            ..default()
        })
        .add_systems(Update, urdf_widgets_ui)
        .add_systems(
            Update,
            visualize_components_for::<Name>(bevy_ui_extras::Display::Side(
                bevy_ui_extras::Side::Right,
            )),
        )
        // Demo systems
        .register_type::<Wheel>()
        .add_systems(Startup, setup)
        .add_systems(Startup, queue_urdf_load_requests)
        .add_systems(Update, control_robot)
        // .add_systems(Update, make_robots_selectable)
        .add_systems(Update, bind_left_and_right_wheel)
        
        //.add_systems(Update, freeze_spawned_robots)
        .run();
}

#[derive(Component, Reflect, Display)]
pub enum Wheel {
    Left,
    Right,
}

/// find what is "probably" the left and right wheel, and give them a marker.
pub fn bind_left_and_right_wheel(
    robots: Query<(Entity, &Name), (With<JointFlag>, Without<Wheel>)>,
    mut commands: Commands,
) {
    for (e, name) in robots.iter() {
        let name_str = name.to_string().to_lowercase();

        let split_up = name_str.split("_").collect::<Vec<&str>>();

        if split_up.contains(&Wheel::Left.to_string().to_lowercase().as_str()) {
            commands.entity(e).insert(Wheel::Left);
        }
        if split_up.contains(&Wheel::Right.to_string().to_lowercase().as_str()) {
            commands.entity(e).insert(Wheel::Right);
        }
    }
}

#[derive(Resource, Default)]
pub struct UrdfHandles {
    pub handle_vec: Vec<Handle<Urdf>>,
}

pub fn control_robot(
    mut rigid_body_flag: Query<&mut RigidBodyFlag, (Without<JointFlag>, With<StructureFlag>)>,
    keys: Res<ButtonInput<KeyCode>>,
    mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut wheels: Query<(&mut JointFlag, &Wheel)>,
) {
    let target_speed = 20.0;

    let leftward_key = KeyCode::ArrowLeft;
    let rightward_key = KeyCode::ArrowRight;
    let forward_key = KeyCode::ArrowUp;
    let backward_key = KeyCode::ArrowDown;

    let freeze_key = KeyCode::KeyP;
    let unfreeze_key = KeyCode::KeyO;

    for mut context in primary_window.iter_mut() {
        egui::Window::new("robot controls")
            .frame(DEBUG_FRAME_STYLE)
            .show(context.get_mut(), |ui| {
                ui.label(format!("Freeze key: {:#?}", freeze_key));
                ui.label(format!("unfreeze key {:#?}", unfreeze_key));
                ui.label("-------------------------");
                ui.label("");
                ui.label("wheel controls")
            });
    }
    for (mut joint, wheel) in wheels.iter_mut() {
        for axis in joint.motors.iter_mut() {
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
                for axis in joint.motors.iter_mut() {
                    if keys.pressed(leftward_key) {
                        axis.target_vel = -target_speed
                    }
                    if keys.pressed(rightward_key) {
                        axis.target_vel = target_speed
                    }
                }
            }
            Wheel::Right => {
                for axis in joint.motors.iter_mut() {
                    if keys.pressed(leftward_key) {
                        axis.target_vel = target_speed
                    }
                    if keys.pressed(rightward_key) {
                        axis.target_vel = -target_speed
                    }
                }
            }
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

pub fn queue_urdf_load_requests(
    mut urdf_load_requests: ResMut<AssetSpawnRequestQueue<Urdf>>,
    mut cached_urdf: ResMut<CachedUrdf>,
    asset_server: Res<AssetServer>,
) {
    // set load_urdf_path to the urdf you want to load.

    let load_urdf_path = "root://model_pkg/urdf/diff_bot.xml";
    cached_urdf.urdf = asset_server.load(load_urdf_path);

    urdf_load_requests.requests.push_front(AssetSpawnRequest {
        source: load_urdf_path.to_owned().into(),
        position: Transform::from_xyz(0.0, 2.0, 0.0),
        ..Default::default()
    });
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

pub fn urdf_widgets_ui(
    mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut utility_selection: ResMut<UtilitySelection>,
    cached_urdf: Res<CachedUrdf>,
    urdfs: Res<Assets<Urdf>>,
) {
    for mut context in primary_window.iter_mut() {
        egui::Window::new("debug widget window")
            //.title_bar(false)
            .frame(DEBUG_FRAME_STYLE)
            .show(context.get_mut(), |ui| {
                // lay out the ui widget selection menu
                ui.horizontal(|ui| {
                    for utility in UtilityType::iter() {
                        if ui.button(utility.to_string()).clicked() {
                            utility_selection.selected = utility;
                        }
                    }
                });

                match utility_selection.selected {
                    UtilityType::UrdfInfo => {
                        if let Some(urdf) = urdfs.get(&cached_urdf.urdf) {
                            let urdf_as_string = format!("{:#?}", urdf.robot);

                            if ui.button("Copy to clipboard").clicked() {
                                ui.output_mut(|o| o.copied_text = urdf_as_string.to_string());
                            }
                            ScrollArea::vertical().show(ui, |ui| {
                                let job = LayoutJob::single_section(
                                    urdf_as_string,
                                    TextFormat::default(),
                                );
                                ui.label(job);
                            });
                        }
                    }
                }
            });
    }
}

pub const ROOT: &str = "root";

/// Whether this is a crate or `main.rs`.
pub enum AppSourcesPlugin {
    CRATE,
    MAIN,
}

impl Plugin for AppSourcesPlugin {
    fn build(&self, app: &mut App) {
        let asset_folder_location = match *self {
            Self::CRATE => "../../assets",
            Self::MAIN => "assets",
        };
        app.register_asset_source(
            ROOT,
            AssetSource::build()
                .with_reader(move || Box::new(FileAssetReader::new(asset_folder_location))),
        );
    }
}
