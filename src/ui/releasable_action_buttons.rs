use bevy::prelude::*;

pub struct ReleaseActionButtonPlugin;
impl Plugin for ReleaseActionButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ButtonReleasedEvent>()
            .add_event::<Action>()
            .add_system(fire_button_release_events.chain(update_last_interaction_trackers))
            .add_system(event_handlers::game_start)
            .add_system(event_handlers::game_quit)
            .add_system(event_handlers::raise_bgm_volume)
            .add_system(event_handlers::lower_bgm_volume)
            .add_system(event_handlers::raise_sfx_volume)
            .add_system(event_handlers::lower_sfx_volume)
            .add_system(event_handlers::options_open)
            .add_system(event_handlers::options_close);
    }
}

#[derive(Component)]
pub struct LastInteractionTracker(pub Interaction);

#[derive(Component)]
pub struct ActionButton(pub Action);

struct ButtonReleasedEvent(pub Action);

#[derive(Clone, Debug)]
pub enum Action {
    GameStart,
    GameQuit,
    OptionsOpen,
    OptionsClose,
    RaiseVolumeBGM,
    LowerVolumeBGM,
    RaiseVolumeSFX,
    LowerVolumeSFX,
}

fn update_last_interaction_trackers(
    mut query: Query<(&Interaction, &mut LastInteractionTracker), Changed<Interaction>>,
) {
    for (interaction, mut last_interaction) in &mut query {
        last_interaction.0 = *interaction;
    }
}

fn fire_button_release_events(
    query: Query<(&ActionButton, &LastInteractionTracker, &Interaction), Changed<Interaction>>,
    mut button_actions: EventWriter<Action>,
) {
    for (button, last_interaction, interaction) in &query {
        if let Interaction::Clicked = last_interaction.0 {
            if let Interaction::Hovered = *interaction {
                button_actions.send(button.0.clone());
            }
        }
    }
}

mod event_handlers {
    use bevy::prelude::*;

    use crate::{audio::AudioChannels, game::GameState, ui::options_menu::OptionsMenu};

    use super::Action;

    pub fn game_start(mut actions: EventReader<Action>, mut state: ResMut<State<GameState>>) {
        for _ in actions.iter().filter(|e| matches!(e, Action::GameStart)) {
            match state.set(GameState::InGame) {
                Ok(_) => bevy::log::info!("state changed to {:?}", *state),
                Err(e) => bevy::log::error!("{e}"),
            }
        }
    }

    pub fn game_quit(mut actions: EventReader<Action>, mut quit: EventWriter<bevy::app::AppExit>) {
        for _ in actions.iter().filter(|e| matches!(e, Action::GameQuit)) {
            quit.send(bevy::app::AppExit);
        }
    }

    pub fn options_open(
        mut actions: EventReader<Action>,
        mut options_menu: Query<&mut Style, With<OptionsMenu>>,
    ) {
        for _ in actions.iter().filter(|e| matches!(e, Action::OptionsOpen)) {
            options_menu.single_mut().display = Display::Flex;
        }
    }

    pub fn options_close(
        mut actions: EventReader<Action>,
        mut options_menu: Query<&mut Style, With<OptionsMenu>>,
    ) {
        for _ in actions.iter().filter(|e| matches!(e, Action::OptionsClose)) {
            options_menu.single_mut().display = Display::None;
        }
    }

    pub fn raise_bgm_volume(mut actions: EventReader<Action>, mut volume: ResMut<AudioChannels>) {
        for _ in actions
            .iter()
            .filter(|e| matches!(e, Action::RaiseVolumeBGM))
        {
            volume.bgm.0 += 1.0;
        }
    }

    pub fn lower_bgm_volume(mut actions: EventReader<Action>, mut volume: ResMut<AudioChannels>) {
        for _ in actions
            .iter()
            .filter(|e| matches!(e, Action::LowerVolumeBGM))
        {
            volume.bgm.0 -= 1.0;
        }
    }
    pub fn raise_sfx_volume(mut actions: EventReader<Action>, mut volume: ResMut<AudioChannels>) {
        for _ in actions
            .iter()
            .filter(|e| matches!(e, Action::RaiseVolumeSFX))
        {
            volume.sfx.0 += 1.0;
        }
    }
    pub fn lower_sfx_volume(mut actions: EventReader<Action>, mut volume: ResMut<AudioChannels>) {
        for _ in actions
            .iter()
            .filter(|e| matches!(e, Action::LowerVolumeSFX))
        {
            volume.sfx.0 -= 1.0;
        }
    }
}
