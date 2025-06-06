use bevy::prelude::*;

use crate::constants::{VIRTUAL_HEIGHT, VIRTUAL_WIDTH};

/// Static background image for the level.
#[derive(Component)]
pub struct Background;

pub fn spawn_background(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("levels/lab.png"),
            transform: Transform::from_xyz(0.0, 0.0, -1.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(
                    VIRTUAL_WIDTH as f32,
                    VIRTUAL_HEIGHT as f32,
                )),
                ..default()
            },
            ..default()
        },
        Background,
    ));
}
