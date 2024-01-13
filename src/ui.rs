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

use crate::{resources::{TypeRegistryOnSave, ComponentsOnSave, ShowSerializable, ShowUnserializable, RefreshCounter}, wrappers::link::{JointFlag, JointAxesMaskWrapper}, loaders::urdf_loader::Urdf};
use egui_extras::{Column, TableBuilder};


#[derive(Default, EnumIter, Display)]
pub enum UtilityType {
    Joints,
    SerializableList,
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

pub fn debug_widgets_window(
    mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut utility_selection: ResMut<UtilitySelection>,
    saved_components: Res<ComponentsOnSave>,
    registered_types: Res<TypeRegistryOnSave>,
    mut refresh_counter: ResMut<RefreshCounter>,
    mut show_serializable: ResMut<ShowSerializable>,
    mut show_unserializable: ResMut<ShowUnserializable>,
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
