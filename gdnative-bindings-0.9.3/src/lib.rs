#![allow(non_snake_case)] // because of the generated bindings.
#![allow(unused_unsafe)]
// False positives on generated drops that enforce lifetime
#![allow(clippy::drop_copy)]
// Disable non-critical lints for generated code.
#![allow(clippy::style, clippy::complexity, clippy::perf)]

mod generated;
pub use generated::*;

pub mod utils;

pub(crate) mod icalls;

// Replicate gdnative prelude
pub mod prelude {
    pub use crate::{
        Button, CanvasItem, CanvasLayer, ColorRect, Control, Image, Input, InputEvent, InputEventKey,
        KinematicBody, KinematicBody2D, Label, Node, Node2D, Object, PackedScene, Reference,
        ResourceLoader, SceneTree, Shader, Spatial, Sprite, Texture, Timer, Tween, Viewport,
    };

    pub use crate::utils::*;
}