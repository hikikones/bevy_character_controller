use bevy::prelude::*;

use bevy_extensions::Vec3SwizzlesExt;

mod tick;

pub use tick::*;

const PHYSICS_TICK_RATE: f32 = 1.0 / 10.0;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum PhysicsStage {
    Update,
    PostUpdate,
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_before(
            CoreStage::Update,
            PhysicsStage::Update,
            SystemStage::parallel().with_run_criteria(tick_run_criteria),
        )
        .add_stage_before(
            CoreStage::PostUpdate,
            PhysicsStage::PostUpdate,
            SystemStage::parallel()
                .with_run_criteria(tick_run_criteria)
                .with_system_set(SystemSet::new().with_system(apply_velocity)),
        );
    }
}

#[derive(Component, Default)]
pub struct Velocity {
    pub linear: Vec3,
    current: Vec3,
}

#[derive(Component, Default)]
pub struct Force(pub Vec3);

impl Velocity {
    pub fn current(&self) -> Vec3 {
        self.current
    }
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
    force: Force,
    acceleration: Acceleration,
    friction: Friction,
    gravity_scale: Gravity,
}

fn apply_velocity(
    mut velocity_q: Query<(
        &mut Transform,
        &mut Velocity,
        &mut Force,
        &Acceleration,
        &Friction,
        &Gravity,
    )>,
    time: Res<Time>,
) {
    if let Ok((mut transform, mut velocity, mut force, acceleration, friction, gravity)) =
        velocity_q.get_single_mut()
    {
        let dt = time.delta_seconds();

        let mut v = velocity.linear;
        v += force.0 * dt;
        v -= Vec3::Y * gravity.0 * dt;
        v = (v.x0z() * (1.0 - friction.0 * dt)).x_z(v.y);

        transform.translation += v * dt;

        if transform.translation.y < 0.0 {
            transform.translation.y = 0.0;
            v.y = 0.0;
        }

        velocity.linear = v;

        force.0 = Vec3::ZERO;

        // // Horizontal
        // let mut horizontal = velocity.current.x0z();
        // horizontal += velocity.linear.x0z() * acceleration.0;
        // horizontal *= 1.0 - friction.0;

        // // Vertical
        // let mut y = velocity.current.y;
        // y += velocity.linear.y;

        // // Apply velocity
        // transform.translation += horizontal.x_z(y) * dt;

        // // Gravity
        // y -= gravity.0 * dt;

        // velocity.current = horizontal.x_z(y);

        // if transform.translation.y < 0.0 {
        //     transform.translation.y = 0.0;
        //     velocity.current.y = 0.0;
        // }
    }
}
