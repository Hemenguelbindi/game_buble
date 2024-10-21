use bevy::prelude::*;
use bevy::window::{Window, PrimaryWindow};

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;

fn main(){
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, spawn_player)
    .add_systems(Startup, spawn_camera)
    .add_systems(Update, player_movement)
    .add_systems(Update, confine_player_movement)
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
    let texture_handle = player.load("sprait/ball_blue_large_alt.png");
    commands.spawn(SpriteBundle {
        texture: texture_handle,
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
    window_qiery: Query<&Window, With<PrimaryWindow>>
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
