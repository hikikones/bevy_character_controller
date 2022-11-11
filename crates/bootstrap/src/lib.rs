use bevy::prelude::*;

pub mod assets;
pub mod camera;
pub mod input;
pub mod platform;
pub mod player;

pub use assets::*;
pub use camera::*;
pub use input::*;
pub use platform::*;
pub use player::*;

pub struct BootstrapPlugin;

impl Plugin for BootstrapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(assets::AssetsPlugin)
            .add_plugin(platform::PlatformPlugin)
            .add_plugin(camera::CameraPlugin)
            .add_plugin(input::InputPlugin)
            .add_plugin(player::PlayerPlugin)
            .add_startup_system(spawn_light);
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
