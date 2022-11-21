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

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum PhysicsStage {
    PreUpdate,
    Update,
    // PostUpdate,
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
            CoreStage::Last,
            PhysicsStage::Last,
            SystemStage::parallel().with_run_criteria(tick_run_criteria),
        )
        .add_plugin(CustomRapierPlugin)
        .add_plugin(TickPlugin)
        .add_plugin(InterpolationPlugin);
    }
}

struct CustomRapierPlugin;

impl Plugin for CustomRapierPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RapierConfiguration {
            timestep_mode: TimestepMode::Fixed {
                dt: PHYSICS_TICK_RATE,
                substeps: 1,
            },
            ..Default::default()
        })
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default().with_default_system_setup(false))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_stage_after(
            CoreStage::Update,
            "rapier_physics",
            Schedule::default()
                .with_run_criteria(tick_run_criteria)
                .with_stage(
                    "sync_backend",
                    SystemStage::parallel().with_system_set(
                        RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsStages::SyncBackend),
                    ),
                )
                .with_stage(
                    "step_simulation",
                    SystemStage::parallel().with_system_set(
                        RapierPhysicsPlugin::<NoUserData>::get_systems(
                            PhysicsStages::StepSimulation,
                        ),
                    ),
                )
                .with_stage(
                    "write_back",
                    SystemStage::parallel().with_system_set(
                        RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsStages::Writeback),
                    ),
                ),
        )
        .add_stage_before(
            CoreStage::Last,
            "detect_despawns",
            SystemStage::parallel().with_system_set(
                RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsStages::DetectDespawn),
            ),
        );
    }
}
