use bevy::prelude::*;
use coclike::prelude::*;

fn main() {
    bevy::log::prelude::debug_once!("a");
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(GameState::MainMenu), setup_menu)
        .add_systems(OnExit(GameState::MainMenu), cleanup_menu)
        .add_systems(OnEnter(GameState::LevelEditor), setup_editor)
        .add_systems(OnExit(GameState::LevelEditor), cleanup_editor)
        .add_systems(
            OnTransition {
                exited: GameState::MainMenu,
                entered: GameState::Playing,
            },
            (
                setup_game,
                setup_grid,
                spawn_initial_buildings,
                setup_hud,
                setup_lower_ui,
                setup_debug_overlay,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                menu_interactions,
                (
                    game_systems,
                    collect_resources,
                    synchronize_buildings_with_map,
                    camera_movement,
                    camera_zoom,
                    update_resource_display,
                    update_debug_overlay,
                    toggle_debug_overlay,
                )
                    .run_if(in_state(GameState::Playing)),
                (
                    editor_interactions,
                    place_editor_building,
                    camera_movement,
                    camera_zoom,
                )
                    .run_if(in_state(GameState::LevelEditor)),
            ),
        )
        .run();
}
