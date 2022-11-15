use bevy::prelude::*;
use bevy_extensions::RandomExt;
use bevy_sequential_actions::*;

mod move_action;

pub use move_action::*;

pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {}
}

pub trait IntoValue<T = Self>
where
    Self: Send + Sync + 'static,
{
    fn value(&self) -> T;
}

impl IntoValue for f32 {
    fn value(&self) -> Self {
        *self
    }
}

impl IntoValue for Vec3 {
    fn value(&self) -> Self {
        *self
    }
}

#[derive(Clone, Copy)]
pub struct Random<T>
where
    T: RandomExt,
{
    min: T,
    max: T,
}

impl<T> Random<T>
where
    T: RandomExt,
{
    pub fn _new(min: T, max: T) -> Self {
        Self { min, max }
    }
}

impl<T> IntoValue<T> for Random<T>
where
    T: Clone + Copy + RandomExt + IntoValue<T>,
{
    fn value(&self) -> T {
        T::random(self.min, self.max)
    }
}
