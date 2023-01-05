use bevy::app::App;
use bevy::prelude::*;
use bevy::window::close_on_esc;
use bevy::DefaultPlugins;
use std::f32::consts::PI;

const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system)
        .add_system(close_on_esc)
        .run();
}

#[derive(Component)]
struct Fighter {
    movement_speed: f32,
    rotation_speed: f32,
}

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let fighter_texture = asset_server.load("fighter.png");

    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            texture: fighter_texture,
            transform: Transform::from_xyz(0., 0. - BOUNDS.y / 2., 0.)
                .with_scale(Vec3::new(0.75, 0.75, 0.75)),
            ..default()
        },
        Fighter {
            movement_speed: 500.,
            rotation_speed: 2. * PI,
        },
    ));
}
