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
}, JointRequest, JointRequestStage, RigidBodyFlag};



// get joints and bind them to their named connection if it exists
pub fn bind_joint_request_to_parent(
    mut joints: Query<(Entity, &mut JointRequest), Without<JointBounded>>,
    link_names: Query<(Entity, &Name), (
        With<RigidBodyFlag>, 
        // JointFlag requires this to be initialized on the parent link to initialize properly        
        With<Transform>
    )>,
    decendents: Query<&Children>,
    
    meshes: Query<&Mesh3d>,
    mut commands: Commands,
) {
    for (e, request) in joints.iter() {
        let parent = match &request.stage {
            JointRequestStage::Name(parent) => {
                let name_matches = link_names
                .iter()
                .filter(|(e, name)| name.as_str() == parent)
                .map(|(e, n)| e)
                .collect::<Vec<_>>();
                //.collect::<HashMap<Entity, Vec<Name>>>();

                if name_matches.len() > 1 {
                    warn!("more than one entity which matches query and is named {:#?}, entities with same name: {:#?}", parent, name_matches);
                    return
                }
                let Some(parent) = name_matches.first() else {
                    return
                };
                parent.clone()

            },
            JointRequestStage::Entity(entity) => entity.clone(),
        };
        
        commands.entity(e).insert(
            JointFlag {
                parent: parent,
                joint: request.joint.clone()
            }
        );
        commands.entity(e).remove::<JointRequest>();

        // let joint_parent_name = request.0;


    }
}

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
