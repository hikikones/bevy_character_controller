use bevy::prelude::*;

use super::*;

pub(super) struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SimulationTick::default())
            .add_system_to_stage(CoreStage::Last, advance_tick);
    }
}

#[derive(Default)]
pub struct SimulationTick(f32);

pub(super) const SIMULATION_TICK_RATE: f32 = 1.0 / 10.0;

impl SimulationTick {
    pub const fn rate(&self) -> f32 {
        SIMULATION_TICK_RATE
    }

    pub fn percent(&self) -> f32 {
        self.0 / SIMULATION_TICK_RATE
    }
}

fn advance_tick(mut tick: ResMut<SimulationTick>, time: Res<Time>) {
    if tick.0 >= SIMULATION_TICK_RATE {
        tick.0 -= SIMULATION_TICK_RATE;
    }

    tick.0 += time.delta_seconds();
}

pub(super) fn time_run_criteria(tick: Res<SimulationTick>) -> ShouldRun {
    match tick.0 >= SIMULATION_TICK_RATE {
        true => ShouldRun::Yes,
        false => ShouldRun::No,
    }
}
