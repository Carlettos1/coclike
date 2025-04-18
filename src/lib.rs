use bevy::prelude::*;

mod buildings;
mod camera;
mod components;
mod constants;
mod game;
mod ui;

pub mod prelude {
    pub use crate::buildings::*;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::constants::*;
    pub use crate::game::*;
    pub use crate::ui::*;
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
