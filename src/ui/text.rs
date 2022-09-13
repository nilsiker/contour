use bevy_egui::egui::RichText;

use super::styling::{TEXT_PRIMARY, TEXT_SECONDARY};

pub fn p(text: &str) -> RichText {
    RichText::new(text).color(TEXT_PRIMARY)
}

pub fn em(text: &str) -> RichText {
    RichText::new(text).color(TEXT_SECONDARY)
}

pub fn h1(text: &str) -> RichText {
    RichText::new(text).color(TEXT_SECONDARY).size(40.0)
}

pub fn h2(text: &str) -> RichText {
    RichText::new(text).color(TEXT_PRIMARY).size(20.0)
}
