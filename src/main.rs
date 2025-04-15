use bevy::prelude::*;
use coclike::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(GameState::MainMenu), setup_menu)
        .add_systems(OnExit(GameState::MainMenu), cleanup_menu)
        .add_systems(
            OnEnter(GameState::Playing),
            (setup_game, setup_grid, spawn_initial_buildings).chain(),
        )
        .add_systems(
            Update,
            (
                menu_interactions.run_if(in_state(GameState::MainMenu)),
                (
                    game_systems,
                    collect_resources,
                    synchronize_buildings_with_map,
                    camera_movement,
                    camera_zoom,
                )
                    .run_if(in_state(GameState::Playing)),
            ),
        )
        .run();
}
