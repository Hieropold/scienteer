use bevy::prelude::*;
use bevy_pixel_camera::{PixelCameraPlugin, PixelZoom};

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const PLAYER_SPEED: f32 = 150.0;

#[derive(Component)]
struct Player;

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
        .add_systems(Update, (player_movement, apply_velocity))
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn((
        Camera2dBundle::default(),
        PixelZoom::FitSize { width: 320, height: 240 },
    ));

    // Player (temporary rectangle for now)
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Player,
        Velocity { speed: Vec2::ZERO },
    ));
}

fn player_movement(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
    time: Res<Time>,
) {
    let mut velocity = query.single_mut();
    let mut direction = Vec2::ZERO;

    if keyboard.pressed(KeyCode::A) || keyboard.pressed(KeyCode::Left) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::D) || keyboard.pressed(KeyCode::Right) {
        direction.x += 1.0;
    }
    if keyboard.pressed(KeyCode::Space) || keyboard.pressed(KeyCode::Up) {
        direction.y += 1.0;
    }

    if direction != Vec2::ZERO {
        direction = direction.normalize();
    }

    velocity.speed = direction * PLAYER_SPEED * time.delta_seconds();
}

fn apply_velocity(mut query: Query<(&Velocity, &mut Transform)>, _time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation.x += velocity.speed.x;
        transform.translation.y += velocity.speed.y;
    }
}
