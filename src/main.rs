use bevy::prelude::*;
use bevy::window::{Window, PrimaryWindow};
use rand::prelude::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;
pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 500.0;
pub const ENEMY_SIZE: f32 = 64.0;

fn main(){
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, spawn_player)
    .add_systems(Startup, spawn_camera)
    .add_systems(Update, player_movement)
    .add_systems(Update, confine_player_movement)
    .add_systems(Startup, spawn_enemy)
    .add_systems(Update, enemy_movement)
    .add_systems(Update, update_enemy_direction)
    .add_systems(Update, confine_enemy_movement)
    .run();
}

#[derive(Component)]
pub struct Player;

pub fn spawn_player(
    mut commands: Commands,
    window_qiery: Query<&Window, With<PrimaryWindow>>,
    player: Res<AssetServer>
) {
    let window = window_qiery.get_single().unwrap();

    commands.spawn(SpriteBundle {
        texture: player.load("sprait/ball_blue_large_alt.png"),
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..Default::default()
    }).insert(Player);
}


pub fn spawn_camera(mut commands: Commands, window_qiery: Query<&Window, With<PrimaryWindow>>) {
    let window = window_qiery.get_single().unwrap();
    commands.spawn(Camera2dBundle{
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..Default::default()
    });
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>
){
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut directio = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyD) {
            directio += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            directio += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::KeyW) {
            directio += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            directio += Vec3::new(0.0, -1.0, 0.0);
        }

        if directio.length() > 0.0 {
            directio = directio.normalize();
        }

        transform.translation += directio * PLAYER_SPEED * time.delta_seconds();
    }

}


pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_qiery: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_qiery.get_single().unwrap();
        let half_player_size = PLAYER_SIZE / 2.0;
        let x_min = 0.0 - half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 - half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }
        
        player_transform.translation = translation;
    }
}


#[derive(Component)]
pub struct Enemy{
    pub direction: Vec2
}

pub fn spawn_enemy(
    mut commands: Commands,
    window_qiery: Query<&Window, With<PrimaryWindow>>,
    enemy: Res<AssetServer>,
) {
    let window = window_qiery.get_single().unwrap();
    

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();
        commands.spawn(SpriteBundle {
            texture: enemy.load("sprait/ball_red_large_alt.png"),
            transform: Transform::from_xyz(random_x, random_y, 0.0),
            ..Default::default()
        }).insert(Enemy{direction: Vec2::new(random::<f32>(), random::<f32>()).normalize()});
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }

}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_qiery: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_qiery.get_single().unwrap();
    let half_player_size = ENEMY_SIZE / 2.0;
    let x_min = 0.0 - half_player_size;
    let x_max = window.width() - half_player_size;
    let y_min = 0.0 - half_player_size;
    let y_max = window.height() - half_player_size;
    for (transform, mut enemy) in enemy_query.iter_mut() {
        let translation = transform.translation;
        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0;
        }
        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.0;
        }
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform,  With<Enemy>>,
    window_qiery: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_qiery.get_single().unwrap();
    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = 0.0 - half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 - half_enemy_size;
    let y_max = window.height() - half_enemy_size;
    for mut transform in enemy_query.iter_mut() {
        let mut translation = transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
            

        
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation    
    }
}