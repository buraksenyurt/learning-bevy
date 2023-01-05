use bevy::app::App;
use bevy::prelude::*;
use bevy::time::FixedTimestep;
use bevy::window::close_on_esc;
use bevy::DefaultPlugins;
use std::f32::consts::PI;

const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);
const DELTA_TIME: f32 = 1. / 60.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(DELTA_TIME as f64))
                .with_system(fighter_movement_system),
        )
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

fn fighter_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Fighter, &mut Transform)>,
) {
    let (fighter, mut transform) = query.single_mut();
    let mut rotation_factor = 0.;
    let mut movement_factor = 0.;

    if keyboard_input.pressed(KeyCode::Left) {
        rotation_factor += 0.5;
    } else if keyboard_input.pressed(KeyCode::Right) {
        rotation_factor -= 0.5;
    }
    if keyboard_input.pressed(KeyCode::Up) {
        movement_factor += 0.75;
    }
    transform.rotate_z(rotation_factor * fighter.rotation_speed * DELTA_TIME);

    // Vec3::Y [0,1,0] birim vektörüdür.
    // Dönüş açısının birim vektör ile çarpımı sayesinde hareket yönünü buluruz
    let movement_direction = transform.rotation * Vec3::Y;
    // yöne göre büyüklüğü hesaplarız
    let movement_distance = movement_factor * fighter.movement_speed * DELTA_TIME;
    let translation_delta = movement_direction * movement_distance;
    transform.translation += translation_delta;

    let range = Vec3::from((BOUNDS / 2.0, 0.));
    transform.translation = transform.translation.min(range).max(-range);
}
