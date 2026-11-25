use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

mod board;
mod board_actions;
mod board_cell;
mod board_events;
mod game;
mod neutron;
