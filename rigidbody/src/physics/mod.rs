use bevy::{ecs::schedule::ShouldRun, prelude::*};
use bevy_rapier3d::prelude::*;

pub use bevy_rapier3d::prelude::{
    Ccd, CoefficientCombineRule, Collider, CollisionGroups, Damping, ExternalForce,
    ExternalImpulse, Friction, Group, InteractionGroups, LockedAxes, QueryFilter,
    RapierContext as PhysicsContext, RigidBody, Sleeping, Velocity,
};

mod interpolation;
mod layer;
mod time;

pub use interpolation::*;
pub use layer::*;
pub use time::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RapierConfiguration {
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
            PhysicsStage::PreUpdate,
            SystemStage::parallel().with_run_criteria(time_run_criteria),
        )
        .add_stage_before(
            CoreStage::Update,
            PhysicsStage::Update,
            SystemStage::parallel().with_run_criteria(time_run_criteria),
        )
        .add_stage_after(
            PhysicsStages::Writeback,
            PhysicsStage::PostUpdate,
            SystemStage::parallel().with_run_criteria(time_run_criteria),
        )
        .add_plugin(TimePlugin)
        .add_plugin(InterpolationPlugin);
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum PhysicsStage {
    PreUpdate,
    Update,
    PostUpdate,
}
