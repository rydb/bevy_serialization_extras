/// collect entities with `ModelFlag` that don't have meshes, and spawn their meshes.  
pub fn spawn_models(
    unspawned_models_query: Query<(Entity, &ModelFlag, &Transform), Without<Handle<Mesh>>>,
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assset_server: Res<AssetServer>,
    //transform_query: Query<&Transform>,
) {
    for (e, model, trans) in unspawned_models_query.iter() {
        println!("spawning model");
        let mesh_check: Option<Mesh> = match model.geometry.clone() {
            Geometry::Primitive(variant) => Some(variant.into()), 
            Geometry::Mesh { filename, .. } => {
                println!("attempting to load mesh: {:#?}", filename);
                meshes.get(&assset_server.load(filename))}.cloned()
        }; 
        if let Some(mesh) = mesh_check {
            let mesh_handle = meshes.add(mesh);

            let material_handle = materials.add(model.material.clone());
            //(*model).serialize()
            commands.entity(e)
            .insert(
                (
                PbrBundle {
                    mesh: mesh_handle,
                    material: material_handle,
                    transform: *trans,
                    ..default()
                }, // add mesh
                MakeSelectableBundle::default(), // makes model selectable 
                Unload, // marks entity to unload on deserialize
            )
            )

       
            ;
            // match model.physics {
            //     Physics::Dynamic => {
            //         commands.entity(e).insert(PhysicsBundle::default());
            //     },
            //     Physics::Fixed => {
            //         commands.entity(e).insert(
            //             PhysicsBundle {
            //             rigid_body: RigidBody::Fixed,
            //             ..default() 
            //         }
            //         );
            //     }
            // }
        } else {
            println!("load attempt failed for this mesh, re-attempting next system call");
        }

        

    }
}