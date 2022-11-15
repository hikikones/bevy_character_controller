use bevy::{ecs::schedule::ShouldRun, prelude::*};
use bevy_rapier3d::prelude::*;

pub use bevy_rapier3d::prelude::{
    Ccd, CoefficientCombineRule, Collider, CollisionGroups, Damping, ExternalForce,
    ExternalImpulse, Friction, Group, InteractionGroups, LockedAxes, QueryFilter,
    RapierContext as PhysicsContext, RigidBody, Sleeping, Velocity,
};

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SimulationTick::default())
            .insert_resource(RapierConfiguration {
                timestep_mode: TimestepMode::Interpolated {
                    dt: SIMULATION_TICK_RATE,
                    time_scale: 1.0,
                    substeps: 1,
                },
                ..Default::default()
            })
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_stage_before(
                CoreStage::PreUpdate,
                SimulationStage::PreUpdate,
                SystemStage::parallel().with_run_criteria(run_criteria),
            )
            .add_stage_before(
                CoreStage::Update,
                SimulationStage::Update,
                SystemStage::parallel().with_run_criteria(run_criteria),
            )
            .add_stage_after(
                PhysicsStages::Writeback,
                SimulationStage::PostUpdate,
                SystemStage::parallel().with_run_criteria(run_criteria),
            )
            .add_system_to_stage(CoreStage::Last, tick)
            // Interpolation
            .add_system_to_stage(CoreStage::PreUpdate, setup_interpolation)
            .add_system_to_stage(CoreStage::Update, interpolate)
            .add_system_to_stage(SimulationStage::PostUpdate, update_interpolation);
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum SimulationStage {
    PreUpdate,
    Update,
    PostUpdate,
}

#[derive(Default)]
pub struct SimulationTick(f32);

const SIMULATION_TICK_RATE: f32 = 1.0 / 10.0;

impl SimulationTick {
    pub const fn rate(&self) -> f32 {
        SIMULATION_TICK_RATE
    }

    pub fn percent(&self) -> f32 {
        self.0 / SIMULATION_TICK_RATE
    }
}

fn tick(mut tick: ResMut<SimulationTick>, time: Res<Time>) {
    if tick.0 >= SIMULATION_TICK_RATE {
        tick.0 -= SIMULATION_TICK_RATE;
    }

    tick.0 += time.delta_seconds();
}

fn run_criteria(tick: Res<SimulationTick>) -> ShouldRun {
    match tick.0 >= SIMULATION_TICK_RATE {
        true => ShouldRun::Yes,
        false => ShouldRun::No,
    }
}

#[derive(Component)]
pub struct Interpolation {
    pub target: Entity,
    pub translate: bool,
    pub rotate: bool,
}

#[derive(Component)]
struct Lerp(Transform, Transform);

fn setup_interpolation(
    simu_added_q: Query<(Entity, &Transform), Added<Interpolation>>,
    mut commands: Commands,
) {
    for (entity, transform) in simu_added_q.iter() {
        commands.entity(entity).insert(Lerp(*transform, *transform));
    }
}

fn interpolate(
    mut lerp_q: Query<(&mut Transform, &Interpolation, &Lerp)>,
    tick: Res<SimulationTick>,
) {
    for (mut transform, simu, lerp) in lerp_q.iter_mut() {
        if simu.translate {
            transform.translation =
                Vec3::lerp(lerp.0.translation, lerp.1.translation, tick.percent());
        }

        if simu.rotate {
            transform.rotation = Quat::slerp(lerp.0.rotation, lerp.1.rotation, tick.percent());
        }
    }
}

fn update_interpolation(
    mut lerp_q: Query<(&mut Lerp, &Interpolation)>,
    target_q: Query<&Transform>,
) {
    for (mut lerp, simu) in lerp_q.iter_mut() {
        lerp.0 = lerp.1;
        lerp.1 = *target_q.get(simu.target).unwrap();
    }
}
