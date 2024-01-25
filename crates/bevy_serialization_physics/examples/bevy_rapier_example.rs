use bevy::{ecs::query::ReadOnlyWorldQuery, prelude::*, render::camera::CameraProjection, utils::HashMap, window::PrimaryWindow};
use bevy_egui::EguiContext;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_raycast::{immediate::Raycast, CursorRay, DefaultRaycastingPlugin};
use bevy_rapier3d::prelude::*;
use bevy_serialization_core::plugins::SerializationPlugin;
use bevy_serialization_physics::{plugins::PhysicsSerializationPlugin, wrappers::link::JointFlag};
use bevy_ui_extras::{stylesheets::DEBUG_FRAME_STYLE, systems::{visualize_left_sidepanel_for, visualize_right_sidepanel_for, visualize_window_for}};
use egui::{text::LayoutJob, Pos2, Rect, ScrollArea, TextFormat};

use bevy::prelude::Vec3;

//RapierImpulseJointHandle

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))

        .add_plugins(
            (
            DefaultRaycastingPlugin,
            WorldInspectorPlugin::new()
            )
        )
        //.insert_resource()
        //.add_plugins(SerializationPlugin)
        .register_type::<Selectable>()
        .add_systems(Update, visualize_window_for::<Selected>)

        .add_systems(Update, selector_raycast)
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, create_revolute_joints)
        //
        .add_plugins(SerializationPlugin)
        .add_plugins(PhysicsSerializationPlugin)
        .add_systems(Update, motor_controller)
        //.add_systems(Update, display_rapier_joint_info)

        .run();
}


// pub trait EditorTool {
//     fn ui(&mut self, ui: &mut bevy_egui::egui::Ui, commands: &mut Commands, world: &mut World);
//     fn name(&self) -> &str;
// }

// pub trait ToolExt {
//     fn editor_tool<T>(&mut self, tool: T)
//     where
//         T: EditorTool + Send + Sync + 'static;
// }

// impl ToolExt for App {
//     fn editor_tool<T>(&mut self, tool: T)
//     where
//         T: EditorTool + Send + Sync + 'static,
//     {
//         self.world
//             .resource_mut::<GameViewTab>()
//             .tools
//             .push(Box::new(tool));
//     }
// }

#[derive(Component, Reflect)]
pub struct Selectable;

// #[derive(Component, Reflect)]
// pub struct Selectable {
//     pub selected: bool,
// }

#[derive(Component, Default)]
pub struct Selected;

// impl Default for Selectable {
//     fn default() -> Self {
//         Self {
//             selected: true,
//         }
//     }
// }
// #[derive(Resource, Default)]
// pub struct SelectedJoints {
//     pub selected_joints: HashMap<String, bool>
// }

// take selected things, and display their info

pub fn gizmo_tool() {
        //FIXME: this will crash if there are multiple cameras
    // let (cam_transform, cam_proj) = editor_camera.single();

    // let view_matrix = Mat4::from(cam_transform.affine().inverse());

    // // if multiple_pressed {
    // //     let mut mean_transform = Transform::IDENTITY;
    // //     for e in &selected {
    // //         let Some(ecell) = cell.get_entity(*e) else {
    // //             continue;
    // //         };
    // //         let Some(global_transform) = (unsafe { ecell.get::<GlobalTransform>() }) else {
    // //             continue;
    // //         };
    // //         let tr = global_transform.compute_transform();
    // //         mean_transform.translation += tr.translation;
    // //         mean_transform.scale += tr.scale;
    // //     }
    // //     mean_transform.translation /= selected.len() as f32;
    // //     mean_transform.scale /= selected.len() as f32;

    // let averaged_translations = {
    //     let mut collected_trans = Transform::IDENTITY;

    //     for (selectable, transform) in selectables.iter() {
    //         collected_trans.translation += transform.translation;
    //         collected_trans.scale += transform.scale;
    //     }
    //     let len = selectables.iter().len() as f32;
    //     collected_trans.translation /= len;
    //     collected_trans.scale /= len;


    //     collected_trans
    // };
    // if let Some(result) = egui_gizmo::Gizmo::new("Selected gizmo mean global".to_string())
    //     .projection_matrix(cam_proj.get_projection_matrix().to_cols_array_2d().into())
    //     .view_matrix(view_matrix.to_cols_array_2d().into())
    //     .model_matrix(averaged_translations.compute_matrix().to_cols_array_2d().into())
    //     .mode(egui_gizmo::GizmoMode::Translate)
    //     .interact(ui)
    // {
    //     mean_transform = Transform {
    //         translation: Vec3::from(<[f32; 3]>::from(result.translation)),
    //         rotation: Quat::from_array(<[f32; 4]>::from(result.rotation)),
    //         scale: Vec3::from(<[f32; 3]>::from(result.scale)),
    //     };
    //     disable_pan_orbit = true;
    // }

}

pub fn motor_controller(
    mut selected_joints: Query<&mut JointFlag, With<Selected>>,
    keyboard: Res<Input<KeyCode>>
) {
    if keyboard.pressed(KeyCode::W) {
        for mut joint in selected_joints.iter_mut() {
            joint.motors[3].target_vel += 1.0;
        }
    }       
    if keyboard.pressed(KeyCode::S) {
        for mut joint in selected_joints.iter_mut() {
            joint.motors[3].target_vel += -1.0;
        }  
    }
}

pub fn selector_raycast(
    cursor_ray: Res<CursorRay>, 
    mut raycast: Raycast, 
    mouse_press: Res<Input<MouseButton>>,
    mut selectables: Query<(Entity, &mut Selectable, &Transform)>,
    editor_camera: Query<(&GlobalTransform, &Projection)>,
    selected: Query<Entity, &Selected>,
    mut commands: Commands,
) {


    if let Some(cursor_ray) = **cursor_ray {
        let hits = raycast.cast_ray(cursor_ray, &default());
        //println!("casting ray")
        for (e, hit) in hits.iter() {
            if mouse_press.just_pressed(MouseButton::Left) {
                println!("clicked {:#?}", e);
                if let Ok((e, mut selectable, trans)) = selectables.get_mut(e.clone()) {
                    match selected.get(e) {
                        Ok(..) => commands.entity(e).remove::<Selected>(),
                        Err(..) => commands.entity(e).insert(Selected)
                    };
                    // if selectable.selected == true {
                    //     selectable.selected = false;
                    // }
                    // else if selectable.selected == false {
                    //     selectable.selected = true;
                    // }
                } 
            }
        }

    }
}

/// widget for controlling selected joints

// pub fn display_rapier_joint_info(
//     mut rapier_joint_window: Query<&mut EguiContext, With<PrimaryWindow>>,
//     mut rapier_joints: Query<(&mut Selectable, &ImpulseJoint)>,
// ) {
//     for mut context in rapier_joint_window.iter_mut() { 
//         egui::Window::new("Rapier Joint Info textbox")
//         .show(context.get_mut(), |ui|{

//             for (i, (mut selectable, joint)) in rapier_joints.iter_mut().enumerate() {
                
                
//                 let selected = selectable.selected.clone();
//                 let window_size =                 ui.input(|i| i.viewport().outer_rect);
//                 ui.checkbox(&mut selectable.selected, "");
//                 if selected {
//                     ScrollArea::vertical()
//                     //.max_height(window_size.unwrap_or(Rect{min: Pos2::default(), max: Pos2::default()}).height())
//                     .max_height(500.0)
//                     .id_source(i.to_string() + "_joint")
//                     .show(
//                         ui, |ui| {
//                             let joint_as_string = format!("{:#?}", joint);
//                             let job = LayoutJob::single_section(
//                                 joint_as_string,
//                                 TextFormat::default()
//                             );
//                             if ui.button("Copy to clipboard").clicked() {
//                                 ui.output_mut(|o| o.copied_text = String::from(job.text.clone()));
//                             }
//                             ui.label(job.clone());
//                         }
//                     )
//                     ;
//                 }
//             }
//         })
//         ;
//     }
// }


pub fn setup_graphics(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(origin.x - 5.0, origin.y , origin.z)
            .looking_at(Vec3::new(13.0, 1.0, 1.0), Vec3::Y),
        ..Default::default()
    });
}

const CUBE_COUNT: usize = 2;
const origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const num: usize = 1;

fn create_revolute_joints(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
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
            }
            //meshes.add(shape::Cube::new(0.5).into())
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
                    }
                    //Collider::cuboid(rad, rad, rad),
                ))
                .id();
        }

        // Setup four joints.
        let x = Vec3::X;
        let z = Vec3::Z;

        let revs = [
            RevoluteJointBuilder::new(z).local_anchor2(Vec3::new(0.0, 0.0, -shift))
            .motor_velocity(100.0, 20.0)
            ,
            RevoluteJointBuilder::new(x).local_anchor2(Vec3::new(-shift, 0.0, 0.0))
            
            //.inse,
            // RevoluteJointBuilder::new(z).local_anchor2(Vec3::new(0.0, 0.0, -shift)),
            // RevoluteJointBuilder::new(x).local_anchor2(Vec3::new(shift, 0.0, 0.0)),
        ];

        commands
            .entity(handles[0])
            .insert(ImpulseJoint::new(curr_parent, revs[0]))
            .insert(Selectable)
            ;
        commands
            .entity(handles[1])
            .insert(ImpulseJoint::new(handles[0], revs[1]))
            .insert(Selectable)
            ;
        // commands
        //     .entity(handles[2])
        //     .insert(ImpulseJoint::new(handles[1], revs[2]));
        // commands
        //     .entity(handles[3])
        //     .insert(ImpulseJoint::new(handles[2], revs[3]));

        curr_parent = *handles.last().unwrap();
    }
}
