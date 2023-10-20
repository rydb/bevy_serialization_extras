use bevy::{prelude::*, window::{WindowResolution, PresentMode}};
use bevy_egui::EguiContext;
use egui::{RichText, Color32};
use std::collections::HashMap;
use moonshine_save::prelude::Save;
use std::any::TypeId;

use crate::resources::{TypeRegistryOnSave, ComponentsOnSave, ShowSerializable, ShowUnserializable};
use egui_extras::{Column, TableBuilder};

#[derive(Component)]
pub struct SerializeWindowMarker;

pub fn spawn_unserializable_window(
    mut commands: Commands,
    window_marker_query: Query<&mut EguiContext, With<SerializeWindowMarker>>,
) {
    if window_marker_query.is_empty() {
        commands.spawn(
            (
                Window {
                    title: "Serializable components list".to_owned(),
                    resolution: WindowResolution::new(800.0, 600.0),
                    present_mode: PresentMode::AutoVsync,
                    ..Default::default()
                },
                SerializeWindowMarker,
            )
        );
    }
}

pub fn update_last_saved_typedata(
    world: &mut World,
) {
    let mut enetities_to_save = world.query_filtered::<Entity, With<Save>>();
    
    println!("updaitng last saved type_data");

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

        return (type_id, id.type_name().to_owned())
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
    // registered_types, saved_component_types
} 

pub fn manage_serialization_ui(
    //registered_and_component_types: In<(HashMap::<TypeId, String>, HashMap::<TypeId, String>)>,
    saved_components: Res<ComponentsOnSave>,
    registered_types: Res<TypeRegistryOnSave>,

    mut show_serializable: ResMut<ShowSerializable>,
    mut show_unserializable: ResMut<ShowUnserializable>,

    mut window_marker_query: Query<&mut EguiContext, With<SerializeWindowMarker>>
) {
    for mut context in window_marker_query.iter_mut() {
        egui::CentralPanel::default()
        .show(context.get_mut(), |ui|{
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
                        });
                        ui.heading("Components to serialize");
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
        ); 
        
    };
}
