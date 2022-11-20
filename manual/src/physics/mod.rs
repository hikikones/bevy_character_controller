use bevy::prelude::*;

use bevy_extensions::{MoveTowardsExt, Vec3SwizzlesExt};

mod interpolation;
mod tick;

pub use interpolation::*;
pub use tick::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum PhysicsStage {
    PreUpdate,
    Update,
    PostUpdate,
    Last,
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_before(
            CoreStage::PreUpdate,
            PhysicsStage::PreUpdate,
            SystemStage::parallel().with_run_criteria(tick_run_criteria),
        )
        .add_stage_before(
            CoreStage::Update,
            PhysicsStage::Update,
            SystemStage::parallel().with_run_criteria(tick_run_criteria),
        )
        .add_stage_before(
            CoreStage::PostUpdate,
            PhysicsStage::PostUpdate,
            SystemStage::parallel()
                .with_run_criteria(tick_run_criteria)
                .with_system_set(SystemSet::new().with_system(systems::apply_velocity)),
        )
        .add_stage_before(
            CoreStage::Last,
            PhysicsStage::Last,
            SystemStage::parallel().with_run_criteria(tick_run_criteria),
        )
        .add_plugin(TickPlugin)
        .add_plugin(InterpolationPlugin);
    }
}

#[derive(Bundle, Default)]
pub struct PhysicsBundle {
    velocity: Velocity,
    force: Force,
    friction: Friction,
    gravity: Gravity,
}

#[derive(Component, Default, Deref, DerefMut)]
pub struct Velocity(pub Vec3);

impl Velocity {
    pub fn move_towards(&mut self, target: Vec3, max_delta: f32) {
        self.0 = self.0.x0z().move_towards(target, max_delta).x_z(self.0.y);
    }
}

#[derive(Component, Default, Deref, DerefMut)]
pub struct Force(pub Vec3);

impl Force {
    pub fn _add(&mut self, f: Vec3) {
        self.0 += f;
    }
}

#[derive(Component, Default)]
pub struct Friction(pub f32);

#[derive(Component, Default)]
pub struct Gravity(pub f32);

pub mod systems {
    use bevy::prelude::*;

    use super::*;

    pub fn apply_velocity(
        mut velocity_q: Query<(
            &mut Transform,
            &mut Velocity,
            &mut Force,
            &Friction,
            &Gravity,
        )>,
        // time: Res<Time>,
        tick: Res<PhysicsTick>,
    ) {
        if let Ok((mut transform, mut velocity, mut force, friction, gravity)) =
            velocity_q.get_single_mut()
        {
            let dt = tick.rate();

            let mut v = velocity.0;
            v += force.0 * dt;
            v -= Vec3::Y * gravity.0 * dt;
            // v = (v.x0z() * ((1.0 - friction.0) * dt)).x_z(v.y);
            v = (v.x0z() * (1.0 - friction.0)).x_z(v.y);
            // v = (v.x0z() * friction.0.powf(dt)).x_z(v.y);

            transform.translation += v * dt;

            if transform.translation.y < 0.0 {
                transform.translation.y = 0.0;
                v.y = 0.0;
            }

            velocity.0 = v;
            force.0 = Vec3::ZERO;
        }
    }
}
