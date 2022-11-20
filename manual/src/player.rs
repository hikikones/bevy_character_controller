use bevy::prelude::*;

use bevy_bootstrap::{InputAction, InputMovement};
use bevy_extensions::{FromLookExt, MoveTowardsExt, Vec3SwizzlesExt};

use crate::{board::*, physics::*};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            CoreStage::PreUpdate,
            SystemSet::new()
                .with_system(set_ground_state)
                .with_system(on_ground_change.after(set_ground_state)),
        )
        .add_system_set_to_stage(
            CoreStage::Update,
            SystemSet::new()
                .with_system(movement)
                .with_system(rotation)
                .with_system(jump.after(movement))
                .with_system(apply_physics_scalars.after(jump)),
        );
    }
}

const BASE_SPEED: f32 = 10.0;
const BASE_ACCELERATION: f32 = BASE_SPEED * 0.5;
const BASE_FRICTION: f32 = BASE_SPEED * 1.0;
const BASE_GRAVITY: f32 = 9.81;
const BASE_JUMP_HEIGHT: f32 = 2.0;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerState {
    velocity_on_ground_change: Vec3,
    // after_ground_change
}

#[derive(Component)]
struct SpeedScale(f32);

#[derive(Component)]
struct AccelerationScale(f32);

#[derive(Component)]
struct FrictionScale(f32);

#[derive(Component)]
struct GravityScale(f32);

#[derive(Component)]
struct JumpHeightScale(f32);

#[derive(Component, Debug, PartialEq, Eq)]
enum GroundState {
    None,
    Normal,
    Slippery,
    Forward,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    marker: Player,
    state: PlayerState,
    ground_state: GroundState,

    #[bundle]
    physics_bundle: PhysicsBundle,

    speed_scale: SpeedScale,
    acceleration_scale: AccelerationScale,
    friction_scale: FrictionScale,
    gravity_scale: GravityScale,
    jump_height_scale: JumpHeightScale,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            marker: Player,
            state: PlayerState {
                velocity_on_ground_change: Vec3::ZERO,
            },
            ground_state: GroundState::Normal,
            physics_bundle: PhysicsBundle::default(),
            speed_scale: SpeedScale(1.0),
            acceleration_scale: AccelerationScale(1.0),
            friction_scale: FrictionScale(1.0),
            gravity_scale: GravityScale(1.0),
            jump_height_scale: JumpHeightScale(1.0),
        }
    }
}

fn movement(
    mut player_q: Query<
        (
            &mut Velocity,
            &PlayerState,
            &Transform,
            &SpeedScale,
            &GroundState,
        ),
        With<Player>,
    >,
    input: Res<InputMovement>,
) {
    if input.is_zero() {
        return;
    }

    let (mut velocity, state, transform, speed_scale, ground_state) = player_q.single_mut();

    velocity.0 = velocity
        .move_towards(input.x0z() * BASE_SPEED, BASE_ACCELERATION)
        .x_z(velocity.y);

    // match ground_state {
    //     GroundState::Forward => {
    //         velocity.linear = if state.velocity_on_ground_change.x0z().length_squared() > 1.0 {
    //             transform.forward() * state.velocity_on_ground_change.x0z().length()
    //         } else {
    //             transform.forward()
    //         };
    //     }
    //     _ => {
    //         velocity.linear = input.x0z() * BASE_SPEED * speed_scale.0;
    //     }
    // }
}

fn rotation(
    mut player_q: Query<&mut Transform, With<Player>>,
    input: Res<InputMovement>,
    time: Res<Time>,
) {
    if input.is_zero() {
        return;
    }

    const ROTATION_SPEED: f32 = 15.0;

    let mut transform = player_q.single_mut();
    transform.rotation = Quat::slerp(
        transform.rotation,
        Quat::from_look(input.x0z(), Vec3::Y),
        ROTATION_SPEED * time.delta_seconds(),
    );
}

fn jump(
    mut player_q: Query<(&mut Velocity, &GravityScale, &JumpHeightScale), With<Player>>,
    input_action: Res<InputAction>,
) {
    if let InputAction::Jump = *input_action {
        let (mut velocity, gravity_scale, jump_height_scale) = player_q.single_mut();
        velocity.y += f32::sqrt(
            2.0 * BASE_GRAVITY * gravity_scale.0 * BASE_JUMP_HEIGHT * jump_height_scale.0,
        );
    }
}

fn set_ground_state(
    mut player_q: Query<(&mut GroundState, &Transform), With<Player>>,
    platforms: Res<Platforms>,
) {
    let (mut ground_state, transform) = player_q.single_mut();
    let pos = transform.translation;

    let state = if pos.y > 0.0 {
        GroundState::None
    } else if let Some(platform) = platforms.get_tile_from_point(pos) {
        match platform {
            Platform::Ground => GroundState::Normal,
            Platform::Ice => GroundState::Slippery,
            Platform::Skate => GroundState::Forward,
        }
    } else {
        GroundState::Normal
    };

    if *ground_state != state {
        *ground_state = state;
    }
}

fn on_ground_change(
    mut player_q: Query<
        (
            &mut PlayerState,
            &mut SpeedScale,
            &mut AccelerationScale,
            &mut FrictionScale,
            &mut GravityScale,
            &GroundState,
            &Velocity,
        ),
        (Changed<GroundState>, With<Player>),
    >,
) {
    if let Ok((
        mut state,
        mut speed_scale,
        mut acceleration_scale,
        mut friction_scale,
        mut gravity_scale,
        ground_state,
        velocity,
    )) = player_q.get_single_mut()
    {
        state.velocity_on_ground_change = velocity.0;

        match ground_state {
            GroundState::None => {
                speed_scale.0 = 1.0;
                acceleration_scale.0 = 0.05;
                friction_scale.0 = 0.05;
                gravity_scale.0 = 1.0;
            }
            GroundState::Normal => {
                speed_scale.0 = 1.0;
                acceleration_scale.0 = 1.0;
                friction_scale.0 = 1.0;
                gravity_scale.0 = 1.0;
            }
            GroundState::Slippery => {
                speed_scale.0 = 1.0;
                acceleration_scale.0 = 0.02;
                friction_scale.0 = 0.01;
                gravity_scale.0 = 0.0;
            }
            GroundState::Forward => {
                speed_scale.0 = 1.0;
                acceleration_scale.0 = 1.0;
                friction_scale.0 = 1.0;
                gravity_scale.0 = 1.0;
            }
        }
    }
}

fn apply_physics_scalars(
    mut player_q: Query<
        (
            &mut Friction,
            &mut Gravity,
            &AccelerationScale,
            &FrictionScale,
            &GravityScale,
        ),
        With<Player>,
    >,
) {
    let (mut friction, mut gravity, acceleration_scale, friction_scale, gravity_scale) =
        player_q.single_mut();

    friction.0 = BASE_FRICTION * friction_scale.0;
    gravity.0 = BASE_GRAVITY * gravity_scale.0;
}
