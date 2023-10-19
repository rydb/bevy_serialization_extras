use bevy::{prelude::*, window::{WindowResolution, PresentMode}, ecs::{world, system::{SystemParam, SystemState, CombinatorSystem}, schedule::SystemConfigs}};
use bevy_egui::EguiContext;
use egui::{RichText, Color32};
use std::collections::HashMap;
use moonshine_save::{prelude::Save, save::SaveSet};
use std::any::TypeId;

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
                    title: "Second window".to_owned(),
                    resolution: WindowResolution::new(800.0, 600.0),
                    present_mode: PresentMode::AutoVsync,
                    ..Default::default()
                },
                SerializeWindowMarker,
            )
        );
    }
}

pub fn fetch_type_data(
    world: &mut World,
) -> (HashMap::<TypeId, String>, HashMap::<TypeId, String>){
    let mut enetities_to_save = world.query_filtered::<Entity, With<Save>>();
    
    println!("fetching type data");

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
    
    return (registered_types, saved_component_types)
} 

pub fn manage_serialization_ui(
    registered_and_component_types: In<(HashMap::<TypeId, String>, HashMap::<TypeId, String>)>,
    mut window_marker_query: Query<&mut EguiContext, With<SerializeWindowMarker>>
) {
    let registered_types = registered_and_component_types.0.0;
    let saved_components = registered_and_component_types.0.1;
    
    println!("populating ui menu");

    for mut context in window_marker_query.iter_mut() {
        egui::CentralPanel::default()
        .show(context.get_mut(), |ui|{
            let table = TableBuilder::new(ui);
                table
                .striped(true)
                .resizable(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::auto())
                .column(Column::initial(100.0).range(40.0..=300.0))
                .column(Column::initial(100.0).at_least(40.0).clip(true))
                .column(Column::remainder())
                .min_scrolled_height(0.0)
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.heading("Components to serialize");
                    });
                    // header.col(|ui| {
                    //     ui.heading("Components to serialize");
                    // });
                    header.col(|ui| {
                        ui.heading(RichText::new("Serializable ✔️").color(Color32::GREEN));
                    });
                    header.col(|ui| {
                        ui.heading(RichText::new("Unserializable ✖️").color(Color32::RED));
                    });
                })
                .body(|mut body| {
                    for (type_id, name) in saved_components.iter() {
                        body.row(30.0, |mut row| {
                        
                            row.col(|ui| {
                                ui.label(format!("{:#?}", name));

                            });
                            row.col(|ui| {
                                //ui.button("world!");
                            });
                        });
                    }

                })
                ;
            //ui.heading("serializable list");
        }
        ); 
        
    };
}


pub fn visualize_serializable(

) {
    println!("running visualize serializable");
    fetch_type_data
    .pipe(manage_serialization_ui)
    ;
}

/// Lists all components on entities marked with [`Saved`], but which have components which are unsavable for whatever reason.
pub fn list_unserializable_window(
    world: &mut World,
    system_state: &mut SystemState<( // Notice the &mut here
        Res<AppTypeRegistry>,
        Query<Entity, With<Save>>,
        Query<&mut EguiContext, With<SerializeWindowMarker>>
    )>
) {
    // let (
    //     type_registry,
    //     entities_to_serialize,
    //     mut window_marker_query,
    // ) = system_state.get_mut(world);

    // let mut saved_component_types = HashMap::new();
    // for e in entities_to_serialize.iter() {
    //     for component in world.entity(e).archetype().components() {

    //         let comp_info = world.components().get_info(component).unwrap();
    //         saved_component_types.insert(comp_info.type_id().unwrap(), comp_info.name().to_owned());
    //     }
    // }

    // let registered_types = type_registry.read().iter()
    // .map(|id| {
    //     let type_id = id.type_id();

    //     return (type_id, id.type_name().to_owned())
    // })
    // .collect::<HashMap<TypeId, String>>();


    // for mut context in window_marker_query.iter_mut() {

    //     egui::CentralPanel::default()
    //     .show(context.get_mut(), |ui|{
    //         let table = TableBuilder::new(ui);
    //             table
    //             .striped(true)
    //             .resizable(true)
    //             .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
    //             .column(Column::auto())
    //             .column(Column::initial(100.0).range(40.0..=300.0))
    //             .column(Column::initial(100.0).at_least(40.0).clip(true))
    //             .column(Column::remainder())
    //             .min_scrolled_height(0.0)
    //             .header(20.0, |mut header| {
    //                 header.col(|ui| {
    //                     ui.heading("Components to serialize");
    //                 });
    //                 // header.col(|ui| {
    //                 //     ui.heading("Components to serialize");
    //                 // });
    //                 header.col(|ui| {
    //                     ui.heading(RichText::new("Serializable ✔️").color(Color32::GREEN));
    //                 });
    //                 header.col(|ui| {
    //                     ui.heading(RichText::new("Unserializable ✖️").color(Color32::RED));
    //                 });
    //             })
    //             .body(|mut body| {
    //                 for e in entities_to_serialize.iter() {
    //                     body.row(30.0, |mut row| {
                        
    //                         row.col(|ui| {
    //                             ui.label(format!("{:#?}", e));

    //                         });
    //                         row.col(|ui| {
    //                             //ui.button("world!");
    //                         });
    //                     });
    //                 }

    //             })
    //             ;
    //         //ui.heading("serializable list");
    //     }
    //     ); 
        
    // };
    }
    


pub fn list_unserializable_old(
    world: &mut World,
){
    let mut enetities_to_save = world.query_filtered::<Entity, With<Save>>();
    

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
    
    println!("Listing imports required for adding unregistered types to type registry: ");
    for item in saved_component_types.keys() {
        if registered_types.contains_key(item) == false {
            println!("use {:#}", saved_component_types[item])
        }
    }
    println!("listing .register_type::<T>'s for unregistered types. copy and paste this into app  ");
    for item in saved_component_types.keys() {
        //println!("listing component types marked to save {:#?}", saved_component_types[item]);
        if registered_types.contains_key(item) == false {
            println!(".register_type::<{:#}>()", 
            saved_component_types[item].split("::")
            .collect::<Vec<_>>()
            .last()
            .unwrap())
        }
    }
}