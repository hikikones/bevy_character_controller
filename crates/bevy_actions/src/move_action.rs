use bevy::prelude::*;
use bevy_extensions::{FromLookExt, MoveTowardsTransformExt};
use bevy_physics::{PhysicsStage, PhysicsTick};
use bevy_sequential_actions::*;

use super::IntoValue;

pub struct MoveActionPlugin;

impl Plugin for MoveActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            PhysicsStage::Update,
            SystemSet::new().with_system(movement).with_system(rotation),
        );
    }
}

pub struct MoveAction<V, F>
where
    V: IntoValue<Vec3>,
    F: IntoValue<f32>,
{
    config: MoveConfig<V, F>,
    bundle: Option<MoveBundle>,
}

pub struct MoveConfig<V, F>
where
    V: IntoValue<Vec3>,
    F: IntoValue<f32>,
{
    pub target: V,
    pub speed: F,
    pub rotate: bool,
}

impl<V, F> MoveAction<V, F>
where
    V: IntoValue<Vec3>,
    F: IntoValue<f32>,
{
    pub fn new(config: MoveConfig<V, F>) -> Self {
        Self {
            config,
            bundle: None,
        }
    }
}

impl<V, F> Action for MoveAction<V, F>
where
    V: IntoValue<Vec3>,
    F: IntoValue<f32>,
{
    fn on_start(&mut self, agent: Entity, world: &mut World, _commands: &mut ActionCommands) {
        let move_bundle = self.bundle.take().unwrap_or(MoveBundle {
            target: Target(self.config.target.value()),
            speed: Speed(self.config.speed.value()),
        });

        let mut agent = world.entity_mut(agent);

        if self.config.rotate {
            let start = agent.get::<Transform>().unwrap().translation;
            let dir = (move_bundle.target.0 - start).normalize_or_zero();
            if dir != Vec3::ZERO {
                agent.insert(Rotate(Quat::from_look(dir, Vec3::Y)));
            }
        }

        agent.insert_bundle(move_bundle);
    }

    fn on_stop(&mut self, agent: Entity, world: &mut World, reason: StopReason) {
        let mut agent = world.entity_mut(agent);
        let bundle = agent.remove_bundle::<MoveBundle>();

        if self.config.rotate {
            agent.remove::<Rotate>();
        }

        if let StopReason::Paused = reason {
            self.bundle = bundle;
        }
    }
}

#[derive(Bundle)]
struct MoveBundle {
    target: Target,
    speed: Speed,
}

#[derive(Component)]
struct Target(Vec3);

#[derive(Component)]
struct Speed(f32);

#[derive(Component)]
struct Rotate(Quat);

fn movement(
    mut move_q: Query<(Entity, &mut Transform, &Target, &Speed)>,
    mut commands: Commands,
    tick: Res<PhysicsTick>,
) {
    for (agent, mut transform, target, speed) in move_q.iter_mut() {
        transform.move_towards(target.0, speed.0 * tick.rate());

        if transform.translation == target.0 {
            commands.actions(agent).next();
        }
    }
}

fn rotation(mut rot_q: Query<(&mut Transform, &Speed, &Rotate)>, tick: Res<PhysicsTick>) {
    for (mut transform, speed, rotate) in rot_q.iter_mut() {
        transform.rotation = Quat::slerp(transform.rotation, rotate.0, speed.0 * 2.0 * tick.rate());
    }
}
