use bevy::prelude::*;
use bevy_pixel_camera::{PixelCameraPlugin, PixelZoom};

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const PLAYER_SPEED: f32 = 50.0;
const JUMP_FORCE: f32 = 400.0;  // Increased from 300
const GRAVITY: f32 = -600.0;    // Reduced from -800
const SPRITE_SIZE: f32 = 32.0;
const FIREBALL_SPEED: f32 = PLAYER_SPEED * 2.0;
const FIREBALL_SIZE: f32 = 16.0;
const FIREBALL_COOLDOWN: f32 = 0.6;

#[derive(Component)]
struct Player {
    is_jumping: bool,
    facing_right: bool,
    last_shot_time: f32,
}

#[derive(Component)]
struct Velocity {
    speed: Vec2,
}

#[derive(Component)]
struct Fireball;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Scienteer".into(),
                    resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                    ..default()
                }),
                ..default()
            }),
            PixelCameraPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                player_movement,
                apply_gravity,
                apply_velocity,
                shoot_fireball,
                cleanup_fireballs,
            ).chain(),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn((
        Camera2dBundle::default(),
        PixelZoom::FitSize { width: 320, height: 240 },
    ));

    // Player with sprite
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("scientist.png"),
            transform: Transform::from_xyz(0.0, SPRITE_SIZE / 2.0, 0.0),  // Start slightly above ground
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

fn player_movement(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Player, &Transform, &mut Sprite)>,
) {
    let (mut velocity, mut player, _transform, mut sprite) = query.single_mut();

    // Horizontal movement
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

    // Jumping
    if (keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::Up)) && !player.is_jumping {
        velocity.speed.y = JUMP_FORCE;
        player.is_jumping = true;
    }

    // Apply horizontal movement
    velocity.speed.x = direction * PLAYER_SPEED;
}

fn apply_gravity(
    mut query: Query<(&mut Velocity, &mut Player, &Transform)>,
    time: Res<Time>,
) {
    let (mut velocity, mut player, transform) = query.single_mut();
    
    // Apply gravity
    velocity.speed.y += GRAVITY * time.delta_seconds();

    // Check if landed (accounting for sprite size)
    if transform.translation.y <= SPRITE_SIZE / 2.0 && velocity.speed.y <= 0.0 {
        velocity.speed.y = 0.0;
        player.is_jumping = false;
    }
}

fn apply_velocity(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        let delta = time.delta_seconds();
        
        // Apply movement with delta time
        transform.translation.x += velocity.speed.x * delta;
        transform.translation.y += velocity.speed.y * delta;
        
        // Keep player above ground level (accounting for sprite size)
        if transform.translation.y < SPRITE_SIZE / 2.0 {
            transform.translation.y = SPRITE_SIZE / 2.0;
        }
    }
}

fn shoot_fireball(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut query: Query<(&mut Player, &Transform)>,
) {
    let (mut player, transform) = query.single_mut();
    
    if (keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight)) 
        && time.elapsed_seconds() >= player.last_shot_time + FIREBALL_COOLDOWN {
        let direction = if player.facing_right { 1.0 } else { -1.0 };
        
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("fireball.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(FIREBALL_SIZE, FIREBALL_SIZE)),
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
            Fireball,
            Velocity {
                speed: Vec2::new(FIREBALL_SPEED * direction, 0.0),
            },
        ));
        
        player.last_shot_time = time.elapsed_seconds();
    }
}

fn cleanup_fireballs(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Fireball>>,
) {
    for (entity, transform) in query.iter() {
        if transform.translation.x < -WINDOW_WIDTH / 2.0 
            || transform.translation.x > WINDOW_WIDTH / 2.0 {
            commands.entity(entity).despawn();
        }
    }
}
