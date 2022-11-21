use bevy::prelude::*;

use bevy_bootstrap::{InputAction, InputMovement};
use bevy_extensions::{FromLookExt, Vec3SwizzlesExt};

use crate::{board::*, physics::*};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            CoreStage::Update,
            SystemSet::new().with_system(rotation).with_system(jump),
        )
        .add_system_set_to_stage(
            PhysicsStage::PreUpdate,
            SystemSet::new()
                .with_system(set_ground_state)
                .with_system(on_ground_change.after(set_ground_state)),
        )
        .add_system_set_to_stage(PhysicsStage::Update, SystemSet::new().with_system(movement))
        .add_system_set_to_stage(
            PhysicsStage::PostUpdate,
            SystemSet::new()
                .before(PhysicsLabel::PostUpdate)
                .with_system(apply_physics_scalars),
        );
    }
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
                input_on_ground_change: Vec3::ZERO,
                velocity_on_ground_change: Vec3::ZERO,
                previous_ground_state: GroundState::default(),
            },
            ground_state: GroundState::default(),
            physics_bundle: PhysicsBundle::default(),
            speed_scale: SpeedScale(1.0),
            acceleration_scale: AccelerationScale(1.0),
            friction_scale: FrictionScale(1.0),
            gravity_scale: GravityScale(1.0),
            jump_height_scale: JumpHeightScale(1.0),
        }
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerState {
    input_on_ground_change: Vec3,
    velocity_on_ground_change: Vec3,
    previous_ground_state: GroundState,
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

#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq)]
enum GroundState {
    None,
    #[default]
    Normal,
    Slippery,
    Forward,
}

struct Scalars {
    speed: f32,
    acceleration: f32,
    friction: f32,
    gravity: f32,
    jump_height: f32,
}

const BASE_SPEED: f32 = 4.0;
const BASE_ACCELERATION: f32 = BASE_SPEED * 0.5;
const BASE_FRICTION: f32 = 0.4;
const BASE_GRAVITY: f32 = 9.81;
const BASE_JUMP_HEIGHT: f32 = 2.0;

impl GroundState {
    fn scalars(&self) -> Scalars {
        match self {
            GroundState::None => Scalars {
                speed: 1.0,
                acceleration: 0.1,
                friction: 0.1,
                gravity: 1.0,
                jump_height: 0.0,
            },
            GroundState::Normal => Scalars {
                speed: 1.0,
                acceleration: 1.0,
                friction: 1.0,
                gravity: 1.0,
                jump_height: 1.0,
            },
            GroundState::Slippery => Scalars {
                speed: 2.0,
                acceleration: 0.02,
                friction: 0.01,
                gravity: 1.0,
                jump_height: 0.0,
            },
            GroundState::Forward => Scalars {
                speed: 1.0,
                acceleration: 1.0,
                friction: 1.0,
                gravity: 1.0,
                jump_height: 0.0,
            },
        }
    }
}

fn set_ground_state(
    mut player_q: Query<(&mut GroundState, &mut PlayerState, &Transform), With<Player>>,
    platforms: Res<Platforms>,
) {
    let (mut ground_state, mut player_state, transform) = player_q.single_mut();
    let pos = transform.translation;

    let new_ground_state = if pos.y > 0.0 {
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

    if *ground_state != new_ground_state {
        player_state.previous_ground_state = *ground_state;
        *ground_state = new_ground_state;
    }
}

fn on_ground_change(
    mut player_q: Query<
        (
            &GroundState,
            &Velocity,
            &mut PlayerState,
            &mut SpeedScale,
            &mut AccelerationScale,
            &mut FrictionScale,
            &mut GravityScale,
            &mut JumpHeightScale,
        ),
        (Changed<GroundState>, With<Player>),
    >,
    input: Res<InputMovement>,
) {
    if let Ok((
        ground_state,
        velocity,
        mut player_state,
        mut speed_scale,
        mut acceleration_scale,
        mut friction_scale,
        mut gravity_scale,
        mut jump_height_scale,
    )) = player_q.get_single_mut()
    {
        player_state.velocity_on_ground_change = velocity.0;
        player_state.input_on_ground_change = input.x0z();

        let scalars = ground_state.scalars();
        speed_scale.0 = scalars.speed;
        acceleration_scale.0 = scalars.acceleration;
        friction_scale.0 = scalars.friction;
        gravity_scale.0 = scalars.gravity;
        jump_height_scale.0 = scalars.jump_height;
    }
}

fn movement(
    mut player_q: Query<
        (
            &mut Velocity,
            &GroundState,
            &PlayerState,
            &Transform,
            &SpeedScale,
            &AccelerationScale,
        ),
        With<Player>,
    >,
    input: Res<InputMovement>,
) {
    // if input.is_zero() {
    //     // return;
    // }

    let (mut velocity, ground_state, player_state, transform, speed_scale, acceleration_scale) =
        player_q.single_mut();

    let speed = BASE_SPEED * speed_scale.0;
    let acceleration = BASE_ACCELERATION * acceleration_scale.0;

    match ground_state {
        GroundState::Forward => {
            velocity.0 = if player_state
                .velocity_on_ground_change
                .x0z()
                .length_squared()
                > 1.0
            {
                // velocity.move_towards(
                //     transform.forward() * player_state.velocity_on_ground_change.x0z().length(),
                //     acceleration,
                // );
                // dbg!(velocity.0);
                // dbg!(player_state.velocity_on_ground_change.x0z().length());
                // velocity.0 =
                //     transform.forward() * player_state.velocity_on_ground_change.x0z().length();
                transform.forward() * player_state.velocity_on_ground_change.x0z().length()
            } else {
                transform.forward()
                // velocity.move_towards(transform.forward(), acceleration);
            };
        }
        _ => {
            // velocity.move_towards(input.x0z() * speed, acceleration);
            velocity.0 = input.x0z() * speed;
        }
    }
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
        velocity.0.y += f32::sqrt(
            2.0 * BASE_GRAVITY * gravity_scale.0 * BASE_JUMP_HEIGHT * jump_height_scale.0,
        );
        dbg!(velocity.0.y);
    }
}

fn apply_physics_scalars(
    mut player_q: Query<
        (
            &mut Acceleration,
            &mut Friction,
            &mut Gravity,
            &AccelerationScale,
            &FrictionScale,
            &GravityScale,
        ),
        With<Player>,
    >,
) {
    let (
        mut acceleration,
        mut friction,
        mut gravity,
        acceleration_scale,
        friction_scale,
        gravity_scale,
    ) = player_q.single_mut();

    acceleration.0 = BASE_ACCELERATION * acceleration_scale.0;
    friction.0 = BASE_FRICTION * friction_scale.0;
    gravity.0 = BASE_GRAVITY * gravity_scale.0;
}
