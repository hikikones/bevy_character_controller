use bevy::{ecs::schedule::ShouldRun, prelude::*};

use bevy_extensions::Vec3SwizzlesExt;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct PhysicsStage;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
struct PhysicsLabel;

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
        self.add_system_to_stage(PhysicsStage, system.before(PhysicsLabel))
    }

    fn add_physics_system_set(&mut self, system_set: SystemSet) -> &mut Self {
        self.add_system_set_to_stage(PhysicsStage, system_set.before(PhysicsLabel))
    }
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PhysicsTick::default())
            .add_stage_after(
                CoreStage::Update,
                PhysicsStage,
                SystemStage::parallel()
                    .with_run_criteria(tick_run_criteria)
                    .with_system_set(
                        SystemSet::new()
                            .label(PhysicsLabel)
                            .with_system(apply_velocity)
                            .with_system(write_transform.after(apply_velocity)),
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
    }
}

const PHYSICS_TICK_RATE: f32 = 1.0 / 33.0;

#[derive(Resource, Default)]
pub struct PhysicsTick {
    accumulator: f32,
    looping: bool,
}

impl PhysicsTick {
    pub const fn _rate(&self) -> f32 {
        PHYSICS_TICK_RATE
    }

    pub fn percent(&self) -> f32 {
        self.accumulator / PHYSICS_TICK_RATE
    }

    fn update(&mut self, time: &Time) -> ShouldRun {
        if !self.looping {
            self.accumulator += time.delta_seconds();
        }

        if self.accumulator >= PHYSICS_TICK_RATE {
            self.accumulator -= PHYSICS_TICK_RATE;
            self.looping = true;
            ShouldRun::YesAndCheckAgain
        } else {
            self.looping = false;
            ShouldRun::No
        }
    }
}

fn tick_run_criteria(mut tick: ResMut<PhysicsTick>, time: Res<Time>) -> ShouldRun {
    tick.update(&time)
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
) {
    if let Ok((mut velocity, mut position, acceleration, friction, gravity)) =
        velocity_q.get_single_mut()
    {
        let dt = PHYSICS_TICK_RATE;

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

fn write_transform(mut write_transform_q: Query<(&mut Transform, &Position)>) {
    if let Ok((mut transform, position)) = write_transform_q.get_single_mut() {
        transform.translation = position.0;
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

fn interpolate(mut lerp_q: Query<(&mut Transform, &Lerp)>, tick: Res<PhysicsTick>) {
    for (mut transform, lerp) in lerp_q.iter_mut() {
        transform.translation = Vec3::lerp(lerp.0, lerp.1, tick.percent());
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
