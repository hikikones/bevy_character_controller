use bevy::prelude::*;

use bevy_extensions::Vec3SwizzlesExt;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .before(bevy::transform::transform_propagate_system)
                .with_system(apply_velocity),
        );
    }
}

#[derive(Component, Default)]
pub struct Velocity {
    pub linear: Vec3,
    current: Vec3,
}

#[derive(Component, Default)]
pub struct Acceleration(pub f32);

#[derive(Component, Default)]
pub struct Friction(pub f32);

#[derive(Component, Default)]
pub struct Gravity(pub f32);

#[derive(Bundle, Default)]
pub struct PhysicsBundle {
    velocity: Velocity,
    acceleration: Acceleration,
    friction: Friction,
    gravity_scale: Gravity,
}

fn apply_velocity(
    mut velocity_q: Query<(
        &mut Transform,
        &mut Velocity,
        &Acceleration,
        &Friction,
        &Gravity,
    )>,
    time: Res<Time>,
) {
    if let Ok((mut transform, mut velocity, acceleration, frction, gravity)) =
        velocity_q.get_single_mut()
    {
        let dt = time.delta_seconds();

        // Horizontal
        let mut horizontal = velocity.current.x0z();
        horizontal += velocity.linear.x0z() * acceleration.0;
        horizontal *= 1.0 - frction.0;

        // Vertical
        let mut y = velocity.current.y;
        y += velocity.linear.y;

        // Apply velocity
        transform.translation += horizontal.x_z(y) * dt;

        // Gravity
        y -= gravity.0 * dt;

        velocity.current = horizontal.x_z(y);

        if transform.translation.y < 0.0 {
            transform.translation.y = 0.0;
            velocity.current.y = 0.0;
        }
    }
}
