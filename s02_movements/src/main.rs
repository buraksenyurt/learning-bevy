use bevy::app::App;
use bevy::prelude::Component;
use bevy::DefaultPlugins;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}

#[derive(Component)]
struct Fighter {
    movement_speed: f32,
    rotation_speed: f32,
}
