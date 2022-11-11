use bevy::prelude::*;

use crate::assets::*;

pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, spawn_platforms);
    }
}

#[derive(Component)]
pub enum Platform {
    Ground,
    Ice,
}

fn spawn_platforms(mut commands: Commands, assets: Res<MyAssets>) {
    // Ground
    commands
        .spawn_bundle(PbrBundle {
            mesh: assets.mesh(MeshName::Cube),
            material: assets.material(MaterialName::DarkGray),
            transform: Transform {
                translation: -Vec3::Y * 0.5,
                scale: Vec3::new(500.0, 1.0, 500.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Platform::Ground);

    // Ice
    commands
        .spawn_bundle(PbrBundle {
            mesh: assets.mesh(MeshName::Cube),
            material: assets.material(MaterialName::Cyan),
            transform: Transform {
                translation: -Vec3::Z * 6.0,
                scale: Vec3::new(12.0, 1.0, 6.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Platform::Ice);
}
