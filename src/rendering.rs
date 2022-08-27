use bevy::{prelude::*, render::texture::ImageSettings, window::PresentMode};

pub struct RenderingPlugin;
impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ImageSettings::default_nearest())
            .insert_resource(WindowDescriptor {
                title: "Bevy Gamejam 2022".to_string(),
                position: WindowPosition::At(Vec2::new(1720., 0.)),
                width: 640.,
                height: 360.,
                present_mode: PresentMode::AutoNoVsync,
                transparent: true,
                resizable: true,
                cursor_visible: true,

                ..default()
            })
            .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
            .add_system(order_z_entities);
    }
}

fn order_z_entities(mut query: Query<&mut Transform, With<OrderedZ>>) {
    for mut transform in &mut query {
        transform.translation.z = -transform.translation.y;
    }
}

#[derive(Component)]
pub struct OrderedZ;
