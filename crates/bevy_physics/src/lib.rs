use bevy::{ecs::schedule::ShouldRun, prelude::*};
use bevy_rapier3d::prelude::*;

pub use bevy_rapier3d::prelude::{
    Ccd, CoefficientCombineRule, Collider, CollisionGroups, Damping, ExternalForce,
    ExternalImpulse, Friction, GravityScale, Group, InteractionGroups,
    KinematicCharacterController, LockedAxes, QueryFilter, RapierContext as PhysicsContext,
    Restitution, RigidBody, Sleeping, Velocity,
};

mod interpolation;
mod layer;
mod tick;

pub use interpolation::*;
pub use layer::*;
pub use tick::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RapierConfiguration {
            timestep_mode: TimestepMode::Fixed {
                dt: PHYSICS_TICK_RATE,
                // time_scale: 1.0,
                substeps: 1,
            },
            ..Default::default()
        })
        .add_stage_after(
            CoreStage::Update,
            "physics",
            Schedule::default()
                .with_run_criteria(tick_run_criteria)
                .with_stage(
                    PhysicsStages::SyncBackend,
                    SystemStage::parallel().with_system_set(
                        RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsStages::SyncBackend),
                    ),
                )
                .with_stage_after(
                    PhysicsStages::SyncBackend,
                    PhysicsStages::StepSimulation,
                    SystemStage::parallel().with_system_set(
                        RapierPhysicsPlugin::<NoUserData>::get_systems(
                            PhysicsStages::StepSimulation,
                        ),
                    ),
                )
                .with_stage_after(
                    PhysicsStages::StepSimulation,
                    PhysicsStages::Writeback,
                    SystemStage::parallel().with_system_set(
                        RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsStages::Writeback),
                    ),
                ),
        )
        .add_stage_before(
            CoreStage::Last,
            PhysicsStages::DetectDespawn,
            SystemStage::parallel().with_system_set(
                RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsStages::DetectDespawn),
            ),
        )
        .add_stage_before("physics", PhysicsStage::Update, SystemStage::parallel())
        .add_stage_after("physics", PhysicsStage::PostUpdate, SystemStage::parallel())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default().with_default_system_setup(false))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(TickPlugin)
        .add_plugin(InterpolationPlugin);
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum PhysicsStage {
    Update,
    PostUpdate,
}
