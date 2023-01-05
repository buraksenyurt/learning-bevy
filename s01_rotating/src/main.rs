use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::time::FixedTimestep;
use bevy::window::close_on_esc;
use std::f32::consts::PI;

const BOUNDS: Vec2 = Vec2::new(800., 600.);
const FPS: f32 = 1. / 60.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(FPS as f64))
                .with_system(player_movement_system)
                .with_system(snap_to_player_system)
                .with_system(rotate_to_player_system),
        )
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
    let enemy_a_texture = asset_server.load("enemy_A.png");
    let ship_texture = asset_server.load("ship_C.png");
    let enemy_b_texture = asset_server.load("enemy_B.png");

    let (h_margin, v_margin) = (BOUNDS.x / 4., BOUNDS.y / 4.);

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
            rot: 2. * PI,
        },
    ));

    // enemy nesneleri ekleniyor.
    // a ve b çizimlerinden birer tane eklenmekte
    // transform değerleri ile konumlar belirtiliyor
    commands.spawn((
        SpriteBundle {
            texture: enemy_a_texture.clone(),
            transform: Transform::from_xyz(0. - h_margin, 0., 0.),
            ..default()
        },
        SnapToPlayer,
    ));
    commands.spawn((
        SpriteBundle {
            texture: enemy_a_texture,
            transform: Transform::from_xyz(0., 0. - v_margin, 0.),
            ..default()
        },
        SnapToPlayer,
    ));

    // RotateToPlayer'a verilen radyan değerleri ile,
    // kaç derecelik bir dönüş uygulanacağını belirtiyoruz
    // Buna göre yüzü oyuncuya dönük olacak şekilde düşman gemileri eklenmiş oluyor
    commands.spawn((
        SpriteBundle {
            texture: enemy_b_texture.clone(),
            transform: Transform::from_xyz(0. + h_margin, 0., 0.),
            ..default()
        },
        RotateToPlayer { vel: PI / 4. },
    ));
    commands.spawn((
        SpriteBundle {
            texture: enemy_b_texture,
            transform: Transform::from_xyz(0., 0. + v_margin, 0.),
            ..default()
        },
        RotateToPlayer { vel: PI / 2. },
    ));
}

// Oyuncu için hareket sistemi
// Resource ile klavye hareketlerini içeriye alıyoruz
// query ile de Transform bileşeni barındıran oyuncu nesnesini sorgulayacağımızı belirtiyoruz
fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    // oyuncu nesnesini transform bileşeni ile birlikte yakaladık
    let (p, mut transform) = query.single_mut();

    let mut rot_factor = 0.;
    let mut mov_factor = 0.;

    // Sağ(D) ve sol(A) tuşlara basılması ile döndürme değerini bir birim artırıyoruz
    if keyboard_input.pressed(KeyCode::A) {
        rot_factor += 1.;
    }
    if keyboard_input.pressed(KeyCode::D) {
        rot_factor -= 1.;
    }
    // Yukarı(W) tuşuna basıldığında ise hareket etme faktörünü bir birim artırıyoruz
    if keyboard_input.pressed(KeyCode::W) {
        mov_factor += 1.;
    }
    // z eksenine göre döndürme faktörü, oyuncunun hızı ve delta time değerinleri kullanarak
    // oyuncu gemisine bir döndürme kazandırıyoruz
    transform.rotate_z(rot_factor * p.rot * FPS);

    //
    let movement_direction = transform.rotation * Vec3::Y;
    let movement_distance = mov_factor * p.vel * FPS;
    let translation_delta = movement_direction * movement_distance;
    transform.translation += translation_delta;

    // oyuncu gemisinin oyun sahasının dışına çıkmasını engellemek için
    // bir aralık tespiti gerekiyor.
    let range = Vec3::from((BOUNDS / 2.0, 0.0));
    // oyuncunun pozisyonu min ve max değerlerine göre sınırlandırılıyor.
    transform.translation = transform.translation.min(range).max(-range);
}

// düşman gemilerinin oyuncuyu takip etmesine yarayan sistem
// query ile SnapToPlayer bileşeni içeren entity'leri ele almaktayız. Bu sorguda Player hariç tutuluyor.
// player_query'de ise sadece Player entity'si ele alınmakta.
// Her iki parametre de oyun sahasında Transform bileşenine sahip olma kritierni aramakta tabii.
fn snap_to_player_system(
    mut query: Query<&mut Transform, (With<SnapToPlayer>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    let player_transform = player_query.single();
    // Oyuncunun x,y koordinatlarını alıyoruz
    let player_translation = player_transform.translation.xy();

    // Transform bileşeni olup SnapToPlayer davranışı içeren tüm düşman nesneleri dolaşılıyor
    for mut e_transform in &mut query {
        // Tipik olarak düşman gemisi ile oyuncu konumu arasındaki vektör bulunum normalizasyon
        // işlemine tabii tutuluyor.
        let to_player = (player_translation - e_transform.translation.xy()).normalize();
        // Yukarıdaki birim vektörden yararlanılarak döndürme açısının hesaplanması sağlanıyor
        let rotate_to_player = Quat::from_rotation_arc(Vec3::Y, to_player.extend(0.));
        // ve düşman gemisini döndürülmesi sağlanıyor.
        e_transform.rotation = rotate_to_player;
    }
}

// düşman gemilerinden RotateToPlayer davranışını içerenlerin
// yüzlerini sürekli olarak oyuncunun olduğu tarafa dönmesini sağlayan sistem.
// ilk sorgu ile RotateToPlayer ve Transform davranışlarına sahip oyun nesnelerini oyuncu haricinde
// parametre olarak geçmekteyiz.
// ikinci sorgu ise sadece Transform bileşenini barındıran oyuncu nesnesine odaklanıyor
fn rotate_to_player_system(
    mut query: Query<(&RotateToPlayer, &mut Transform), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
) {
    // Oyuncu gemisinin güncel x,y bilgilerini(yani konumunu alıyoruz)
    let player_transform = player_query.single();
    let player_translation = player_transform.translation.xy();

    // Şimdi sahada RotateToPlayer bileşeni içeren tüm nesneleri geziyoruz
    for (rot_to_plyr, mut enemy_transform) in &mut query {
        // Aşağıda nokta çarpım(Dot Product) kullanılarak,
        let enemy_forward = (enemy_transform.rotation * Vec3::Y).xy();
        let to_player = (player_translation - enemy_transform.translation.xy()).normalize();
        let forward_dot_player = enemy_forward.dot(to_player);

        // elde edilen skaler değerin pozitif olup olmadığına bakıyoruz.
        // pozitif ise aynı yöne baktıklarına kanaat getirilip döngünün sonraki
        // iterasyonuna atlayıp devam eden hesaplamaların boş yere yapılmasını engelliyoruz.
        if (forward_dot_player - 1.0).abs() < f32::EPSILON {
            continue;
        }
        let enemy_right = (enemy_transform.rotation * Vec3::X).xy();
        let right_dot_player = enemy_right.dot(to_player);
        let rotation_sign = -f32::copysign(1.0, right_dot_player);
        let max_angle = forward_dot_player.clamp(-1.0, 1.0).acos();
        let rotation_angle = rotation_sign * (rot_to_plyr.vel * FPS).min(max_angle);
        enemy_transform.rotate_z(rotation_angle);
    }
}
