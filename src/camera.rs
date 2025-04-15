use crate::prelude::*;
use bevy::{
    input::{mouse::MouseWheel, touch::TouchPhase},
    prelude::*,
};

pub fn setup_camera(mut commands: Commands) {
    let game_camera = GameCamera::default();
    let mut orthographic_projection = OrthographicProjection::default_2d();
    orthographic_projection.scale = 1.0 / game_camera.zoom;
    commands.spawn((
        Transform::from_xyz(50.0, 45.0, 0.0),
        orthographic_projection,
        Camera2d,
        game_camera,
    ));
}

pub fn camera_movement(
    mut camera_query: Query<(&mut Transform, &mut GameCamera)>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut touch_input: EventReader<TouchInput>,
    windows: Query<&Window>,
    time: Res<Time>,
) {
    let window = windows.single();
    let (mut transform, mut controller) = camera_query.single_mut();

    // Handle mouse input
    if mouse.just_pressed(MouseButton::Left) {
        controller.is_dragging = true;
        controller.last_position = window.cursor_position().map(|pos| Vec2::new(pos.x, pos.y));
    }

    if mouse.just_released(MouseButton::Left) {
        controller.is_dragging = false;
        controller.last_position = None;
    }

    // Continue drag if pressed
    if controller.is_dragging {
        if let Some(current_pos) = window.cursor_position().map(|pos| Vec2::new(pos.x, pos.y)) {
            if let Some(last_pos) = controller.last_position {
                let delta = current_pos - last_pos;
                transform.translation.x -= delta.x * controller.speed * time.delta_secs();
                transform.translation.y += delta.y * controller.speed * time.delta_secs();
            }
            controller.last_position = Some(current_pos);
        }
    }

    // Handle touch input
    let touches = touch_input.read().collect::<Vec<_>>();
    match touches.len() {
        1 => {
            if let Some(touch) = touch_input.read().next() {
                match touch.phase {
                    TouchPhase::Started => {
                        controller.is_dragging = true;
                        controller.last_position = Some(touch.position);
                    }
                    TouchPhase::Ended | TouchPhase::Canceled => {
                        controller.is_dragging = false;
                        controller.last_position = None;
                    }
                    TouchPhase::Moved if controller.is_dragging => {
                        if let Some(last_pos) = controller.last_position {
                            let delta = touch.position - last_pos;
                            transform.translation.x -=
                                delta.x * controller.speed * time.delta_secs();
                            transform.translation.y +=
                                delta.y * controller.speed * time.delta_secs();
                        }
                        controller.last_position = Some(touch.position);
                    }
                    _ => (),
                }
            }
        }
        2 => warn!("two Touches, should zoom"),
        _ => (),
    }
}

pub fn camera_zoom(
    mut scroll_event: EventReader<MouseWheel>,
    mut camera_query: Query<(&mut OrthographicProjection, &mut GameCamera), With<Camera2d>>,
) {
    let scroll = scroll_event.read().map(|e| e.y).sum::<f32>();
    if scroll == 0.0 {
        return;
    }

    for (mut projection, mut camera) in camera_query.iter_mut() {
        camera.zoom += scroll * camera.zoom_speed;
        camera.zoom = camera.zoom.clamp(camera.min_zoom, camera.max_zoom);

        projection.scale = 1.0 / camera.zoom;
    }
}
