use crate::prelude::*;
use bevy::prelude::*;

pub fn setup_debug_overlay(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(30.0),
                height: Val::Percent(70.0),
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                top: Val::Px(10.0),
                padding: UiRect::all(Val::Px(10.0)),
                display: Display::None,
                ..default()
            },
            BackgroundColor(Color::linear_rgba(0.0, 0.0, 0.0, 0.8)),
            DebugOverlayMarker,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text("DEBUG INFORMATION".into()),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
            ));

            parent.spawn((Text("".into()), DebugOverlayMarker));
        });
}

pub fn update_debug_overlay(
    mut query: Query<&mut Text, With<DebugOverlayMarker>>,
    camera_query: Query<(&Transform, &GameCamera), With<Camera2d>>,
    resources: Res<PlayerResources>,
    map_query: Query<&TileMap>,
    building_query: Query<&Building>,
) {
    // Skip the title text by getting the second text element
    let mut debug_text = query.single_mut();
    let (camera_transform, camera) = camera_query.single();
    let map = map_query.single();
    let building_count = building_query.iter().count();

    let mut content = format!(
        "Camera Pos: ({:.1}, {:.1})\n",
        camera_transform.translation.x, camera_transform.translation.y
    );
    content.push_str(&format!("Camera Zoom: {:.2}\n", camera.zoom));
    content.push_str(&format!("Map Size: {}x{}\n", map.width, map.height));
    content.push_str(&format!("Buildings: {}\n", building_count));
    content.push_str("\nResources:");
    for (resource_type, amount) in &resources.resources {
        content.push_str(&format!("\n  {:?}: {:.1}", resource_type, amount));
    }

    debug_text.0 = content;
}

pub fn toggle_debug_overlay(
    mut debug_state: ResMut<DebugState>,
    mut query: Query<&mut Node, With<DebugOverlayMarker>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::F3) {
        debug_state.visible = !debug_state.visible;

        for mut style in query.iter_mut() {
            style.display = if debug_state.visible {
                Display::Block
            } else {
                Display::None
            };
        }
    }
}
