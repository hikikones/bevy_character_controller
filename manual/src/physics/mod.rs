use bevy::prelude::*;

use bevy_extensions::Vec3SwizzlesExt;

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

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum PhysicsLabel {
    PreUpdate,
    Update,
    PostUpdate,
    Last,
}

pub struct PhysicsPlugin;

impl PhysicsPlugin {
    pub fn get_systems(stage: PhysicsStage) -> SystemSet {
        match stage {
            PhysicsStage::PreUpdate => SystemSet::new().label(PhysicsLabel::Update),
            PhysicsStage::Update => SystemSet::new().label(PhysicsLabel::Update),
            PhysicsStage::PostUpdate => SystemSet::new()
                .label(PhysicsLabel::Update)
                .with_system(apply_velocity),
            PhysicsStage::Last => SystemSet::new().label(PhysicsLabel::Update),
        }
    }
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_after(
            CoreStage::Update,
            "physics",
            Schedule::default()
                .with_run_criteria(tick_run_criteria)
                .with_stage(
                    PhysicsStage::PreUpdate,
                    SystemStage::parallel()
                        .with_system_set(Self::get_systems(PhysicsStage::PreUpdate)),
                )
                .with_stage(
                    PhysicsStage::Update,
                    SystemStage::parallel()
                        .with_system_set(Self::get_systems(PhysicsStage::Update)),
                )
                .with_stage(
                    PhysicsStage::PostUpdate,
                    SystemStage::parallel()
                        .with_system_set(Self::get_systems(PhysicsStage::PostUpdate)),
                )
                .with_stage(
                    PhysicsStage::Last,
                    SystemStage::parallel().with_system_set(Self::get_systems(PhysicsStage::Last)),
                ),
        )
        // .add_stage_before(
        //     CoreStage::PreUpdate,
        //     PhysicsStage::PreUpdate,
        //     SystemStage::parallel().with_run_criteria(tick_run_criteria),
        // )
        // .add_stage_before(
        //     CoreStage::Update, // NOOOO!!! Put all fixed stages AFTER UPDATE.....
        //     PhysicsStage::Update,
        //     SystemStage::parallel().with_run_criteria(tick_run_criteria),
        // )
        // .add_stage_before(
        //     CoreStage::PostUpdate,
        //     PhysicsStage::PostUpdate,
        //     SystemStage::parallel()
        //         .with_run_criteria(tick_run_criteria)
        //         .with_system_set(SystemSet::new().with_system(systems::apply_velocity)),
        // )
        // .add_stage_before(
        //     CoreStage::Last,
        //     PhysicsStage::Last,
        //     SystemStage::parallel().with_run_criteria(tick_run_criteria),
        // )
        .add_plugin(TickPlugin)
        .add_plugin(InterpolationPlugin);
    }
}

#[derive(Bundle, Default)]
pub struct PhysicsBundle {
    velocity: Velocity,
    acceleration: Acceleration,
    friction: Friction,
    gravity: Gravity,
    current_velocity: CurrentVelocity,
}

#[derive(Component, Default)]
pub struct Velocity(pub Vec3);

#[derive(Component, Default)]
pub struct Acceleration(pub f32);

#[derive(Component, Default)]
pub struct Friction(pub f32);

#[derive(Component, Default)]
pub struct Gravity(pub f32);

#[derive(Component, Default)]
struct CurrentVelocity(Vec3);

fn apply_velocity(
    mut velocity_q: Query<(
        &mut CurrentVelocity,
        &mut Transform,
        &Velocity,
        &Acceleration,
        &Friction,
        &Gravity,
    )>,
    tick: Res<PhysicsTick>,
) {
    if let Ok((mut current_velocity, mut transform, velocity, acceleration, friction, gravity)) =
        velocity_q.get_single_mut()
    {
        let dt = tick.rate();

        let mut v = current_velocity.0;
        println!("APPLY BEFORE: {}", v);
        println!("VELOCITY: {}", velocity.0);
        // v += velocity.0 * Vec3::new(acceleration.0, gravity.0, acceleration.0);
        v += velocity.0.x0z() * acceleration.0;
        // v -= Vec3::Y * gravity.0 * dt;
        v.y += velocity.0.y;
        v = (v.x0z() * (1.0 - friction.0)).x_z(v.y);

        println!("APPLY AFTER: {}", v);

        // let mut v = velocity.0;
        // v += force.0 * dt;
        // v -= Vec3::Y * gravity.0 * dt;
        // // v = (v.x0z() * ((1.0 - friction.0) * dt)).x_z(v.y);
        // v = (v.x0z() * (1.0 - friction.0)).x_z(v.y);
        // // v = (v.x0z() * friction.0.powf(dt)).x_z(v.y);

        transform.translation += v * dt;

        v.y -= gravity.0 * dt;

        if transform.translation.y < 0.0 {
            transform.translation.y = 0.0;
            v.y = 0.0;
        }

        current_velocity.0 = v;

        // velocity.0 = v;
        // force.0 = Vec3::ZERO;
    }
}
