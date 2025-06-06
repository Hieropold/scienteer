use bevy::prelude::*;
use crate::constants::{
    PLAYER_SPEED,
    JUMP_FORCE,
    GRAVITY,
    SPRITE_SIZE,
    CHEMICAL_PROJECTILE_SPEED,
    CHEMICAL_PROJECTILE_SIZE,
    CHEMICAL_PROJECTILE_COOLDOWN,
    GROUND_LEVEL,
};
use crate::movement::Velocity;
use crate::projectile::ChemicalProjectile;

/// Marker component for the player character.
#[derive(Component)]
pub struct Player {
    pub is_jumping: bool,
    pub facing_right: bool,
    pub last_shot_time: f32,
}

pub fn spawn_player(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("scientist.png"),
            transform: Transform::from_xyz(0.0, GROUND_LEVEL, 0.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(SPRITE_SIZE, SPRITE_SIZE)),
                ..default()
            },
            ..default()
        },
        Player {
            is_jumping: false,
            facing_right: true,
            last_shot_time: 0.0,
        },
        Velocity { speed: Vec2::ZERO },
    ));
}

pub fn player_movement(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Player, &Transform, &mut Sprite)>,
) {
    let (mut velocity, mut player, _transform, mut sprite) = query.single_mut();

    let mut direction = 0.0;
    if keyboard.pressed(KeyCode::A) || keyboard.pressed(KeyCode::Left) {
        direction -= 1.0;
        player.facing_right = false;
        sprite.flip_x = true;
    }
    if keyboard.pressed(KeyCode::D) || keyboard.pressed(KeyCode::Right) {
        direction += 1.0;
        player.facing_right = true;
        sprite.flip_x = false;
    }

    if (keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::Up)) && !player.is_jumping {
        velocity.speed.y = JUMP_FORCE;
        player.is_jumping = true;
    }

    velocity.speed.x = direction * PLAYER_SPEED;
}

pub fn apply_gravity(
    mut query: Query<(&mut Velocity, &mut Player, &Transform)>,
    time: Res<Time>,
) {
    let (mut velocity, mut player, transform) = query.single_mut();

    velocity.speed.y += GRAVITY * time.delta_seconds();

    if transform.translation.y <= GROUND_LEVEL && velocity.speed.y <= 0.0 {
        velocity.speed.y = 0.0;
        player.is_jumping = false;
    }
}

pub fn shoot_chemical(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut query: Query<(&mut Player, &Transform)>,
) {
    let (mut player, transform) = query.single_mut();

    if (keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight))
        && time.elapsed_seconds() >= player.last_shot_time + CHEMICAL_PROJECTILE_COOLDOWN
    {
        let direction = if player.facing_right { 1.0 } else { -1.0 };

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("bottle.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(CHEMICAL_PROJECTILE_SIZE, CHEMICAL_PROJECTILE_SIZE)),
                    flip_x: !player.facing_right,
                    ..default()
                },
                transform: Transform::from_xyz(
                    transform.translation.x,
                    transform.translation.y,
                    0.0,
                ),
                ..default()
            },
            ChemicalProjectile,
            Velocity {
                speed: Vec2::new(CHEMICAL_PROJECTILE_SPEED * direction, 0.0),
            },
        ));

        player.last_shot_time = time.elapsed_seconds();
    }
}
