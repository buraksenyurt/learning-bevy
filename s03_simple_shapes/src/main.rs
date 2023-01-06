use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system)
        .add_system(square_scale_system)
        .run();
}

#[derive(Component)]
struct Square;

fn setup_system(
    mut commands: Commands,
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
        Square,
    ));
}

fn square_scale_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Square, &mut Transform)>,
) {
    for (_, mut tr) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Up) {
            tr.scale += 0.05;
        } else if keyboard_input.pressed(KeyCode::Down) {
            tr.scale -= 0.05;
        }
    }
}
