use bevy::prelude::*;

pub struct WeatherPlugin;
impl Plugin for WeatherPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Rain::Heavy)
        .insert_resource(Wind::Heavy)
        .insert_resource(Thunder::Heavy)
    }
}

pub enum Rain {
    None,
    Light,
    Heavy,
}

pub enum Wind {
    None,
    Light,
    Heavy,
}

pub enum Thunder {
    None,
    Light,
    Heavy,
}
