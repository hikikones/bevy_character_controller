use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera);
    }
}

pub const CAMERA_OFFSET: Vec3 = Vec3::new(0.0, 10.0, 10.0);

#[derive(Component)]
pub struct CameraMain;

#[derive(Component)]
pub struct CameraPivot;

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
