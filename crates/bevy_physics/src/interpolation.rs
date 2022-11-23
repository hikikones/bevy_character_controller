use bevy::prelude::*;

use super::*;

pub(super) struct InterpolationPlugin;

impl Plugin for InterpolationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PreUpdate, setup_interpolation)
            .add_system_to_stage(CoreStage::Update, interpolate)
            .add_system_to_stage(
                PhysicsStage,
                update_interpolation.after(PhysicsLabel::Writeback),
            );
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

fn interpolate(
    mut lerp_q: Query<(&mut Transform, &PhysicsInterpolation, &Lerp)>,
    tick: Res<PhysicsTick>,
) {
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

fn update_interpolation(
    mut lerp_q: Query<(&mut Lerp, &PhysicsInterpolation)>,
    transform_q: Query<&Transform>,
    tick: Res<PhysicsTick>,
) {
    if tick.is_looping() {
        return;
    }

    for (mut lerp, interpolate) in lerp_q.iter_mut() {
        lerp.0 = lerp.1;
        lerp.1 = *transform_q.get(interpolate.target).unwrap();
    }
}
