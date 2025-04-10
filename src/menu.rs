use crate::prelude::*;
use bevy::prelude::*;

pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        })
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
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
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

// Menu interaction system
pub fn menu_interactions(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match button {
                MenuButton::Play => {
                    next_state.set(GameState::Playing);
                    println!("Play button pressed");
                }
                MenuButton::Editor => next_state.set(GameState::LevelEditor),
                MenuButton::Quit => std::process::exit(0),
            }
        }
    }
}
