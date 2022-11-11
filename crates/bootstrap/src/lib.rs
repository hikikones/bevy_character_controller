use bevy::prelude::*;

pub mod actor;
pub mod assets;
pub mod camera;
pub mod input;
pub mod level;

pub use actor::*;
pub use assets::*;
pub use camera::*;
pub use input::*;
pub use level::*;

pub struct BootstrapPlugin;

impl Plugin for BootstrapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(assets::AssetsPlugin)
            .add_plugin(level::LevelPlugin)
            .add_plugin(camera::CameraPlugin)
            .add_plugin(input::InputPlugin)
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
