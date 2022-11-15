use bevy::prelude::*;
use bevy_extensions::FromLookExt;

use crate::assets::*;

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, spawn_blocks);
    }
}

#[derive(Component)]
pub enum BlockName {
    Cube,
}

fn spawn_blocks(mut commands: Commands, assets: Res<MyAssets>) {
    // Cube
    commands
        .spawn_bundle(PbrBundle {
            mesh: assets.mesh(MeshName::Cube),
            material: assets.material(MaterialName::Red),
            transform: Transform {
                translation: Vec3::new(-5.5, 0.5, -2.5),
                rotation: Quat::from_look(Vec3::X, Vec3::Y),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(BlockName::Cube);
}
