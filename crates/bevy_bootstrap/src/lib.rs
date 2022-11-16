use bevy::prelude::*;

mod actor;
mod assets;
mod block;
mod camera;
mod input;
mod level;

pub use actor::*;
pub use assets::*;
pub use block::*;
pub use camera::*;
pub use input::*;

pub struct BootstrapPlugin;

impl Plugin for BootstrapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(assets::AssetsPlugin)
            .add_plugin(level::LevelPlugin)
            .add_plugin(camera::CameraPlugin)
            .add_plugin(input::InputPlugin)
            .add_system_to_stage(CoreStage::PreUpdate, bevy::window::close_on_esc);
    }
}
