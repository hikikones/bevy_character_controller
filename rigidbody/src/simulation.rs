use bevy::{ecs::schedule::ShouldRun, prelude::*};
use bevy_rapier3d::prelude::*;

pub use bevy_rapier3d::prelude::{
    Ccd, Collider, CollisionGroups, Damping, ExternalForce, ExternalImpulse, Friction, Group,
    LockedAxes, RigidBody, Sleeping, Velocity,
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
            .add_system_to_stage(CoreStage::Last, tick);
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

const SIMULATION_TICK_RATE: f32 = 1.0 / 5.0;

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
