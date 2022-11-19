use bevy::prelude::*;

use bevy_actions::*;
use bevy_bootstrap::*;
// use bevy_extensions::*;
use bevy_physics::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin)
        .add_plugin(ActionsPlugin)
        .add_plugin(BootstrapPlugin)
        .add_startup_system(setup)
        .add_system_set_to_stage(
            CoreStage::Update,
            SystemSet::new().with_system(movement),
            // .with_system(rotation)
            // .with_system(jump),
        )
        .run();
}

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands) {
    // Player
    let player = commands.spawn_actor(ActorConfig::default());
    commands.entity(player).insert((
        Player,
        Collider::capsule((Vec3::Y * 0.5).into(), (Vec3::Y * 1.5).into(), 0.5),
        KinematicCharacterController::default(),
    ));

    // Camera follow
    commands.camera_follow(player);
}

const MAX_SPEED: f32 = 10.0;
// const MAX_ACCELERATION: f32 = MAX_SPEED * 2.0;
// const ROTATION_SPEED: f32 = MAX_SPEED * 1.5;
// const JUMP_HEIGHT: f32 = 2.0;

fn movement(
    mut player_q: Query<&mut KinematicCharacterController, With<Player>>,
    input: Res<InputMovement>,
    time: Res<Time>,
) {
    let mut controller = player_q.single_mut();

    let input = input.x0z();
    let dt = time.delta_seconds();
    let target = input * MAX_SPEED;
    // let max_delta = MAX_ACCELERATION * dt;

    controller.translation = Some(target * dt);

    // let v = velocity
    //     .linvel
    //     .move_towards(target, max_delta)
    //     .x_z(velocity.linvel.y);
}

// fn rotation(
//     mut player_q: Query<&mut Transform, With<Player>>,
//     input: Res<InputMovement>,
//     time: Res<Time>,
// ) {
//     if input.is_zero() {
//         return;
//     }

//     let mut transform = player_q.single_mut();
//     transform.rotation = Quat::slerp(
//         transform.rotation,
//         Quat::from_look(input.x0z(), Vec3::Y),
//         ROTATION_SPEED * time.delta_seconds(),
//     );
// }

// fn jump(mut player_q: Query<&mut ExternalImpulse, With<Player>>, input_action: Res<InputAction>) {
//     if let InputAction::Jump = *input_action {
//         let force = Vec3::Y * f32::sqrt(2.0 * 9.81 * JUMP_HEIGHT);
//         player_q.single_mut().impulse = force;
//     }
// }
