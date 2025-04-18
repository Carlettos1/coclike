use crate::prelude::*;
use bevy::prelude::*;

// Event to handle button interactions
#[derive(Event)]
pub enum ButtonInteractionEvent<T: Component + Copy> {
    Pressed(T),
    Hovered(T),
    None,
}

#[allow(type_alias_bounds)]
type QueryTuple<'a, T: Component + Copy> = (
    &'a Interaction,
    &'a T,
    &'a ColorPalette,
    &'a mut BackgroundColor,
);
type Filters = (Changed<Interaction>, With<Button>);
pub fn handle_button_interactions<T: Component + Copy>(
    mut button_query: Query<QueryTuple<T>, Filters>,
    mut interaction_events: EventWriter<ButtonInteractionEvent<T>>,
) {
    for (interaction, button_type, palette, mut bg_color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                interaction_events.send(ButtonInteractionEvent::Pressed(*button_type))
            }
            Interaction::Hovered => {
                interaction_events.send(ButtonInteractionEvent::Hovered(*button_type))
            }
            Interaction::None => interaction_events.send(ButtonInteractionEvent::None),
        };
        palette.do_interaction(interaction, &mut bg_color);
    }
}
