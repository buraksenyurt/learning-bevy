use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system)
        .add_system(square_scale_system)
        .run();
}

#[derive(Component)]
struct Shape;

fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba_u8(237, 43, 191, 250),
                custom_size: Some(Vec2::new(64., 64.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(-200., 0., 0.),
                ..default()
            },
            ..default()
        },
        Shape,
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::RegularPolygon::new(32., 12).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::rgba_u8(43, 198, 238, 250))),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
        Shape,
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(32.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::rgba_u8(237, 146, 43, 250))),
            transform: Transform::from_translation(Vec3::new(200., 0., 0.)),
            ..default()
        },
        Shape,
    ));
}

fn square_scale_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Shape, &mut Transform)>,
) {
    for (_, mut tr) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Up) {
            tr.scale += 0.05;
        } else if keyboard_input.pressed(KeyCode::Down) {
            tr.scale -= 0.05;
        }
    }
}
