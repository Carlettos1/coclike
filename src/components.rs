use bevy::prelude::*;

// Button marker component
#[derive(Component)]
pub enum MenuButton {
    Play,
    Editor,
    Quit,
}
