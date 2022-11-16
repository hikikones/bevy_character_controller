use bevy::prelude::*;

pub(super) struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera)
            .add_system_to_stage(CoreStage::PostUpdate, camera_follow);
    }
}

pub const CAMERA_OFFSET: Vec3 = Vec3::new(0.0, 10.0, 10.0);

#[derive(Component)]
pub struct CameraMain;

#[derive(Component)]
pub struct CameraPivot;

pub trait CameraFollowExt {
    fn camera_follow(&mut self, target: Entity) -> &mut Self;
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn_bundle(TransformBundle::default())
        .insert(CameraPivot)
        .with_children(|child| {
            child
                .spawn_bundle(Camera3dBundle {
                    transform: Transform::from_translation(CAMERA_OFFSET)
                        .looking_at(Vec3::ZERO, Vec3::Y),
                    ..Default::default()
                })
                .insert(CameraMain);
        });
}

impl CameraFollowExt for Commands<'_, '_> {
    fn camera_follow(&mut self, target: Entity) -> &mut Self {
        self.add(move |world: &mut World| {
            let camera_pivot = world
                .query_filtered::<Entity, With<CameraPivot>>()
                .single(world);
            world.entity_mut(camera_pivot).insert(Follow(target));
        });

        self
    }
}

#[derive(Component)]
struct Follow(Entity);

fn camera_follow(
    mut follow_q: Query<(&mut Transform, &Follow)>,
    transform_q: Query<&Transform, Without<Follow>>,
) {
    if let Ok((mut transform, follow)) = follow_q.get_single_mut() {
        if let Ok(target) = transform_q.get(follow.0) {
            transform.translation = transform.translation.lerp(target.translation, 0.125);
        }
    }
}
