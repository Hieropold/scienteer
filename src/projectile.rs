use bevy::prelude::*;
use crate::constants::{WINDOW_WIDTH, CHEMICAL_PROJECTILE_ROTATION_SPEED};

/// Projectile shot by the player.
#[derive(Component)]
pub struct ChemicalProjectile;

pub fn cleanup_projectiles(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<ChemicalProjectile>>,
) {
    for (entity, transform) in query.iter() {
        if transform.translation.x < -WINDOW_WIDTH / 2.0
            || transform.translation.x > WINDOW_WIDTH / 2.0
        {
            commands.entity(entity).despawn();
        }
    }
}

pub fn rotate_projectiles(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<ChemicalProjectile>>,
) {
    for mut transform in query.iter_mut() {
        transform.rotate_z(CHEMICAL_PROJECTILE_ROTATION_SPEED * time.delta_seconds());
    }
}
