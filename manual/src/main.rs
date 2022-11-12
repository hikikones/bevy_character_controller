use bevy::{prelude::*, window::PresentMode};

use bevy_extensions::*;
use bootstrap::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            present_mode: PresentMode::AutoNoVsync,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(BootstrapPlugin)
        .add_startup_system(setup)
        .add_system_set_to_stage(
            CoreStage::Update,
            SystemSet::new().with_system(movement).with_system(rotation),
        )
        .run();
}

fn setup(
    player_q: Query<Entity, With<Player>>,
    platform_q: Query<(Entity, &Platform)>,
    mut commands: Commands,
) {
    // ...
}

const MAX_SPEED: f32 = 10.0;
const MAX_ACCELERATION: f32 = MAX_SPEED * 2.0;
const ROTATION_SPEED: f32 = MAX_SPEED * 1.5;
const JUMP_HEIGHT: f32 = 2.0;

fn movement(
    mut player_q: Query<&mut Transform, With<Player>>,
    input: Res<InputMovement>,
    time: Res<Time>,
) {
    let mut transform = player_q.single_mut();

    let input = input.x0z();
    let dt = time.delta_seconds();
    let target = input * MAX_SPEED;
    let max_delta = MAX_ACCELERATION * dt;

    transform.translation += target * dt;
}

fn rotation(
    mut player_q: Query<&mut Transform, With<Player>>,
    input: Res<InputMovement>,
    time: Res<Time>,
) {
    if input.is_zero() {
        return;
    }

    let mut transform = player_q.single_mut();
    transform.rotation = Quat::slerp(
        transform.rotation,
        Quat::from_look(input.x0z(), Vec3::Y),
        ROTATION_SPEED * time.delta_seconds(),
    );
}
