use bevy::prelude::*;
use coclike::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(OnEnter(GameState::MainMenu), setup_menu)
        .add_systems(OnEnter(GameState::Playing), setup_game)
        .add_systems(
            Update,
            (
                menu_interactions.run_if(in_state(GameState::MainMenu)),
                game_systems.run_if(in_state(GameState::Playing)),
            ),
        )
        .run();
}
