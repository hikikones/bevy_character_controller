use bevy::prelude::*;

pub mod actor;
pub mod assets;
pub mod block;
pub mod camera;
pub mod input;
pub mod platform;

pub use actor::*;
pub use assets::*;
pub use block::*;
pub use camera::*;
pub use input::*;
pub use platform::*;

pub struct BootstrapPlugin;

impl Plugin for BootstrapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(assets::AssetsPlugin)
            .add_plugin(platform::PlatformPlugin)
            .add_plugin(block::BlockPlugin)
            .add_plugin(camera::CameraPlugin)
            .add_plugin(input::InputPlugin)
            .add_startup_system(spawn_light)
            .add_system_to_stage(CoreStage::PreUpdate, bevy::window::close_on_esc);
    }
}

fn spawn_light(mut commands: Commands) {
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 25000.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(10.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
