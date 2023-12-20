use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::EguiContext;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use bevy_serialization_extras::plugins::SerializationPlugin;
use egui::{ScrollArea, text::LayoutJob, TextFormat};

//RapierImpulseJointHandle

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            0xF9 as f32 / 255.0,
            0xF9 as f32 / 255.0,
            0xFF as f32 / 255.0,
        )))
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        .add_plugins(SerializationPlugin)
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, create_revolute_joints)
        .add_plugins(WorldInspectorPlugin::new())
        //.add_plugins(SerializationPlugin)
        .add_systems(Update, display_rapier_joint_info)
        .run();
}

pub fn display_rapier_joint_info(
    mut rapier_joint_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    rapier_joints: Query<&ImpulseJoint>,
) {
    for mut context in rapier_joint_window.iter_mut() { 
        egui::Window::new("Rapier Joint Info textbox")
        .show(context.get_mut(), |ui|{
            //println!("number of joints {:#?}", rapier_joints.iter().len());
            if let Some(joint) = rapier_joints.iter().last() {
                ScrollArea::vertical().show(
                    ui, |ui| {
                        let joint_as_string = format!("{:#?}", joint);
                        let job = LayoutJob::single_section(
                            joint_as_string,
                            TextFormat::default()
                        );
                        if ui.button("Copy to clipboard").clicked() {
                            ui.output_mut(|o| o.copied_text = String::from(job.text.clone()));
                        }
                        ui.label(job.clone());

                    }
                );
            }
            // for joint in rapier_joints.iter() {


            // }
        })
        ;
    }
}


pub fn setup_graphics(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(15.0, 5.0, 42.0)
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
            TransformBundle::from(Transform::from_xyz(origin.x, origin.y, 0.0)),
            RigidBody::Fixed,
            AsyncCollider::default(),
            meshes.add(shape::Cube::new(0.5).into())
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
                    TransformBundle::from(Transform::from_translation(positions[k])),
                    RigidBody::Dynamic,
                    AsyncCollider::default(),
                    meshes.add(shape::Cube::new(0.5).into())
                    //Collider::cuboid(rad, rad, rad),
                ))
                .id();
        }

        // Setup four joints.
        let x = Vec3::X;
        let z = Vec3::Z;

        let revs = [
            RevoluteJointBuilder::new(z).local_anchor2(Vec3::new(0.0, 0.0, -shift)),
            RevoluteJointBuilder::new(x).local_anchor2(Vec3::new(-shift, 0.0, 0.0)),
            RevoluteJointBuilder::new(z).local_anchor2(Vec3::new(0.0, 0.0, -shift)),
            RevoluteJointBuilder::new(x).local_anchor2(Vec3::new(shift, 0.0, 0.0)),
        ];

        commands
            .entity(handles[0])
            .insert(ImpulseJoint::new(curr_parent, revs[0]));
        commands
            .entity(handles[1])
            .insert(ImpulseJoint::new(handles[0], revs[1]));
        // commands
        //     .entity(handles[2])
        //     .insert(ImpulseJoint::new(handles[1], revs[2]));
        // commands
        //     .entity(handles[3])
        //     .insert(ImpulseJoint::new(handles[2], revs[3]));

        curr_parent = *handles.last().unwrap();
    }
}


// pub fn setup_physics(mut commands: Commands) {
//     //create_prismatic_joints(&mut commands, Vec3::new(20.0, 10.0, 0.0), 5);
//     create_revolute_joints(&mut commands, Vec3::new(0.0, 0.0, 0.0), 1);
//     //create_fixed_joints(&mut commands, Vec3::new(0.0, -5.0, 0.0), 6);
//     //create_rope_joints(&mut commands, Vec3::new(30.0, 10.0, 0.0), 5);
//     //create_ball_joints(&mut commands, 15);
// }