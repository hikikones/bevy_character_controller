use bevy::prelude::*;
use bevy_actions::*;
use bevy_physics::*;

use crate::assets::*;

#[derive(Component)]
pub enum Block {
    Ground,
    Ground2,
    Ice,
    Spinner,
    Cube,
}

pub trait SpawnBlockExt {
    fn spawn_block(&mut self, assets: &MyAssets, block: Block, transform: Transform) -> &mut Self;
}

impl SpawnBlockExt for Commands<'_, '_> {
    fn spawn_block(&mut self, assets: &MyAssets, block: Block, transform: Transform) -> &mut Self {
        match block {
            Block::Ground => {
                self.spawn((
                    PbrBundle {
                        mesh: assets.mesh(MeshName::Cube),
                        material: assets.material(MaterialName::DarkGray),
                        transform,
                        ..Default::default()
                    },
                    Block::Ground,
                    RigidBody::Fixed,
                    Collider::cuboid(0.5, 0.5, 0.5),
                    CollisionGroups::from(PhysicsLayer::PLATFORM),
                    Friction::coefficient(0.3),
                    Restitution::coefficient(0.0),
                ));
            }
            Block::Ground2 => {
                self.spawn((
                    PbrBundle {
                        mesh: assets.mesh(MeshName::Cube),
                        material: assets.material(MaterialName::SeaGreen),
                        transform,
                        ..Default::default()
                    },
                    Block::Ground,
                    RigidBody::Fixed,
                    Collider::cuboid(0.5, 0.5, 0.5),
                    CollisionGroups::from(PhysicsLayer::PLATFORM),
                    Friction::coefficient(0.3),
                    Restitution::coefficient(0.0),
                ));
            }
            Block::Ice => {
                self.spawn((
                    PbrBundle {
                        mesh: assets.mesh(MeshName::Cube),
                        material: assets.material(MaterialName::Cyan),
                        transform,
                        ..Default::default()
                    },
                    Block::Ice,
                    RigidBody::Fixed,
                    Collider::cuboid(0.5, 0.5, 0.5),
                    CollisionGroups::from(PhysicsLayer::PLATFORM),
                    Friction::coefficient(0.0),
                    Restitution::coefficient(0.0),
                ));
            }
            Block::Spinner => {
                let spinner_sim = self
                    .spawn((
                        TransformBundle::from_transform(transform),
                        Block::Spinner,
                        RigidBody::KinematicVelocityBased,
                        Collider::cuboid(0.5, 0.5, 0.5),
                        CollisionGroups::from(PhysicsLayer::PLATFORM),
                        Friction::coefficient(0.3),
                        Restitution::coefficient(0.0),
                        Velocity {
                            linvel: Vec3::ZERO,
                            angvel: Vec3::X * 1.0,
                        },
                    ))
                    .id();

                self.spawn((
                    PbrBundle {
                        mesh: assets.mesh(MeshName::Cube),
                        material: assets.material(MaterialName::MidnightBlue),
                        transform,
                        ..Default::default()
                    },
                    PhysicsInterpolation {
                        target: spinner_sim,
                        translate: false,
                        rotate: true,
                    },
                ));
            }
            Block::Cube => {
                let start = transform.translation;
                let end = start + transform.forward() * 11.0;

                let block_sim = self
                    .spawn((
                        TransformBundle::from_transform(transform),
                        ActionsBundle::new(),
                        Block::Cube,
                        RigidBody::KinematicPositionBased,
                        Collider::cuboid(0.5, 0.5, 0.5),
                        CollisionGroups::from(PhysicsLayer::BLOCK),
                        Friction::coefficient(0.3),
                        Restitution::coefficient(0.0),
                    ))
                    .id();

                self.spawn((
                    PbrBundle {
                        mesh: assets.mesh(MeshName::Cube),
                        material: assets.material(MaterialName::Red),
                        transform,
                        ..Default::default()
                    },
                    PhysicsInterpolation {
                        target: block_sim,
                        translate: true,
                        rotate: false,
                    },
                ));

                self.actions(block_sim)
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
            }
        }

        self
    }
}
