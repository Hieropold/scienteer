use bevy::prelude::*;
use bevy_pixel_camera::{PixelCameraPlugin, PixelZoom};

mod constants;
mod movement;
mod player;
mod projectile;
mod enemy;
mod background;

use constants::*;
use movement::apply_velocity;
use player::{apply_gravity, player_movement, shoot_chemical, spawn_player};
use projectile::{cleanup_projectiles, rotate_projectiles};
use enemy::{enemy_movement, spawn_enemy};
use background::spawn_background;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
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
                enemy_movement,
            )
                .chain(),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2dBundle::default(),
        PixelZoom::FitSize {
            width: VIRTUAL_WIDTH,
            height: VIRTUAL_HEIGHT,
        },
    ));

    spawn_background(&mut commands, &asset_server);
    spawn_player(&mut commands, &asset_server);
    spawn_enemy(&mut commands, &asset_server);
}
