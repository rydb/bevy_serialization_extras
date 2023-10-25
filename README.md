# bevy_serialization_extras
A library for bevy which contains wrappers/systems for managing serialization in bevy with [`moonshine-save`](https://github.com/Zeenobit/moonshine_save) for a more "hands off" serialization workflow


[demo_gif.webm](https://github.com/rydb/bevy_serialization_extras/assets/43288084/3bda45f1-c75a-437b-a02d-27e58bd3276e)




### Features

Out of the box serialization Through [`plugins`] for components
```Rust
    // From<Component> -> Wrapper + From<Wrapper> -> Component
    .add_plugins(SerializeComponentFor::<AsyncCollider, ColliderFlag>::default())

    // From<Asset> -> Wrapper + From<Wrapper> -> Asset
    .add_plugins(SerializeAssetFor::<StandardMaterial, MaterialFlag>::default())
    
    // From<Wrapper> -> Asset
    .add_plugins(DeserializeAssetFrom::<GeometryFlag, Mesh>::default())
```

- type registration for wrappers through `ManagedTypeRegistration` trait
- A visualization gui for types registerd to save, but that are not added to the type registry


