//use bevy::prelude::*;

use std::{collections::HashMap, primitive};

use bevy_core::prelude::*;
use bevy_ecs::prelude::*;
use bevy_hierarchy::Children;
use bevy_log::{info, warn};
use bevy_render::mesh::Mesh3d;
use bevy_transform::prelude::*;

use crate::prelude::{link::{
    GeometryShiftMarked, GeometryShifted, JointBounded, JointFlag, LinkFlag,
}, RigidBodyFlag};





// /// shifts local frame to match link offset
// pub fn local_frame2_shift(
//     mut unshifted_models: Query<
//         (Entity, &LinkFlag, &mut JointFlag),
//         (Without<GeometryShifted>, With<GeometryShiftMarked>),
//     >,
//     //mut meshes: ResMut<Assets<Mesh>>,
//     mut commands: Commands,
// ) {
//     for (e, link_flag, mut joint_flag) in unshifted_models.iter_mut() {
//         info!("shifting model local_frame 2");
//         // match joint_flag.local_frame2 {
//         //     Some(local_frame2) => {

//         //     }
//         // }
//         //joint_flag.local_frame1.translation += link_flag.geom_offset;
//         joint_flag.local_frame2 = Transform::from_translation(-link_flag.geom_offset);
//         commands.entity(e).insert(GeometryShifted::default());
//     }
// }
