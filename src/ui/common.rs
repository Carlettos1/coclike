//! Here are common Components, Markers, Resources and generator functions to UI/HUD elements
//! :D

use crate::prelude::*;
use bevy::prelude::*;

pub mod components;

pub use self::components::*;

pub fn whole_screen_center() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Column,
        ..default()
    }
}

pub fn whole_screen() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        ..default()
    }
}

pub fn title_text(text: &str) -> impl Bundle {
    (
        Text(text.into()),
        TextFont {
            font_size: 80.0,
            ..default()
        },
    )
}

pub fn big_text(text: &str) -> impl Bundle {
    (
        Text(text.into()),
        TextFont {
            font_size: 40.0,
            ..default()
        },
    )
}

pub fn text(text: &str) -> impl Bundle {
    (Text(text.into()),)
}

pub fn standard_button() -> impl Bundle {
    (
        Button,
        Node {
            width: BUTTON_WIDTH,
            height: BUTTON_HEIGHT,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ColorPalette::new_with_bg(GRAY_35, GRAY_25, GRAY_15),
    )
}

pub fn create_panel(width: Val, height: Val) -> impl Bundle {
    (
        Node {
            width,
            height,
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        BackgroundColor(PANEL_BG_COLOR),
    )
}
