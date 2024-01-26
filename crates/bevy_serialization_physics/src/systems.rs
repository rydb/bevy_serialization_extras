use bevy::prelude::*;

use crate::prelude::link::{JointFlag, LinkFlag, JointBounded, GeometryShifted, GeometryShiftMarked};



// get joints and bind them to their named connection if it exists
pub fn bind_joints_to_entities(
    mut joints: Query<(Entity, &mut JointFlag), (Without<JointBounded>)>,
    link_names: Query<(Entity, &Name)>,
    mut commands: Commands,
) {
    for (joint_e, mut joint) in joints.iter_mut() {
        let joint_parent_name = joint.parent_name.clone();
        match joint_parent_name {
            Some(name) => {
                for (i, (e, link_name)) in link_names.iter()
                .filter(|(e, link_name)| name == link_name.to_string())
                .enumerate() {
                    if i > 0 {
                        panic!("more then 1 entity with joint name that this joint can bind to! to prevent undefined behaviour, erroring here!")
                    }
                    if joint.parent_id != Some(e) {
                        joint.parent_id = Some(e);
                        commands.entity(joint_e).insert(JointBounded::default());
                    }

                }
            }
            None => {}
        }
    }
}


/// shifts local frame to match link offset
pub fn local_frame2_shift(
    mut unshifted_models: Query<(Entity, &LinkFlag, &mut JointFlag), (Without<GeometryShifted>, With<GeometryShiftMarked>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands
) {
    for (e, link_flag, mut joint_flag) in unshifted_models.iter_mut() {
        println!("shifting model local_frame 2");
        // match joint_flag.local_frame2 {
        //     Some(local_frame2) => {
                
        //     }
        // }
        //joint_flag.local_frame1.translation += link_flag.geom_offset;
        joint_flag.local_frame2 = Some(Transform::from_translation(-link_flag.geom_offset));
        commands.entity(e).insert(GeometryShifted::default());
    }
}

// pub fn gizmo_tool() {
//     //FIXME: this will crash if there are multiple cameras
// // let (cam_transform, cam_proj) = editor_camera.single();

// // let view_matrix = Mat4::from(cam_transform.affine().inverse());

// // // if multiple_pressed {
// // //     let mut mean_transform = Transform::IDENTITY;
// // //     for e in &selected {
// // //         let Some(ecell) = cell.get_entity(*e) else {
// // //             continue;
// // //         };
// // //         let Some(global_transform) = (unsafe { ecell.get::<GlobalTransform>() }) else {
// // //             continue;
// // //         };
// // //         let tr = global_transform.compute_transform();
// // //         mean_transform.translation += tr.translation;
// // //         mean_transform.scale += tr.scale;
// // //     }
// // //     mean_transform.translation /= selected.len() as f32;
// // //     mean_transform.scale /= selected.len() as f32;

// // let averaged_translations = {
// //     let mut collected_trans = Transform::IDENTITY;

// //     for (selectable, transform) in selectables.iter() {
// //         collected_trans.translation += transform.translation;
// //         collected_trans.scale += transform.scale;
// //     }
// //     let len = selectables.iter().len() as f32;
// //     collected_trans.translation /= len;
// //     collected_trans.scale /= len;


// //     collected_trans
// // };
// // if let Some(result) = egui_gizmo::Gizmo::new("Selected gizmo mean global".to_string())
// //     .projection_matrix(cam_proj.get_projection_matrix().to_cols_array_2d().into())
// //     .view_matrix(view_matrix.to_cols_array_2d().into())
// //     .model_matrix(averaged_translations.compute_matrix().to_cols_array_2d().into())
// //     .mode(egui_gizmo::GizmoMode::Translate)
// //     .interact(ui)
// // {
// //     mean_transform = Transform {
// //         translation: Vec3::from(<[f32; 3]>::from(result.translation)),
// //         rotation: Quat::from_array(<[f32; 4]>::from(result.rotation)),
// //         scale: Vec3::from(<[f32; 3]>::from(result.scale)),
// //     };
// //     disable_pan_orbit = true;
// // }

// }