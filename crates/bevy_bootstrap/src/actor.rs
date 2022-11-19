use bevy::prelude::*;

use super::assets::*;

#[derive(Component)]
pub struct Actor;

#[derive(Default)]
pub struct ActorConfig {
    pub position: Vec3,
    pub rotation: Quat,
}

pub trait SpawnActorExt {
    fn spawn_actor(&mut self, config: ActorConfig) -> Entity;
}

impl SpawnActorExt for Commands<'_, '_> {
    fn spawn_actor(&mut self, config: ActorConfig) -> Entity {
        let entity = self.spawn_empty().id();

        self.add(move |w: &mut World| {
            w.resource_scope(|w: &mut World, assets: Mut<MyAssets>| {
                w.entity_mut(entity)
                    .insert((
                        Actor,
                        SpatialBundle::from_transform(Transform {
                            translation: config.position,
                            rotation: config.rotation,
                            ..Default::default()
                        }),
                    ))
                    .with_children(|child| {
                        // Capsule
                        child.spawn(PbrBundle {
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

                        child.spawn(PbrBundle {
                            mesh: assets.mesh(MeshName::Icosphere),
                            material: assets.material(MaterialName::Black),
                            transform: Transform {
                                translation: eye_left,
                                scale: eye_scale,
                                ..Default::default()
                            },
                            ..Default::default()
                        });
                        child.spawn(PbrBundle {
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
            })
        });

        entity
    }
}
