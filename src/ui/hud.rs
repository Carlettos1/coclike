use crate::prelude::*;
use bevy::prelude::*;

pub fn setup_hud(mut commands: Commands) {
    commands
        .spawn((whole_screen(), GameHUDMarker))
        .with_children(|parent| {
            // Resources panel (top right)
            parent
                .spawn((
                    Node {
                        width: Val::Auto,
                        height: Val::Auto,
                        position_type: PositionType::Absolute,
                        top: Val::Px(10.0),
                        right: Val::Px(10.0),
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::linear_rgba(0.1, 0.1, 0.1, 0.7)),
                    ResourceDisplayMarker,
                ))
                .with_children(|parent| {
                    parent.spawn(Text("Resources:".into()));
                    parent.spawn((
                        Text("".into()), // Will be updated by the system
                        ResourceDisplayMarker,
                    ));
                });
        });

    // Insert debug state resource
    commands.insert_resource(DebugState::default());
}

pub fn update_resource_display(
    resources: Res<PlayerResources>,
    mut query: Query<&mut Text, With<ResourceDisplayMarker>>,
) {
    let mut text = query.single_mut();
    let mut content = String::new();
    for (resource_type, amount) in &resources.resources {
        content.push_str(&format!("\n{resource_type:?}: {:.0}", amount.trunc()));
    }
    text.0 = content;
}

pub fn setup_lower_ui(mut commands: Commands) {
    commands
        .spawn((whole_screen(), GameHUDMarker))
        .with_children(|parent| {
            // Build button (bottom right)
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Auto,
                        height: Val::Auto,
                        position_type: PositionType::Absolute,
                        bottom: Val::Px(10.0),
                        right: Val::Px(10.0),
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    ColorPalette::new_with_bg(BROWN_4, BROWN_3, BROWN_2),
                    MenuButton::Editor,
                ))
                .with_children(|parent| {
                    parent.spawn(Text("Editor".into()));
                });
        });
}
