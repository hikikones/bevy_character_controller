use bevy::prelude::*;

use bevy_bootstrap::{
    ActorConfig, CameraFollowExt, MaterialName, MeshName, MyAssets, SpawnActorExt,
};
use physics::PhysicsInterpolation;

mod board;
mod physics;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_bootstrap::AssetsPlugin)
        .add_plugin(bevy_bootstrap::CameraPlugin)
        .add_plugin(bevy_bootstrap::InputPlugin)
        .add_plugin(board::BoardPlugin)
        .add_plugin(physics::PhysicsPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_startup_system(setup)
        .add_system_to_stage(CoreStage::PreUpdate, bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands, assets: Res<MyAssets>) {
    // Light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 25000.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(10.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    // Floor
    commands.spawn(PbrBundle {
        mesh: assets.mesh(MeshName::Cube),
        material: assets.material(MaterialName::DarkGray),
        transform: Transform {
            translation: -Vec3::Y * 0.6,
            scale: Vec3::new(500.0, 1.0, 500.0),
            ..Default::default()
        },
        ..Default::default()
    });

    // Player
    let player = commands
        .spawn((TransformBundle::default(), player::PlayerBundle::default()))
        .id();

    // Actor
    let actor = commands.spawn_actor(ActorConfig::default());
    // commands.entity(actor).insert(PhysicsInterpolation {
    //     target: player,
    //     translate: true,
    //     rotate: true,
    // });
    commands
        .entity(actor)
        .insert(PhysicsInterpolation::new(player));

    // Camera follow
    // commands.camera_follow(actor);
}
