use bevy::{ecs::schedule::ShouldRun, prelude::*};

use bevy_extensions::{MoveTowardsExt, Vec3SwizzlesExt};

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct PhysicsStage;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
struct PhysicsLabel;

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
                            .with_system(update_interpolation.after(apply_velocity)),
                    ),
            )
            .add_system_set_to_stage(
                CoreStage::PreUpdate,
                SystemSet::new().with_system(setup_interpolation),
            )
            .add_system(interpolate);
    }
}

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

const PHYSICS_TICK_RATE: f64 = 20.0;
const PHYSICS_DELTA: f64 = 1.0 / PHYSICS_TICK_RATE;

#[derive(Resource, Default)]
pub struct PhysicsTick {
    accumulator: f64,
    looping: bool,
}

impl PhysicsTick {
    pub const fn _rate(&self) -> f32 {
        PHYSICS_TICK_RATE as f32
    }

    pub const fn delta(&self) -> f32 {
        PHYSICS_DELTA as f32
    }

    pub fn percent(&self) -> f32 {
        (self.accumulator / PHYSICS_DELTA) as f32
    }

    fn update(&mut self, time: &Time) -> ShouldRun {
        if !self.looping {
            self.accumulator += time.delta_seconds_f64();
        }

        if self.accumulator >= PHYSICS_DELTA {
            self.accumulator -= PHYSICS_DELTA;
            if self.accumulator >= PHYSICS_DELTA {
                self.looping = true;
                ShouldRun::YesAndCheckAgain
            } else {
                self.looping = false;
                ShouldRun::Yes
            }
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
    impulse: Impulse,
    damping: Damping,
    gravity: Gravity,
}

#[derive(Component, Default)]
pub struct Velocity(pub Vec3);

impl Velocity {
    pub fn move_towards(&mut self, target: Vec3, acceleration: f32) {
        self.0 = self
            .0
            .x0z()
            .move_towards(target, acceleration)
            .x_z(self.0.y);
    }
}

#[derive(Component, Default)]
pub struct Impulse(pub Vec3);

#[derive(Component, Default)]
pub struct Damping(pub f32);

#[derive(Component, Default)]
pub struct Gravity(pub f32);

fn apply_velocity(
    mut velocity_q: Query<(
        &mut Velocity,
        &mut Impulse,
        &mut Transform,
        &Damping,
        &Gravity,
    )>,
    tick: Res<PhysicsTick>,
) {
    if let Ok((mut velocity, mut impulse, mut transform, damping, gravity)) =
        velocity_q.get_single_mut()
    {
        let dt = tick.delta();

        let mut v = velocity.0;
        v += impulse.0;
        v = (v.x0z() * (1.0 - damping.0 * dt)).x_z(v.y);

        transform.translation += v * dt;

        v.y -= gravity.0 * dt;

        if transform.translation.y < 0.0 {
            transform.translation.y = 0.0;
            v.y = 0.0;
        }

        velocity.0 = v;
        impulse.0 = Vec3::ZERO;
    }
}

#[derive(Component)]
pub struct PhysicsInterpolation {
    pub target: Entity,
    pub translate: bool,
    pub rotate: bool,
}

#[derive(Component)]
struct Lerp(Transform, Transform);

fn setup_interpolation(
    lerp_added_q: Query<(Entity, &Transform), Added<PhysicsInterpolation>>,
    mut commands: Commands,
) {
    for (entity, transform) in lerp_added_q.iter() {
        commands.entity(entity).insert(Lerp(*transform, *transform));
    }
}

fn interpolate(
    mut lerp_q: Query<(&mut Transform, &PhysicsInterpolation, &Lerp)>,
    tick: Res<PhysicsTick>,
) {
    for (mut transform, interpolate, lerp) in lerp_q.iter_mut() {
        if interpolate.translate {
            transform.translation =
                Vec3::lerp(lerp.0.translation, lerp.1.translation, tick.percent());
        }

        if interpolate.rotate {
            transform.rotation = Quat::slerp(lerp.0.rotation, lerp.1.rotation, tick.percent());
        }
    }
}

fn update_interpolation(
    mut lerp_q: Query<(&mut Lerp, &PhysicsInterpolation)>,
    transform_q: Query<&Transform>,
    tick: Res<PhysicsTick>,
) {
    if tick.looping {
        return;
    }

    for (mut lerp, interpolate) in lerp_q.iter_mut() {
        lerp.0 = lerp.1;
        lerp.1 = *transform_q.get(interpolate.target).unwrap();
    }
}
