

# bevy_serialization_extras 0.9

bevy_serialization_extras has now been updated to 0.9!

This is a big update with a lot of refactors, new features, and quality life improvements.

# changes


### bevy_assemble refactors:

bevy_assemble's 0.1 release had a lot of rough edges around it. So, this update has been mostly focused on improvements to it.

#### renamed FromStructure into Disassemble, and impl refactors

[`FromStructure`] has been renamed to [`Disassemble`] to match the new composition over inheritance approach for asset deserialization. No longer are assets deserialized from one giant function, but instead, decomposed into:

- their constituent components
- further [`DisassembleAssetRequest<T>`]s


World access based initialization was a source of many footguns and fights with the borrow checker.

For disassembly, structures are disassembled into a `Structure<impl Bundle>` that holds either the components of the structure, or a collection of bundles which hold the components of its children. 

E.G: A request is made to spawn a robot via urdf

```rust
// request to spawn a robot saved in the URDF format.
    commands.spawn((
        RequestAssetStructure::<UrdfWrapper>::Path("root://model_pkg/urdf/diff_bot.xml".to_owned()),
        Transform::from_xyz(-2.0, 0.0, 0.0),
    ));
```

and this wrapper is disassembled into either its constituent components, or its constituent children.

```rust
impl Disassemble for UrdfWrapper {
    fn components(value: Self) -> Structure<impl Bundle> {
        let mut structured_joint_map = HashMap::new();

        for joint in &value.0.joints {
            structured_joint_map.insert(joint.child.link.clone(), joint.clone());
        }

        let mut linkage = Vec::new();
        for link in value.0 .0.links {
            linkage.push((
                link.clone(),
                structured_joint_map
                    .get_key_value(&link.name)
                    .map(|(_, joint)| joint.clone()),
            ))
        }
        Structure::Root((
            Name::new(value.0 .0.name),
            RequestStructure(LinksNJoints(linkage)),
        ))
    }
}
...

#[derive(Clone, Deref)]
pub struct LinksNJoints(Vec<(Link, Option<Joint>)>);

impl Disassemble for LinksNJoints {
    fn components(value: Self) -> Structure<impl Bundle> {
        let mut children = Vec::new();

        for (link, joint) in value.0 {
            let joint = joint.map(|n| DisassembleRequest(UrdfJoint(n)),
            );
            children.push((
                Name::new(link.name),
                DisassembleRequest(LinkColliders(link.collision)),
                Maybe(joint),
                Visibility::default(),
            ))
        }
        Structure::Children(children, Split(true))
    }
}
```

So on and so forth untill all `DisassembleRequest`s and `DisassembleAssetRequest`s have been disassembled into their underlying components.



### Assset saving systems implemented, and IntoHashMap renamed into [`Assemble`]

- 
    IntoHashMap is now [`Assemble`], and Assembling assets is now properly implemented!

    Previously, the systems in charge for this were skeletons, but they have now been properly implemented to allow asset saving! Implement [`Assemble`] on an asset wrapper around your asset, and you can save into that asset's file format!

    See crate's `/urdf` module for example impl


- 
    Post impl, assets can now be saved via the [`AssembleRequest<T>`] resource on `Assemble` implementing asset wrappers E.G to serialize a urdf:
    ```rust
    let request = AssembleRequest::<UrdfWrapper>::new(
        // file name
        "diff_bot".into(),
        // assetSource string literal id
        "saves".to_string(),
        // selected entities to save into asset
        entities.clone(),
    );
    ```

### bevy_synonymize refactors:

#### trait bound consolidation
wrapper components have had their trait bounds consolidated under the new [`WrapperComponent`] trait.

wrapper components around assets have had their trait bounds consolidated under [`AssetWrapper`]
               
### Mandatory pure/path variants for wrappers + Simplified trait impls

[`FromWrapper`] has been removed in favor of From impls from your asset wrapper to asset directly.

In exchange, [`AssetWrapper`]s must now be enums with a `PureVariant` and path variant (from string). 

E.G:

```rust
#[derive(Component, Reflect, From)]
#[reflect(Component)]
pub enum Mesh3dFlag {
    Path(String),
    Pure(MeshWrapper),
}
```

### bevy_synonymize_physics refactors:

- AsyncColliderFlag has been removed in favor of [`ColliderFlag`]. If you wish to initialize AsyncColliderFlag, use AsyncCollider from rapier.

- [`JointFlag`] no longer has a name attribute to prevent truth desync