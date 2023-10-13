# bevy_serialization_extras
A library for bevy which contains wrappers/systems for managing serialization in bevy with moonshine-save for a more "hands off" serialization workflow


[insert example_world gif here]



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

Outline:

- features

- how to use

<!-- contains plugins/systems to make serialization/deserialization with bevy smoother -->


<!-- [![Github All Releases](https://img.shields.io/github/downloads/rydb/ROS2_easy/total.svg)]() -->

- [Features](#description)
- [Installation](#installation)

