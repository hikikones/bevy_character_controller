use bevy::prelude::*;

use super::*;

pub(super) struct TickPlugin;

impl Plugin for TickPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PhysicsTick::default());
    }
}

const PHYSICS_TICK_RATE: f64 = 20.0;
pub(super) const PHYSICS_DELTA: f64 = 1.0 / PHYSICS_TICK_RATE;

#[derive(Resource, Default)]
pub struct PhysicsTick {
    accumulator: f64,
    looping: bool,
}

impl PhysicsTick {
    pub const fn rate(&self) -> f32 {
        PHYSICS_TICK_RATE as f32
    }

    pub const fn delta(&self) -> f32 {
        PHYSICS_DELTA as f32
    }

    pub fn percent(&self) -> f32 {
        (self.accumulator / PHYSICS_DELTA) as f32
    }

    fn update(&mut self, time: &Time) -> ShouldRun {
        if !self.looping {
            self.accumulator += time.delta_seconds_f64();
        }

        if self.accumulator >= PHYSICS_DELTA {
            self.accumulator -= PHYSICS_DELTA;
            if self.accumulator >= PHYSICS_DELTA {
                self.looping = true;
                ShouldRun::YesAndCheckAgain
            } else {
                self.looping = false;
                ShouldRun::Yes
            }
        } else {
            self.looping = false;
            ShouldRun::No
        }
    }

    pub(super) fn is_looping(&self) -> bool {
        self.looping
    }
}

pub(super) fn tick_run_criteria(mut tick: ResMut<PhysicsTick>, time: Res<Time>) -> ShouldRun {
    tick.update(&time)
}
