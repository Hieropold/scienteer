use bevy::prelude::*;
use bevy_pixel_camera::{PixelCameraPlugin, PixelZoom};

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const PLAYER_SPEED: f32 = 150.0;
const JUMP_FORCE: f32 = 300.0;
const GRAVITY: f32 = -800.0;
const SPRITE_SIZE: f32 = 32.0;

#[derive(Component)]
struct Player {
    is_jumping: bool,
    facing_right: bool,
}

#[derive(Component)]
struct Velocity {
    speed: Vec2,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Retro Side-Scroller".into(),
                    resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                    ..default()
                }),
                ..default()
            }),
            PixelCameraPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (player_movement, apply_velocity, apply_gravity))
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
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(SPRITE_SIZE, SPRITE_SIZE)),
                ..default()
            },
            ..default()
        },
        Player {
            is_jumping: false,
            facing_right: true,
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
    if keyboard.just_pressed(KeyCode::Space) && !player.is_jumping {
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

    // Check if landed
    if transform.translation.y <= 0.0 {
        velocity.speed.y = 0.0;
        player.is_jumping = false;
    }
}

fn apply_velocity(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation.x += velocity.speed.x * time.delta_seconds();
        transform.translation.y += velocity.speed.y * time.delta_seconds();
        
        // Keep player on the ground
        if transform.translation.y < 0.0 {
            transform.translation.y = 0.0;
        }
    }
}
