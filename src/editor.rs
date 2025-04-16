use crate::prelude::*;
use bevy::prelude::*;

pub fn cleanup_editor(mut commands: Commands, query: Query<Entity, With<BuildUI>>) {
    info!(
        "Doing menu cleanup of: {} items",
        query.iter().remaining().count()
    );

    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn setup_editor(mut commands: Commands) {
    // Setup editor UI
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BuildUI,
        ))
        .with_children(|parent| {
            // Top panel for building selection
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(80.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::linear_rgba(0.1, 0.1, 0.1, 0.8)),
                ))
                .with_children(|parent| {
                    spawn_building_button(
                        parent,
                        "Town Hall",
                        EditorButton::Building(BuildingType::TownHall),
                    );
                    spawn_building_button(
                        parent,
                        "Gold Collector",
                        EditorButton::Building(BuildingType::Collector(ResourceType::Gold)),
                    );
                    spawn_building_button(
                        parent,
                        "Elixir Collector",
                        EditorButton::Building(BuildingType::Collector(ResourceType::Elixir)),
                    );
                    spawn_building_button(
                        parent,
                        "Gold Storage",
                        EditorButton::Building(BuildingType::Storage(ResourceType::Gold)),
                    );
                    spawn_building_button(
                        parent,
                        "Elixir Storage",
                        EditorButton::Building(BuildingType::Storage(ResourceType::Elixir)),
                    );
                    spawn_building_button(
                        parent,
                        "Defense",
                        EditorButton::Building(BuildingType::Defense),
                    );
                    spawn_building_button(
                        parent,
                        "Wall",
                        EditorButton::Building(BuildingType::Wall),
                    );
                });

            // Back button (bottom left)
            parent
                .spawn((
                    Node {
                        width: Val::Px(120.0),
                        height: Val::Px(50.0),
                        position_type: PositionType::Absolute,
                        bottom: Val::Px(20.0),
                        left: Val::Px(20.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    Button,
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.4)),
                    EditorButton::Back,
                ))
                .with_children(|parent| {
                    parent.spawn(Text("Back".into()));
                });
        });

    // Initialize editor state
    commands.insert_resource(EditorState::default());
}

fn spawn_building_button(parent: &mut ChildBuilder, name: &str, button_type: EditorButton) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(120.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            button_type,
        ))
        .with_children(|parent| {
            parent.spawn(Text(name.into()));
        });
}

pub fn editor_interactions(
    mut next_state: ResMut<NextState<GameState>>,
    mut editor_state: ResMut<EditorState>,
    mut button_query: Query<
        (&Interaction, &EditorButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, button, mut bg_color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                match button {
                    EditorButton::Back => {
                        next_state.set(GameState::Playing);
                    }
                    EditorButton::Building(building) => {
                        editor_state.selected_building = Some(*building);
                    }
                }
                *bg_color = Color::srgb(0.35, 0.35, 0.35).into();
            }
            Interaction::Hovered => {
                *bg_color = Color::srgb(0.25, 0.25, 0.25).into();
            }
            Interaction::None => {
                *bg_color = Color::srgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}

pub fn place_editor_building(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut map_query: Query<&mut TileMap>,
    editor_state: Res<EditorState>,
    assets: Res<BuildingAssets>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(building_type) = &editor_state.selected_building {
            // Get cursor position in world
            let (camera, camera_transform) = camera_q.single();
            let window = windows.single();

            if let Some(cursor_pos) = window.cursor_position() {
                if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                    // Convert to grid coordinates
                    let grid_x = world_pos.x.floor() as usize;
                    let grid_y = world_pos.y.floor() as usize;

                    // Place building at grid position
                    let _entity = place_building(
                        &mut commands,
                        &assets,
                        *building_type,
                        1, // Default to level 1
                        grid_x,
                        grid_y,
                    );
                    info!("Placed {:?} at ({}, {})", building_type, grid_x, grid_y);

                    if let Ok(mut _map) = map_query.get_single_mut() {
                        // TODO: info! map.can_place(x, y, size)
                    }
                }
            }
        }
    }
}
