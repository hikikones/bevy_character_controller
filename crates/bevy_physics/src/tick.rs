use bevy::prelude::*;

use super::*;

pub(super) struct TickPlugin;

impl Plugin for TickPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PhysicsTick::default());
    }
}

pub(super) const PHYSICS_STEP: f32 = 1.0 / 20.0;

#[derive(Resource, Default)]
pub struct PhysicsTick {
    accumulator: f32,
    looping: bool,
}

impl PhysicsTick {
    pub const fn rate(&self) -> f32 {
        PHYSICS_STEP
    }

    pub fn percent(&self) -> f32 {
        self.accumulator / PHYSICS_STEP
    }

    fn update(&mut self, time: &Time) -> ShouldRun {
        if !self.looping {
            self.accumulator += time.delta_seconds();
        }

        if self.accumulator >= PHYSICS_STEP {
            self.accumulator -= PHYSICS_STEP;
            if self.accumulator >= PHYSICS_STEP {
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
