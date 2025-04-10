use bevy::prelude::*;

mod components;
mod game;
mod menu;

pub mod prelude {
    pub use crate::components::*;
    pub use crate::game::*;
    pub use crate::menu::*;
    pub use crate::setup;
    pub use crate::GameState;
}

// Game states
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    LevelEditor,
    Paused,
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
