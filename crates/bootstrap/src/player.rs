use bevy::prelude::*;

use super::assets::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, spawn_player);
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands, assets: Res<MyAssets>) {
    commands
        .spawn_bundle(SpatialBundle::from_transform(Transform {
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            ..Default::default()
        }))
        .insert(Player)
        .with_children(|child| {
            // Capsule
            child.spawn_bundle(PbrBundle {
                mesh: assets.mesh(MeshName::Capsule),
                material: assets.material(MaterialName::White),
                transform: Transform {
                    translation: Vec3::Y,
                    ..Default::default()
                },
                ..Default::default()
            });

            // Eyes
            let eye_left = Vec3::new(-0.2, 1.6, -0.4);
            let eye_right = Vec3::new(-eye_left.x, eye_left.y, eye_left.z);
            let eye_scale = Vec3::splat(0.15);

            child.spawn_bundle(PbrBundle {
                mesh: assets.mesh(MeshName::Icosphere),
                material: assets.material(MaterialName::Black),
                transform: Transform {
                    translation: eye_left,
                    scale: eye_scale,
                    ..Default::default()
                },
                ..Default::default()
            });
            child.spawn_bundle(PbrBundle {
                mesh: assets.mesh(MeshName::Icosphere),
                material: assets.material(MaterialName::Black),
                transform: Transform {
                    translation: eye_right,
                    scale: eye_scale,
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}
