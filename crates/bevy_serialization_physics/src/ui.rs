use bevy::{prelude::*, window::PrimaryWindow, reflect::TypeInfo, ecs::schedule::{SystemConfig, SystemConfigs}};
use bevy_egui::EguiContext;
use bevy_rapier3d::dynamics::ImpulseJoint;
use bitvec::{view::BitView, order::Msb0, field::BitField};
use egui::{RichText, Color32, ScrollArea, text::LayoutJob, TextFormat, Align2, InnerResponse, Ui};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, IntoStaticStr, Display};
use std::collections::HashMap;
use moonshine_save::prelude::Save;
use std::any::TypeId;

use egui_extras::{Column, TableBuilder};

use crate::{loaders::urdf_loader::Urdf, wrappers::link::{JointFlag, JointAxesMaskWrapper}};



#[derive(Default, EnumIter, Display)]
pub enum UtilityType {
    Joints,
    #[default]
    UrdfInfo,
}
#[derive(Resource, Default)]
pub struct UtilitySelection {
    pub selected: UtilityType
}
#[derive(Resource, Default)]
pub struct CachedUrdf {
    pub urdf: Handle<Urdf>
}

pub fn physics_widgets_window(
    mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut utility_selection: ResMut<UtilitySelection>,
    mut asset_server: Res<AssetServer>,
    mut cached_urdf: Res<CachedUrdf>,
    mut urdfs: Res<Assets<Urdf>>,


    mut joint_flags: Query<&mut JointFlag>,

    rapier_joints: Query<&ImpulseJoint>,

) {
    for mut context in primary_window.iter_mut() { 
        egui::Window::new("debug widget window")
        //.title_bar(false)
        .show(context.get_mut(), |ui|{
            // lay out the ui widget selection menu
            ui.horizontal(|ui| {
                for utility in UtilityType::iter() {
                    if ui.button(utility.to_string()).clicked() {
                        utility_selection.selected = utility;
                    }
                }
            });

            match utility_selection.selected {
                UtilityType::Joints => {
                    for mut joint in joint_flags.iter_mut() {
                

                        ui.label("limit axis bits");
                        ui.horizontal(|ui| {
                            let mut limit_axis_bits = joint.limit_axes.bits().clone();
                            let limit_axis_bitvec = limit_axis_bits.view_bits_mut::<Msb0>();
        
                            for mut bit in limit_axis_bitvec.iter_mut(){
                                //let mut bit_value = bit;
                                
                                ui.checkbox(&mut bit, "");
            
            
                            }
                            let new_joint_mask = JointAxesMaskWrapper::from_bits_truncate(limit_axis_bitvec.load_le());
                            // stops component from being registered as changed if nothing is happening to it
                            if joint.limit_axes != new_joint_mask {
                                joint.limit_axes = new_joint_mask;
                            }        
                        });
                        
                        ui.label("locked axis bits");
                        ui.horizontal(|ui| {
                            let mut locked_axis_bits = joint.locked_axes.bits().clone();
                            let limit_axis_bitvec = locked_axis_bits.view_bits_mut::<Msb0>();
        
                            for mut bit in limit_axis_bitvec.iter_mut(){
                                //let mut bit_value = bit;
                                
                                ui.checkbox(&mut bit, "");
            
                            }
                            let new_joint_mask = JointAxesMaskWrapper::from_bits_truncate(limit_axis_bitvec.load_le());

                            if joint.locked_axes != new_joint_mask {
                                joint.locked_axes = new_joint_mask;
                            }       
                        });            
                        
                    }

                    
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
                }
                UtilityType::UrdfInfo => {
                    if let Some(urdf) = urdfs.get(cached_urdf.urdf.clone()) {
                        let urdf_as_string = format!("{:#?}", urdf.robot);
                        
                        if ui.button("Copy to clipboard").clicked() {
                            ui.output_mut(|o| o.copied_text = urdf_as_string.to_string());
                        }
                        ScrollArea::vertical().show(
                            ui, |ui| {

                                let job = LayoutJob::single_section(
                                    urdf_as_string,
                                    TextFormat::default()
                                );
                                ui.label(job);
                            }
                        );
                    }                    
                }
            }
        })

        ;
    }
}
