use bevy::{app::AppExit, prelude::*};
use bevy_egui::{
    egui::{style::Margin, Color32, Frame, Pos2, Rect, RichText, Rounding, Vec2},
    EguiContext,
};

use super::{
    styling::{MENU_BUTTON_FILL, MENU_FILL, MENU_STROKE},
    options_menu::OptionsUiState,
};

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update);
    }
}

fn update(
    mut egui: ResMut<EguiContext>,
    windows: Res<Windows>,
    mut options_menu_state: ResMut<OptionsUiState>,
    mut exit: EventWriter<AppExit>,
) {
    let window = windows.get_primary().unwrap();
    let save_exists = true; // TODO actually check for save.

    let center_pos: Pos2 = (window.width() / 6.0, window.height() / 2.0).into();
    let size: Vec2 = (window.width() / 6.0, window.height() / 2.0).into();
    let button_size = [window.width() / 9.0, window.height() / 10.0];

    bevy_egui::egui::Window::new("MainMenu")
        .title_bar(false)
        .fixed_rect(Rect::from_center_size(center_pos, size))
        .frame(Frame {
            outer_margin: Margin {
                left: 50.0,
                ..default()
            },
            inner_margin: Margin::same(40.0),
            fill: MENU_FILL,
            rounding: Rounding::same(8.0),
            stroke: MENU_STROKE,
            ..default()
        })
        .show(egui.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                ui.label(RichText::new("CONTOUR").color(Color32::WHITE).size(40.0));
                ui.add_space(30.0);

                if save_exists
                    && ui
                        .add_sized(
                            button_size,
                            bevy_egui::egui::Button::new(RichText::new("Continue").size(20.0))
                                .stroke(MENU_STROKE)
                                .fill(MENU_BUTTON_FILL),
                        )
                        .clicked()
                {}
                ui.add_space(10.0);
                if ui
                    .add_sized(
                        button_size,
                        bevy_egui::egui::Button::new(RichText::new("New").size(20.0))
                            .stroke(MENU_STROKE)
                            .fill(MENU_BUTTON_FILL),
                    )
                    .clicked()
                {}

                ui.add_space(10.0);

                if ui
                    .add_sized(
                        button_size,
                        bevy_egui::egui::Button::new(RichText::new("Options").size(20.0))
                            .stroke(MENU_STROKE)
                            .fill(MENU_BUTTON_FILL),
                    )
                    .clicked()
                {
                    options_menu_state.open = true
                }
                ui.add_space(10.0);

                if ui
                    .add_sized(
                        button_size,
                        bevy_egui::egui::Button::new(RichText::new("Quit").size(20.0))
                            .stroke(MENU_STROKE)
                            .fill(MENU_BUTTON_FILL),
                    )
                    .clicked()
                {
                    exit.send(AppExit);
                }
            });
        });
}
