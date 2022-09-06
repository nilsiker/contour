use bevy::prelude::*;

pub struct ReleaseActionButtonPlugin;
impl Plugin for ReleaseActionButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ButtonReleasedEvent>();
        app.add_system(
            fire_button_release_events
                .chain(update_last_interaction_trackers.chain(release_buttons)),
        );
    }
}

#[derive(Component)]
pub struct LastInteractionTracker(pub Interaction);

#[derive(Component, Copy, Clone)]
pub struct ActionButton(pub Action);

#[derive(Component, Copy, Clone)]
pub enum Action {
    QuitApp,
}

struct ButtonReleasedEvent(Action);

fn update_last_interaction_trackers(
    mut query: Query<(&Interaction, &mut LastInteractionTracker), Changed<Interaction>>,
) {
    for (interaction, mut last_interaction) in &mut query {
        last_interaction.0 = *interaction;
    }
}

fn fire_button_release_events(
    mut ev_released_button: EventWriter<ButtonReleasedEvent>,
    query: Query<(&ActionButton, &LastInteractionTracker, &Interaction), Changed<Interaction>>,
) {
    for (button, last_interaction, interaction) in &query {
        if let Interaction::Clicked = last_interaction.0 {
            if let Interaction::Hovered = *interaction {
                ev_released_button.send(ButtonReleasedEvent(button.0));
            }
        }
    }
}

fn release_buttons(
    mut released_button_events: EventReader<ButtonReleasedEvent>,
    mut app_exit_events: EventWriter<bevy::app::AppExit>,
) {
    for released_button in released_button_events.iter() {
        match released_button.0 {
            Action::QuitApp => app_exit_events.send(bevy::app::AppExit),
        }
    }
}
