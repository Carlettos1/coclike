use crate::prelude::*;
use bevy::prelude::*;

pub fn setup_menu(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands
        .spawn((whole_screen_center(), MainMenuMarker))
        .with_children(|parent| {
            parent.spawn(title_text("Clash-like Game"));
            parent
                .spawn((standard_button(), MenuButton::Play))
                .with_children(|parent| {
                    parent.spawn(big_text("Play"));
                });
        });
}

pub fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MainMenuMarker>>) {
    info!(
        "Doing menu cleanup of: {} items",
        query.iter().remaining().count()
    );

    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

/// Triggered bv [`handle_button_interactions`]
pub fn menu_button_handler(
    mut events: EventReader<ButtonInteractionEvent<MenuButton>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for event in events.read() {
        if let ButtonInteractionEvent::Pressed(button) = event {
            match button {
                MenuButton::Play => next_state.set(GameState::Playing),
                MenuButton::Editor => next_state.set(GameState::LevelEditor),
                MenuButton::Quit => std::process::exit(0),
            }
        }
    }
}
