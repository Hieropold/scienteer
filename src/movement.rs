use bevy::prelude::*;

use crate::constants::GROUND_LEVEL;

/// Simple 2D velocity component.
#[derive(Component)]
pub struct Velocity {
    pub speed: Vec2,
}

pub fn apply_velocity(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        let delta = time.delta_seconds();
        transform.translation.x += velocity.speed.x * delta;
        transform.translation.y += velocity.speed.y * delta;
        transform.translation.y = transform.translation.y.max(GROUND_LEVEL);
    }
}
