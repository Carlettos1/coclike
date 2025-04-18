use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub struct ColorPalette {
    pub pressed: Color,
    pub hovered: Color,
    pub none: Color,
}

impl ColorPalette {
    /// Creates a BackgroundColor with a color palette
    pub fn new_with_bg(pressed: Color, hovered: Color, none: Color) -> impl Bundle {
        (
            ColorPalette {
                pressed,
                hovered,
                none,
            },
            BackgroundColor(none),
        )
    }

    /// Changes BackgroundColor depending on interaction
    pub fn do_interaction(&self, interaction: &Interaction, bg: &mut BackgroundColor) {
        match interaction {
            Interaction::Pressed => *bg = self.pressed.into(),
            Interaction::Hovered => *bg = self.hovered.into(),
            Interaction::None => *bg = self.none.into(),
        }
    }
}

/// Button types
#[derive(Component, Debug, Clone, Copy)]
pub enum MenuButton {
    Play,
    Editor,
    Quit,
}

#[derive(Component, Debug, Clone, Copy)]
pub enum EditorButton {
    Building(BuildingType),
    Back,
}

// Markers

#[derive(Component, Debug, Clone, Copy)]
pub struct MainMenuMarker;

#[derive(Component, Debug, Clone, Copy)]
pub struct GameHUDMarker;

#[derive(Component, Debug, Clone, Copy)]
pub struct ResourceDisplayMarker;

#[derive(Component, Debug, Clone, Copy)]
pub struct DebugOverlayMarker;

#[derive(Component, Debug, Clone, Copy)]
pub struct BuildUIMarker;

// Resources

#[derive(Resource, Default)]
pub struct DebugState {
    pub visible: bool,
}
