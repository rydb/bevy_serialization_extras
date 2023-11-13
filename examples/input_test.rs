use bevy::{prelude::*, input::InputPlugin};
use bevy::log::LogPlugin;
use bevy::core::TaskPoolPlugin;
use bevy::prelude::TypeRegistrationPlugin;
fn main() {

    App::new()
    .add_plugins(DefaultPlugins)
    // .add_plugins(LogPlugin::default())
    // .add_plugins(TaskPoolPlugin::default())
    // .add_plugins(TypeRegistrationPlugin)
    // .add_plugins(InputPlugin)
    .add_systems(Update, press_w)
    .run();
}

fn press_w(
    keyboard: Res<Input<KeyCode>>
) {
    if keyboard.pressed(KeyCode::W) {
        println!("pressed w!")
    }
    if keyboard.pressed(KeyCode::A) {
        println!("pressed a!")
    }

    //println!("pressing {:#?}", keyboard)
}
