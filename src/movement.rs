use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity {
    pub speed: Vec2,
}

pub fn apply_velocity(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        let delta = time.delta_seconds();
        transform.translation.x += velocity.speed.x * delta;
        transform.translation.y += velocity.speed.y * delta;
        if transform.translation.y < -90.0 {
            transform.translation.y = -90.0;
        }
    }
}
