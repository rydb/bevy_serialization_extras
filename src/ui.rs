use bevy::{prelude::*, window::PrimaryWindow, reflect::TypeInfo, ecs::schedule::{SystemConfig, SystemConfigs}};
use bevy_egui::EguiContext;
use bevy_rapier3d::dynamics::ImpulseJoint;
use bitvec::{view::BitView, order::Msb0, field::BitField};
use egui::{RichText, Color32, ScrollArea, text::LayoutJob, TextFormat, Align2, InnerResponse, Ui};
use std::collections::HashMap;
use moonshine_save::prelude::Save;
use std::any::TypeId;

use crate::{resources::{TypeRegistryOnSave, ComponentsOnSave, ShowSerializable, ShowUnserializable, RefreshCounter}, wrappers::link::{JointFlag, JointAxesMaskWrapper}};
use egui_extras::{Column, TableBuilder};

#[derive(Default)]
pub enum UtilityType {
    #[default]
    Joints,
    SerializableList
}
#[derive(Resource, Default)]
pub struct UtilitySelection {
    pub selected: UtilityType
}

pub fn debug_widgets_window(
    mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut utility_selection: ResMut<UtilitySelection>,
    saved_components: Res<ComponentsOnSave>,
    registered_types: Res<TypeRegistryOnSave>,
    mut refresh_counter: ResMut<RefreshCounter>,
    mut show_serializable: ResMut<ShowSerializable>,
    mut show_unserializable: ResMut<ShowUnserializable>,


    mut joint_flags: Query<&mut JointFlag>,

    rapier_joints: Query<&ImpulseJoint>,

) {
    for mut context in primary_window.iter_mut() { 
        egui::Window::new("debug widget window")
        //.title_bar(false)
        .show(context.get_mut(), |ui|{
            ui.horizontal(|ui| {
                if ui.button("Joints").clicked() {
                    utility_selection.selected = UtilityType::Joints;     
                };
                if ui.button("SerializableList").clicked() {
                    utility_selection.selected = UtilityType::SerializableList;
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
                            joint.limit_axes = JointAxesMaskWrapper::from_bits_truncate(limit_axis_bitvec.load_le());
        
                        });
                        
                        ui.label("locked axis bits");
                        ui.horizontal(|ui| {
                            let mut locked_axis_bits = joint.locked_axes.bits().clone();
                            let limit_axis_bitvec = locked_axis_bits.view_bits_mut::<Msb0>();
        
                            for mut bit in limit_axis_bitvec.iter_mut(){
                                //let mut bit_value = bit;
                                
                                ui.checkbox(&mut bit, "");
            
                            }
                            joint.locked_axes = JointAxesMaskWrapper::from_bits_truncate(limit_axis_bitvec.load_le());
        
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
                UtilityType::SerializableList => {
                    let table = TableBuilder::new(ui);
                    table
                    .striped(true)
                    .resizable(true)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::auto())
                    .min_scrolled_height(0.0)
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.horizontal(|ui| {
                                ui.checkbox(&mut show_serializable.check, "show savable");
                                ui.checkbox(&mut show_unserializable.check, "show unsavable");
                                if ui.button("refresh").clicked() {
                                    refresh_counter.counter += 1;
                                }
                            });
    
                        });
                    })
                    .body(|mut body| {
                        for (type_id, name) in saved_components.components.iter() {
                            if registered_types.registry.contains_key(type_id) {
                                if show_serializable.check == true {
                                    body.row(30.0, |mut row| {
                                        row.col(|ui| {
                                            ui.label(RichText::new(name).color(Color32::GREEN));
                                        })
                                        ;
                                    })
                                }
                            } else {
                                if show_unserializable.check == true {
                                    body.row(30.0, |mut row| {
                                        row.col(|ui| {
                                            ui.label(RichText::new(name).color(Color32::RED));
                                        })
                                        ;
                                    })
                                }
                            }
                        }
    
                    })
                    ;
                }
            }
        })

        ;
    }
}

pub fn update_last_saved_typedata(
    world: &mut World,
) {
    let mut enetities_to_save = world.query_filtered::<Entity, With<Save>>();
    
    //println!("updating last saved type_data");

    let type_registry = world.resource::<AppTypeRegistry>();

    let mut saved_component_types = HashMap::new();
    for e in enetities_to_save.iter(&world) {
        for component in world.entity(e).archetype().components() {

            let comp_info = world.components().get_info(component).unwrap();
            saved_component_types.insert(comp_info.type_id().unwrap(), comp_info.name().to_owned());
        }
    }

    let registered_types = type_registry.read().iter()
    .map(|id| {
        let type_id = id.type_id();

        return (type_id, TypeInfo::type_path(id.type_info()).to_owned())
    })
    .collect::<HashMap<TypeId, String>>();
    
    type L = TypeRegistryOnSave;
    world.insert_resource::<L>(
        L {
            registry: registered_types
        }
    );
    type O = ComponentsOnSave;
    world.insert_resource::<O>(
        O {
            components: saved_component_types
        }
    );
} 
