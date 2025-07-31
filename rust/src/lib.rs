use godot::prelude::*;

pub mod State_Machine;
pub mod SpriteAnimationLoader;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
