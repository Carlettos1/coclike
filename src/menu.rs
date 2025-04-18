use crate::prelude::*;
use bevy::prelude::*;

pub fn setup_menu(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            MainMenuUI,
        ))
        .with_children(|parent| {
            // Game title
            parent.spawn((
                Text("Clash-like Game".into()),
                TextFont {
                    font_size: 80.0,
                    ..default()
                },
            ));

            // Play button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(65.0),
                        margin: UiRect::all(Val::Px(20.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(DARK_GRAY),
                    MenuButton::Play,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text("Play".into()),
                        TextFont {
                            font_size: 40.0,
                            ..default()
                        },
                    ));
                });
        });
}

type QueryTuple<'a> = (&'a Interaction, &'a MenuButton, &'a mut BackgroundColor);
type Filters = (Changed<Interaction>, With<Button>);
// Menu interaction system
pub fn menu_interactions(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<QueryTuple, Filters>,
) {
    for (interaction, button, mut bg_color) in &mut interaction_query {
        info!("Main state Interaction: {interaction:?} on button {button:?}");
        match *interaction {
            Interaction::Pressed => match button {
                MenuButton::Play => next_state.set(GameState::Playing),
                MenuButton::Editor => next_state.set(GameState::LevelEditor),
                MenuButton::Quit => std::process::exit(0),
            },
            Interaction::Hovered => match button {
                MenuButton::Editor => *bg_color = BROWN.into(),
                MenuButton::Play => *bg_color = GRAY.into(),
                MenuButton::Quit => *bg_color = GRAY.into(),
            },
            Interaction::None => match button {
                MenuButton::Editor => *bg_color = BROWN_SATURATED.into(),
                MenuButton::Play => *bg_color = DARK_GRAY.into(),
                MenuButton::Quit => *bg_color = DARK_GRAY.into(),
            },
        }
    }
}

pub fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MainMenuUI>>) {
    info!(
        "Doing menu cleanup of: {} items",
        query.iter().remaining().count()
    );

    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
