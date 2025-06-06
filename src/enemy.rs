use bevy::prelude::*;
use crate::constants::{WINDOW_WIDTH, ENEMY_SIZE, ENEMY_SPEED, WAVE_AMPLITUDE, WAVE_FREQUENCY};
use crate::movement::Velocity;

/// Simple enemy that moves horizontally in a wave pattern.
#[derive(Component)]
pub struct Enemy {
    pub start_y: f32,
    pub time: f32,
    pub moving_right: bool,
}

pub fn spawn_enemy(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("alien.png"),
            transform: Transform::from_xyz(-WINDOW_WIDTH / 2.0, 0.0, 0.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(ENEMY_SIZE / 2.0, ENEMY_SIZE)),
                ..default()
            },
            ..default()
        },
        Enemy {
            start_y: 0.0,
            time: 0.0,
            moving_right: true,
        },
        Velocity { speed: Vec2::ZERO },
    ));
}

pub fn enemy_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Enemy)>,
) {
    for (mut transform, mut enemy) in query.iter_mut() {
        enemy.time += time.delta_seconds();

        let y_offset = (enemy.time * WAVE_FREQUENCY).sin() * WAVE_AMPLITUDE;
        transform.translation.y = enemy.start_y + y_offset;

        if enemy.moving_right {
            transform.translation.x += ENEMY_SPEED * time.delta_seconds();
            if transform.translation.x > WINDOW_WIDTH / 2.0 - ENEMY_SIZE {
                enemy.moving_right = false;
                transform.scale.x = -1.0;
            }
        } else {
            transform.translation.x -= ENEMY_SPEED * time.delta_seconds();
            if transform.translation.x < -WINDOW_WIDTH / 2.0 + ENEMY_SIZE {
                enemy.moving_right = true;
                transform.scale.x = 1.0;
            }
        }
    }
}
