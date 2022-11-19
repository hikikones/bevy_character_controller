use bevy::prelude::*;

use super::*;

pub(super) struct TickPlugin;

impl Plugin for TickPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PhysicsTick::default())
            .add_system_to_stage(CoreStage::Last, advance_tick);
    }
}

#[derive(Resource, Default)]
pub struct PhysicsTick(f32);

pub(super) const PHYSICS_TICK_RATE: f32 = 1.0 / 20.0;

impl PhysicsTick {
    pub const fn rate(&self) -> f32 {
        PHYSICS_TICK_RATE
    }

    pub fn percent(&self) -> f32 {
        self.0 / PHYSICS_TICK_RATE
    }
}

fn advance_tick(mut tick: ResMut<PhysicsTick>, time: Res<Time>) {
    if tick.0 >= PHYSICS_TICK_RATE {
        tick.0 -= PHYSICS_TICK_RATE;
    }

    tick.0 += time.delta_seconds();
}

pub(super) fn tick_run_criteria(tick: Res<PhysicsTick>) -> ShouldRun {
    match tick.0 >= PHYSICS_TICK_RATE {
        true => ShouldRun::Yes,
        false => ShouldRun::No,
    }
}
