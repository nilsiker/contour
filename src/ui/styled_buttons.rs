use bevy::prelude::*;

const COLOR_BUTTON_NONE: Color = Color::rgb(0.15, 0.15, 0.15);
const COLOR_BUTTON_HOVERED: Color = Color::rgb(0.25, 0.25, 0.25);
const COLOR_BUTTON_CLICKED: Color = Color::rgb(0.55, 0.55, 0.55);

pub struct InteractionStyledButtonsPlugin;
impl Plugin for InteractionStyledButtonsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(button_colors_on_interaction);
    }
}

fn button_colors_on_interaction(
    mut query: Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in &mut query {
        match *interaction {
            Interaction::Clicked => *color = COLOR_BUTTON_CLICKED.into(),
            Interaction::Hovered => *color = COLOR_BUTTON_HOVERED.into(),
            Interaction::None => *color = COLOR_BUTTON_NONE.into(),
        }
    }
}
