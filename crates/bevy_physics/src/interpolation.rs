use bevy::prelude::*;

use super::*;

pub(super) struct InterpolationPlugin;

impl Plugin for InterpolationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PreUpdate, setup_interpolation)
            .add_system_to_stage(CoreStage::Update, interpolate)
            .add_system_to_stage(PhysicsStage::PostUpdate, update_interpolation);
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

fn interpolate(mut lerp_q: Query<(&mut Transform, &Interpolation, &Lerp)>, tick: Res<PhysicsTick>) {
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
