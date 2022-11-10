use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use bevy_extensions::*;
use bootstrap::*;

fn main() {
    App::new()
        .insert_resource(RapierConfiguration {
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(BootstrapPlugin)
        .add_startup_system(setup)
        .add_system_to_stage(CoreStage::PreUpdate, bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands) {
    let player = commands.spawn_actor(ActorConfig::default());
}