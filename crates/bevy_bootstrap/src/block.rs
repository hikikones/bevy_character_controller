use bevy::prelude::*;
use bevy_actions::*;
use bevy_extensions::FromLookExt;
use bevy_physics::*;

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
    let start = Vec3::new(-5.5, 0.5, -2.5);
    let end = Vec3::new(5.5, 0.5, -2.5);
    let look = Quat::from_look(Vec3::X, Vec3::Y);

    let block_sim = commands
        .spawn()
        .insert_bundle(TransformBundle::from_transform(Transform {
            translation: start,
            rotation: look,
            ..Default::default()
        }))
        .insert_bundle(ActionsBundle::default())
        .insert_bundle((
            Collider::cuboid(0.5, 0.5, 0.5),
            Friction::coefficient(0.0),
            CollisionGroups::from(PhysicsLayer::BLOCK),
        ))
        .id();

    commands
        .actions(block_sim)
        .config(AddConfig {
            repeat: Repeat::Forever,
            ..Default::default()
        })
        .add(WaitAction::new(1.0))
        .add(LerpAction::new(LerpConfig {
            target: block_sim,
            lerp_type: LerpType::Position(end),
            duration: 2.0,
        }))
        .add(WaitAction::new(1.0))
        .add(LerpAction::new(LerpConfig {
            target: block_sim,
            lerp_type: LerpType::Position(start),
            duration: 2.0,
        }));

    commands
        .spawn_bundle(PbrBundle {
            mesh: assets.mesh(MeshName::Cube),
            material: assets.material(MaterialName::Red),
            transform: Transform {
                translation: start,
                rotation: look,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(BlockName::Cube)
        .insert(Interpolation {
            target: block_sim,
            translate: true,
            rotate: false,
        });
}
