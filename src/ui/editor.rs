use crate::prelude::*;
use bevy::prelude::*;

pub fn cleanup_editor(mut commands: Commands, query: Query<Entity, With<BuildUIMarker>>) {
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
            BuildUIMarker,
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
                    ColorPalette::new_with_bg(BLUE_6, BLUE_4, BLUE_2),
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
        .spawn((standard_button(), button_type))
        .with_children(|parent| {
            parent.spawn(Text(name.into()));
        });
}

/// Triggered bv [`handle_button_interactions`]
pub fn editor_button_handler(
    mut events: EventReader<ButtonInteractionEvent<EditorButton>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut editor_state: ResMut<EditorState>,
) {
    for event in events.read() {
        match event {
            ButtonInteractionEvent::Pressed(button) => match button {
                EditorButton::Back => {
                    next_state.set(GameState::Playing);
                }
                EditorButton::Building(building) => {
                    editor_state.selected_building = Some(*building);
                    editor_state.is_selected = true;
                }
            },
            ButtonInteractionEvent::None => {
                editor_state.is_selected = false;
            }
            _ => (),
        }
    }
}

pub fn place_editor_building(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut map_query: Query<&mut TileMap>,
    mut editor_state: ResMut<EditorState>,
    assets: Res<BuildingAssets>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if editor_state.is_selected {
            return;
        }
        if let Some(building_type) = &editor_state.selected_building {
            // Get cursor position in world
            let (camera, camera_transform) = camera_q.single();
            let window = windows.single();

            if let Some(cursor_pos) = window.cursor_position() {
                if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                    // Convert to grid coordinates
                    let grid_x = world_pos.x.floor() as usize;
                    let grid_y = world_pos.y.floor() as usize;

                    if let Ok(mut map) = map_query.get_single_mut() {
                        if map.can_place(grid_x, grid_y, to_size(*building_type)) {
                            place_building(
                                &mut commands,
                                &assets,
                                *building_type,
                                1,
                                grid_x,
                                grid_y,
                            );
                            info!("Placed {:?} at ({}, {})", building_type, grid_x, grid_y);
                            editor_state.selected_building = None;
                        }
                    }
                }
            }
        }
    }
}
