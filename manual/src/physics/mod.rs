use bevy::{
    prelude::*,
    time::{FixedTimestep, FixedTimesteps},
};

use bevy_extensions::Vec3SwizzlesExt;

// mod interpolation;
// mod tick;

// pub use interpolation::*;
// pub use tick::*;

// #[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
// pub enum PhysicsStage {
//     PreUpdate,
//     Update,
//     PostUpdate,
//     Last,
// }

// #[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
// pub enum PhysicsLabel {
//     PreUpdate,
//     Update,
//     PostUpdate,
//     Last,
// }

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct MyPhysicsStage;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
struct MyPhysicsLabel;

// #[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
// struct PhysicsSchedule;

// #[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
// enum MyPhysicsStage {
//     Update,
//     Write,
// }

const PHYSICS_TIMESTEP_LABEL: &str = "physics_timestep";

pub trait PhysicsAppExt {
    fn add_physics_system<Params>(
        &mut self,
        system: impl IntoSystemDescriptor<Params>,
    ) -> &mut Self;

    fn add_physics_system_set(&mut self, system_set: SystemSet) -> &mut Self;
}

impl PhysicsAppExt for App {
    fn add_physics_system<Params>(
        &mut self,
        system: impl IntoSystemDescriptor<Params>,
    ) -> &mut Self {
        // self.schedule
        //     .stage(PhysicsSchedule, |schedule: &mut Schedule| {
        //         schedule.add_system_to_stage(MyPhysicsStage::Update, system)
        //     });
        // self
        self.add_system_to_stage(MyPhysicsStage, system.before(MyPhysicsLabel))
    }

    fn add_physics_system_set(&mut self, system_set: SystemSet) -> &mut Self {
        // self.schedule
        //     .stage(PhysicsSchedule, |schedule: &mut Schedule| {
        //         schedule.add_system_set_to_stage(MyPhysicsStage::Update, system_set)
        //     });
        // self
        self.add_system_set_to_stage(MyPhysicsStage, system_set.before(MyPhysicsLabel))
    }
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_after(
            CoreStage::Update,
            MyPhysicsStage,
            SystemStage::parallel()
                .with_run_criteria(
                    FixedTimestep::step(1.0 / 33.0).with_label(PHYSICS_TIMESTEP_LABEL),
                )
                .with_system_set(
                    SystemSet::new()
                        .label(MyPhysicsLabel)
                        .with_system(apply_velocity),
                ),
        )
        .add_system_set_to_stage(
            CoreStage::PreUpdate,
            SystemSet::new()
                .with_system(setup_physics)
                .with_system(setup_interpolation),
        )
        .add_system(interpolate)
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new().with_system(update_interpolation),
        );

        // .add_stage_after(
        //     CoreStage::Update,
        //     PhysicsSchedule,
        //     Schedule::default()
        //         .with_run_criteria(FixedTimestep::step(1.0 / 33.0))
        //         .with_stage(
        //             MyPhysicsStage::Update,
        //             SystemStage::parallel().with_system(setup_interpolation),
        //         )
        //         .with_stage(
        //             MyPhysicsStage::Write,
        //             SystemStage::parallel().with_system_set(
        //                 SystemSet::new()
        //                     .with_system(apply_velocity)
        //                     .with_system(update_transforms.after(apply_velocity)),
        //             ),
        //         ),
        // );
        // .add_system(lerp);
        // app.add_stage_before(
        //     CoreStage::PreUpdate,
        //     PhysicsStage::PreUpdate,
        //     SystemStage::parallel()
        //         .with_run_criteria(tick_run_criteria)
        //         .with_system_set(SystemSet::new().label(PhysicsLabel::PreUpdate)),
        // )
        // .add_stage_before(
        //     CoreStage::Update,
        //     PhysicsStage::Update,
        //     SystemStage::parallel()
        //         .with_run_criteria(tick_run_criteria)
        //         .with_system_set(SystemSet::new().label(PhysicsLabel::Update)),
        // )
        // .add_stage_before(
        //     CoreStage::PostUpdate,
        //     PhysicsStage::PostUpdate,
        //     SystemStage::parallel()
        //         .with_run_criteria(tick_run_criteria)
        //         .with_system_set(
        //             SystemSet::new()
        //                 .label(PhysicsLabel::PostUpdate)
        //                 .with_system(apply_velocity),
        //         ),
        // )
        // .add_stage_before(
        //     CoreStage::Last,
        //     PhysicsStage::Last,
        //     SystemStage::parallel()
        //         .with_run_criteria(tick_run_criteria)
        //         .with_system_set(SystemSet::new().label(PhysicsLabel::Last)),
        // )
        // .add_plugin(TickPlugin)
        // .add_plugin(InterpolationPlugin);
    }
}

#[derive(Bundle, Default)]
pub struct PhysicsBundle {
    velocity: Velocity,
    acceleration: Acceleration,
    friction: Friction,
    gravity: Gravity,
    position: Position,
}

#[derive(Component, Default)]
pub struct Velocity {
    pub target: Vec3,
    current: Vec3,
    added: Vec3,
}

impl Velocity {
    pub fn add(&mut self, v: Vec3) {
        self.added += v;
    }
}

#[derive(Component, Default)]
pub struct Acceleration(pub f32);

#[derive(Component, Default)]
pub struct Friction(pub f32);

#[derive(Component, Default)]
pub struct Gravity(pub f32);

#[derive(Component, Default)]
struct Position(Vec3);

fn setup_physics(mut physics_added_q: Query<(&mut Position, &Transform)>) {
    for (mut position, transform) in physics_added_q.iter_mut() {
        position.0 = transform.translation;
    }
}

fn apply_velocity(
    mut velocity_q: Query<(
        &mut Velocity,
        &mut Position,
        &Acceleration,
        &Friction,
        &Gravity,
    )>,
    fixed_timesteps: Res<FixedTimesteps>,
) {
    if let Ok((mut velocity, mut position, acceleration, friction, gravity)) =
        velocity_q.get_single_mut()
    {
        let dt = fixed_timesteps.get(PHYSICS_TIMESTEP_LABEL).unwrap().step() as f32;

        let mut v = velocity.current;
        v += velocity.added;
        v += velocity.target * acceleration.0;
        v = (v.x0z() * (1.0 - friction.0)).x_z(v.y);

        position.0 += v * dt;

        v.y -= gravity.0 * dt;

        if position.0.y < 0.0 {
            position.0.y = 0.0;
            v.y = 0.0;
        }

        velocity.current = v;
        velocity.added = Vec3::ZERO;
    }
}

#[derive(Component)]
pub struct PhysicsInterpolation(Entity);

impl PhysicsInterpolation {
    pub fn new(target: Entity) -> Self {
        Self(target)
    }
}

#[derive(Component)]
struct Lerp(Vec3, Vec3);

fn setup_interpolation(
    lerp_added_q: Query<(Entity, &Transform), Added<PhysicsInterpolation>>,
    mut commands: Commands,
) {
    for (entity, transform) in lerp_added_q.iter() {
        let pos = transform.translation;
        commands.entity(entity).insert(Lerp(pos, pos));
    }
}

fn interpolate(
    mut lerp_q: Query<(&mut Transform, &PhysicsInterpolation, &Lerp)>,
    fixed_timesteps: Res<FixedTimesteps>,
) {
    let t = fixed_timesteps
        .get(PHYSICS_TIMESTEP_LABEL)
        .unwrap()
        .overstep_percentage() as f32;
    for (mut transform, interpolate, lerp) in lerp_q.iter_mut() {
        transform.translation = Vec3::lerp(lerp.0, lerp.1, t);
    }
}

fn update_interpolation(
    mut lerp_q: Query<(&mut Lerp, &PhysicsInterpolation)>,
    position_changed_q: Query<&Position, Changed<Position>>,
) {
    for (mut lerp, interpolate) in lerp_q.iter_mut() {
        if let Ok(pos) = position_changed_q.get(interpolate.0) {
            lerp.0 = lerp.1;
            lerp.1 = pos.0;
        }
    }
}
