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
                .with_system(fighter_movement_system)
                .with_system(missile_flight_system)
                .with_system(fighter_fire_system),
        )
        .add_system(close_on_esc)
        .run();
}

#[derive(Component)]
struct Fighter {
    movement_speed: f32,
    rotation_speed: f32,
}

#[derive(Component, Copy, Clone)]
struct Missile {
    movement_speed: f32,
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
            movement_speed: 400.,
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

fn fighter_fire_system(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut query: Query<(&Fighter, &mut Transform)>,
) {
    let (_, mut transform) = query.single_mut();

    if keyboard_input.just_pressed(KeyCode::Space) {
        let translation = transform.translation;
        let rotation = transform.rotation;

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("missile.png"),
                transform: Transform {
                    translation,
                    rotation,
                    ..default()
                },
                ..default()
            },
            Missile {
                movement_speed: 600.,
            },
        ));
    }
}

fn missile_flight_system(
    mut commands: Commands,
    mut query: Query<(Entity, &Missile, &mut Transform), With<Missile>>,
) {
    for (missile_entity, &missile, mut transform) in query.iter_mut() {
        let movement_direction = transform.rotation * Vec3::Y;
        let movement_distance = missile.movement_speed * DELTA_TIME;
        let translation_delta = movement_direction * movement_distance;
        transform.translation += translation_delta;

        if transform.translation.x < -1. * (BOUNDS.x / 2.)
            || transform.translation.x > BOUNDS.x
            || transform.translation.y < -1. * (BOUNDS.y / 2.)
            || transform.translation.y > BOUNDS.y
        {
            info!("{:?}", transform.translation);
            commands.entity(missile_entity).despawn();
        }
    }
}
