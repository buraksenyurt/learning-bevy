use bevy::prelude::*;
use bevy::time::FixedTimestep;
use bevy::window::close_on_esc;

const BOUNDS: Vec2 = Vec2::new(800., 600.);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system)
        .add_system_set(SystemSet::new().with_run_criteria(FixedTimestep::step(1. / 60.)))
        .add_system(close_on_esc)
        .run();
}

// Oyuncu bileşeni
#[derive(Component)]
struct Player {
    vel: f32, // sabit hız değeri
    rot: f32, // radyan cinsiden dönüş hızı
}

// Davranışsal bileşen
#[derive(Component)]
struct SnapToPlayer;

// Davranışsal bileşen
#[derive(Component)]
struct RotateToPlayer {
    vel: f32, // dönüş hızı
}

// kurulum işlemlerini yapan sistem fonksiyonu
fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // oyunda kullanılan texture'lar asset_server ile yüklenir
    //let enemy_a_texture = asset_server.load("enemy_A.png");
    let ship_texture = asset_server.load("ship_C.png");
    //let enemy_b_texture = asset_server.load("enemy_B.png");

    // varsayılan ayarları ile 2D ortografik kamera hazırlanır
    commands.spawn(Camera2dBundle::default());

    // oyuncunun kontrol ettiği gemi oluşturulur
    // İlk parametre ile bir Sprite ekleniyor.
    // İkinci parametre de Player bileşeni ile ilişkilendirmemizi sağlıyor
    commands.spawn((
        SpriteBundle {
            texture: ship_texture,
            ..default()
        },
        Player {
            vel: 500.,
            rot: 360_f32.to_radians(),
        },
    ));
}
