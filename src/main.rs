use bevy::prelude::*;
use bevy_pixel_camera::{PixelCameraPlugin, PixelZoom};

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const PLAYER_SPEED: f32 = 100.0;
const JUMP_FORCE: f32 = 300.0;  // Increased from 300
const GRAVITY: f32 = -800.0;    // Reduced from -800
const SPRITE_SIZE: f32 = 32.0;
const CHEMICAL_PROJECTILE_SPEED: f32 = PLAYER_SPEED * 2.0;
const CHEMICAL_PROJECTILE_SIZE: f32 = 16.0;
const CHEMICAL_PROJECTILE_COOLDOWN: f32 = 0.6;
const CHEMICAL_PROJECTILE_ROTATION_SPEED: f32 = 2.0; // Radians per second

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
struct ChemicalProjectile;

#[derive(Component)]
struct Background;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(
                ImagePlugin::default_nearest() // Use nearest-neighbor filtering for crisp pixel art
            ).set(WindowPlugin {
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
                shoot_chemical,
                cleanup_projectiles,
                rotate_projectiles,
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

    // Background
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("levels/lab.png"),
            transform: Transform::from_xyz(0.0, 0.0, -1.0),  // Place behind other elements
            sprite: Sprite {
                custom_size: Some(Vec2::new(320.0, 240.0)),  // Match the PixelZoom dimensions
                ..default()
            },
            ..default()
        },
        Background,
    ));

    // Player with sprite
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("scientist.png"),
            transform: Transform::from_xyz(0.0, -90.0, 0.0),  // Position at 25% from bottom of screen (240 * 0.25 - 240/2 = -90)
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
    if transform.translation.y <= -90.0 && velocity.speed.y <= 0.0 {
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
        
        // Keep player above ground level
        if transform.translation.y < -90.0 {
            transform.translation.y = -90.0;
        }
    }
}

fn shoot_chemical(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut query: Query<(&mut Player, &Transform)>,
) {
    let (mut player, transform) = query.single_mut();
    
    if (keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight)) 
        && time.elapsed_seconds() >= player.last_shot_time + CHEMICAL_PROJECTILE_COOLDOWN {
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

fn cleanup_projectiles(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<ChemicalProjectile>>,
) {
    for (entity, transform) in query.iter() {
        if transform.translation.x < -WINDOW_WIDTH / 2.0 
            || transform.translation.x > WINDOW_WIDTH / 2.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn rotate_projectiles(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<ChemicalProjectile>>,
) {
    for mut transform in query.iter_mut() {
        transform.rotate_z(CHEMICAL_PROJECTILE_ROTATION_SPEED * time.delta_seconds());
    }
}
