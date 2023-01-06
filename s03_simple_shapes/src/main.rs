use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system)
        .run();
}

#[derive(Component)]
struct SomeSquare;

fn setup_system(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba_u8(237, 43, 191, 100),
                custom_size: Some(Vec2::new(64., 64.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(-200., 0., 0.),
                ..default()
            },
            ..default()
        },
        SomeSquare,
    ));
}
