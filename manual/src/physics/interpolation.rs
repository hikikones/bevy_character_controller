use bevy::prelude::*;

use super::*;

pub(super) struct InterpolationPlugin;

impl Plugin for InterpolationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(PhysicsStage::PreUpdate, setup_interpolation)
            .add_system_to_stage(CoreStage::Update, lerp)
            .add_system_to_stage(PhysicsStage::Last, update_transforms);
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

fn lerp(mut lerp_q: Query<(&mut Transform, &PhysicsInterpolation, &Lerp)>, tick: Res<PhysicsTick>) {
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

fn update_transforms(
    mut lerp_q: Query<(&mut Lerp, &PhysicsInterpolation)>,
    target_q: Query<&Transform>,
) {
    for (mut lerp, interpolate) in lerp_q.iter_mut() {
        lerp.0 = lerp.1;
        lerp.1 = *target_q.get(interpolate.target).unwrap();
    }
}
