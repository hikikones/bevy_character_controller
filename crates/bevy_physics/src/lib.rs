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
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default().with_default_system_setup(false))
        .add_plugin(RapierDebugRenderPlugin::default())
        // .add_stage_after(CoreStage::Update, "physics", Schedule::default())
        .add_stage_before(
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
            SystemStage::parallel().with_run_criteria(tick_run_criteria),
        )
        .add_stage_after(
            PhysicsStage::PostUpdate,
            "rapier",
            Schedule::default()
                .with_run_criteria(tick_run_criteria)
                .with_stage(
                    "sync",
                    SystemStage::parallel().with_system_set(
                        RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsStages::SyncBackend),
                    ),
                )
                .with_stage(
                    "step",
                    SystemStage::parallel().with_system_set(
                        RapierPhysicsPlugin::<NoUserData>::get_systems(
                            PhysicsStages::StepSimulation,
                        ),
                    ),
                )
                .with_stage(
                    "write",
                    SystemStage::parallel().with_system_set(
                        RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsStages::Writeback),
                    ),
                )
                .with_stage(
                    "detect_despawns",
                    SystemStage::parallel().with_system_set(
                        RapierPhysicsPlugin::<NoUserData>::get_systems(
                            PhysicsStages::DetectDespawn,
                        ),
                    ),
                ),
        )
        // .add_system_set_to_stage(stage_label, system_set)
        // .add_system_to_stage("physics", || {
        //     println!("yoyo");
        // })
        // .add_stage_before(
        //     CoreStage::PreUpdate,
        //     PhysicsStage::PreUpdate,
        //     SystemStage::parallel().with_run_criteria(tick_run_criteria),
        // )
        // .add_stage_before(
        //     CoreStage::Update,
        //     PhysicsStage::Update,
        //     SystemStage::parallel().with_run_criteria(tick_run_criteria),
        // )
        // .add_stage_after(
        //     PhysicsStages::Writeback,
        //     PhysicsStage::PostUpdate,
        //     SystemStage::parallel().with_run_criteria(tick_run_criteria),
        // )
        .add_plugin(TickPlugin)
        .add_plugin(InterpolationPlugin);
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum PhysicsStage {
    PreUpdate,
    Update,
    PostUpdate,
    // Last,
}
