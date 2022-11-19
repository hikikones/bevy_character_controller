use bevy::prelude::*;
use bevy_extensions::FromLookExt;

use crate::{assets::*, Block, SpawnBlockExt};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_set_to_stage(
            StartupStage::PreStartup,
            SystemSet::new()
                .with_system(spawn_light)
                .with_system(spawn_blocks),
        );
    }
}

fn spawn_light(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 25000.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(10.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn spawn_blocks(mut commands: Commands, assets: Res<MyAssets>) {
    commands
        // Floor
        .spawn_block(
            &assets,
            Block::Ground,
            Transform {
                translation: -Vec3::Y * 0.5,
                scale: Vec3::new(500.0, 1.0, 500.0),
                ..Default::default()
            },
        )
        // Ice
        .spawn_block(
            &assets,
            Block::Ice,
            Transform {
                translation: -Vec3::Z * 6.0,
                scale: Vec3::new(12.0, 1.0, 6.0),
                ..Default::default()
            },
        )
        // Cube
        .spawn_block(
            &assets,
            Block::Cube,
            Transform {
                translation: Vec3::new(-5.5, 0.5, -2.5),
                rotation: Quat::from_look(Vec3::X, Vec3::Y),
                ..Default::default()
            },
        )
        // Ramp
        .spawn_block(
            &assets,
            Block::Ground2,
            Transform {
                translation: Vec3::new(-5.0, 0.0, -0.5),
                rotation: Quat::from_rotation_z(-15.0_f32.to_radians()),
                scale: Vec3::new(5.0, 1.0, 2.5),
            },
        )
        // Ramp steep
        .spawn_block(
            &assets,
            Block::Ground2,
            Transform {
                translation: Vec3::new(-5.0, 0.0, 2.0),
                rotation: Quat::from_rotation_z(-35.0_f32.to_radians()),
                scale: Vec3::new(7.0, 1.0, 2.5),
            },
        )
        // Wall
        .spawn_block(
            &assets,
            Block::Ground2,
            Transform {
                translation: Vec3::new(-6.0, 0.0, 4.0),
                scale: Vec3::new(1.0, 8.0, 1.0),
                ..Default::default()
            },
        )
        // Spinner
        .spawn_block(
            &assets,
            Block::Spinner,
            Transform {
                translation: Vec3::X * 5.0,
                scale: Vec3::new(5.0, 3.0, 2.5),
                ..Default::default()
            },
        );
}
