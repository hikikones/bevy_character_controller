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
struct PhysicsStage;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum PhysicsLabel {
    PreUpdate,
    Update,
    PostUpdate,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
enum RapierLabel {
    SyncBackend,
    StepSimulation,
    Writeback,
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RapierConfiguration {
            timestep_mode: TimestepMode::Fixed {
                dt: PHYSICS_DELTA as f32,
                substeps: 1,
            },
            ..Default::default()
        })
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default().with_default_system_setup(false))
        .add_stage_after(
            CoreStage::Update,
            PhysicsStage,
            SystemStage::parallel()
                .with_run_criteria(tick_run_criteria)
                .with_system_set(
                    SystemSet::new()
                        .label(PhysicsLabel::PreUpdate)
                        .with_system(|| {}),
                )
                .with_system_set(
                    SystemSet::new()
                        .label(PhysicsLabel::Update)
                        .after(PhysicsLabel::PreUpdate)
                        .with_system(|| {}),
                )
                .with_system_set(
                    SystemSet::new()
                        .label(PhysicsLabel::PostUpdate)
                        .after(PhysicsLabel::Update)
                        .with_system(|| {}),
                )
                .with_system_set(
                    RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsStages::SyncBackend)
                        .label(RapierLabel::SyncBackend)
                        .after(PhysicsLabel::PostUpdate),
                )
                .with_system_set(
                    RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsStages::StepSimulation)
                        .label(RapierLabel::StepSimulation)
                        .after(RapierLabel::SyncBackend),
                )
                .with_system_set(
                    RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsStages::Writeback)
                        .label(RapierLabel::Writeback)
                        .after(RapierLabel::StepSimulation),
                ),
        )
        .add_stage_before(
            CoreStage::Last,
            PhysicsStages::DetectDespawn,
            SystemStage::parallel().with_system_set(
                RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsStages::DetectDespawn),
            ),
        )
        .add_plugin(TickPlugin)
        .add_plugin(InterpolationPlugin);

        #[cfg(debug_assertions)]
        app.add_plugin(RapierDebugRenderPlugin::default());
    }
}

pub trait PhysicsAppExt {
    fn add_physics_system<Params>(
        &mut self,
        label: PhysicsLabel,
        system: impl IntoSystemDescriptor<Params>,
    ) -> &mut Self;

    fn add_physics_system_set(&mut self, label: PhysicsLabel, system_set: SystemSet) -> &mut Self;
}

impl PhysicsAppExt for App {
    fn add_physics_system<Params>(
        &mut self,
        label: PhysicsLabel,
        system: impl IntoSystemDescriptor<Params>,
    ) -> &mut Self {
        match label {
            PhysicsLabel::PreUpdate => {
                self.add_system_to_stage(PhysicsStage, system.before(PhysicsLabel::PreUpdate))
            }
            PhysicsLabel::Update => self.add_system_to_stage(
                PhysicsStage,
                system
                    .before(PhysicsLabel::Update)
                    .after(PhysicsLabel::PreUpdate),
            ),
            PhysicsLabel::PostUpdate => self.add_system_to_stage(
                PhysicsStage,
                system
                    .before(PhysicsLabel::PostUpdate)
                    .after(PhysicsLabel::Update),
            ),
        }
    }

    fn add_physics_system_set(&mut self, label: PhysicsLabel, system_set: SystemSet) -> &mut Self {
        match label {
            PhysicsLabel::PreUpdate => self
                .add_system_set_to_stage(PhysicsStage, system_set.before(PhysicsLabel::PreUpdate)),
            PhysicsLabel::Update => self.add_system_set_to_stage(
                PhysicsStage,
                system_set
                    .before(PhysicsLabel::Update)
                    .after(PhysicsLabel::PreUpdate),
            ),
            PhysicsLabel::PostUpdate => self.add_system_set_to_stage(
                PhysicsStage,
                system_set
                    .before(PhysicsLabel::PostUpdate)
                    .after(PhysicsLabel::Update),
            ),
        }
    }
}
