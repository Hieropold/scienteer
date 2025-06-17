use bevy::prelude::*;

#[derive(Component)]
pub struct Background;

pub fn spawn_background(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("levels/lab.png"),
            transform: Transform::from_xyz(0.0, 0.0, -1.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(320.0, 240.0)),
                ..default()
            },
            ..default()
        },
        Background,
    ));
}
