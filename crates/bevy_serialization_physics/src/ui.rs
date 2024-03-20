use bevy_egui::EguiContext;
use bevy_rapier3d::dynamics::ImpulseJoint;
use bevy_ui_extras::stylesheets::DEBUG_FRAME_STYLE;
use bevy_window::PrimaryWindow;
use bitvec::prelude::Msb0;
use bitvec::{field::BitField, view::BitView};
use egui::{text::LayoutJob, ScrollArea, TextFormat, Ui};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

use crate::prelude::link::{JointAxesMaskWrapper, JointFlag};

use bevy_reflect::prelude::*;
use bevy_ecs::prelude::*;
use bevy_input::prelude::*;


const DASHES: usize = 5;

#[derive(Component, Reflect)]
pub struct Selectable;

#[derive(Resource, Default, Clone, Copy, PartialEq)]
pub struct SelectedMotorAxis {
    pub axis: MotorAxis,
}

#[derive(Component, Default)]
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

// A collection of utilities that make debugging physics easier
pub fn physics_utilities_ui(
    mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut utility_selection: ResMut<PhysicsUtilitySelection>,

    mut selected_joints: Query<(Entity, &mut JointFlag), With<Selected>>,
) {
    for mut context in primary_window.iter_mut() {
        egui::Window::new("Physics Utilities")
            //.title_bar(false)
            .frame(DEBUG_FRAME_STYLE)
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
            .frame(DEBUG_FRAME_STYLE)
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

pub fn rapier_joint_info_ui(
    mut rapier_joint_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut rapier_joints: Query<&ImpulseJoint, With<Selected>>,
) {
    for mut context in rapier_joint_window.iter_mut() {
        egui::Window::new("Rapier Joint Info textbox")
            .frame(DEBUG_FRAME_STYLE)
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
