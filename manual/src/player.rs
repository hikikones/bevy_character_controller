use bevy::prelude::*;

use bevy_bootstrap::{Actor, InputAction, InputMovement};
use bevy_extensions::{FromLookExt, Vec3SwizzlesExt};

use crate::{board::*, physics::*};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            CoreStage::Update,
            SystemSet::new().with_system(rotation).with_system(jump),
        )
        .add_physics_system_set(
            PhysicsLabel::PreUpdate,
            SystemSet::new()
                .with_system(set_ground_state)
                .with_system(on_ground_change.after(set_ground_state)),
        )
        .add_physics_system_set(PhysicsLabel::Update, SystemSet::new().with_system(movement))
        .add_physics_system_set(
            PhysicsLabel::PostUpdate,
            SystemSet::new().with_system(apply_physics_scalars),
        );
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    marker: Player,
    history: PlayerHistory,
    ground_state: GroundState,

    #[bundle]
    physics_bundle: PhysicsBundle,

    speed_scale: SpeedScale,
    acceleration_scale: AccelerationScale,
    damping_scale: DampingScale,
    gravity_scale: GravityScale,
    jump_height_scale: JumpHeightScale,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            marker: Player,
            history: PlayerHistory {
                input_on_ground_change: Vec3::ZERO,
                velocity_on_ground_change: Vec3::ZERO,
                forward_on_ground_change: Vec3::ZERO,
                previous_ground_state: GroundState::default(),
            },
            ground_state: GroundState::default(),
            physics_bundle: PhysicsBundle::default(),
            speed_scale: SpeedScale(1.0),
            acceleration_scale: AccelerationScale(1.0),
            damping_scale: DampingScale(1.0),
            gravity_scale: GravityScale(1.0),
            jump_height_scale: JumpHeightScale(1.0),
        }
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct SpeedScale(f32);

#[derive(Component)]
struct AccelerationScale(f32);

#[derive(Component)]
struct DampingScale(f32);

#[derive(Component)]
struct GravityScale(f32);

#[derive(Component)]
struct JumpHeightScale(f32);

#[derive(Component, Debug)]
struct PlayerHistory {
    input_on_ground_change: Vec3,
    velocity_on_ground_change: Vec3,
    forward_on_ground_change: Vec3,
    previous_ground_state: GroundState,
}

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
    damping: f32,
    gravity: f32,
    jump_height: f32,
}

const BASE_SPEED: f32 = 15.0;
const BASE_ACCELERATION: f32 = BASE_SPEED * 4.0;
const BASE_DAMPING: f32 = BASE_SPEED * 0.4;
const BASE_GRAVITY: f32 = 9.81;
const BASE_JUMP_HEIGHT: f32 = 2.0;

impl GroundState {
    fn scalars(&self) -> Scalars {
        match self {
            GroundState::None => Scalars {
                speed: 1.0,
                acceleration: 0.2,
                damping: 0.2,
                gravity: 1.0,
                jump_height: 0.0,
            },
            GroundState::Normal => Scalars {
                speed: 1.0,
                acceleration: 1.0,
                damping: 1.0,
                gravity: 1.0,
                jump_height: 1.0,
            },
            GroundState::Slippery => Scalars {
                speed: 1.5,
                acceleration: 0.1,
                damping: 0.01,
                gravity: 1.0,
                jump_height: 0.0,
            },
            GroundState::Forward => Scalars {
                speed: 1.0,
                acceleration: 1.0,
                damping: 0.0,
                gravity: 1.0,
                jump_height: 0.0,
            },
        }
    }
}

fn set_ground_state(
    mut player_q: Query<(&mut GroundState, &mut PlayerHistory, &Transform), With<Player>>,
    platforms: Res<Platforms>,
) {
    let (mut ground_state, mut player_history, transform) = player_q.single_mut();
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
        player_history.previous_ground_state = *ground_state;
        *ground_state = new_ground_state;
    }
}

fn on_ground_change(
    mut player_q: Query<
        (
            &GroundState,
            &Velocity,
            &mut PlayerHistory,
            &mut SpeedScale,
            &mut AccelerationScale,
            &mut DampingScale,
            &mut GravityScale,
            &mut JumpHeightScale,
        ),
        (Changed<GroundState>, With<Player>),
    >,
    actor_q: Query<&Transform, With<Actor>>,
    input: Res<InputMovement>,
) {
    if let Ok((
        ground_state,
        velocity,
        mut player_history,
        mut speed_scale,
        mut acceleration_scale,
        mut damping_scale,
        mut gravity_scale,
        mut jump_height_scale,
    )) = player_q.get_single_mut()
    {
        player_history.input_on_ground_change = input.x0z();
        player_history.velocity_on_ground_change = velocity.0;
        player_history.forward_on_ground_change = actor_q.single().forward();

        let scalars = ground_state.scalars();
        speed_scale.0 = scalars.speed;
        acceleration_scale.0 = scalars.acceleration;
        damping_scale.0 = scalars.damping;
        gravity_scale.0 = scalars.gravity;
        jump_height_scale.0 = scalars.jump_height;
    }
}

fn movement(
    mut player_q: Query<
        (
            &mut Velocity,
            &GroundState,
            &PlayerHistory,
            &SpeedScale,
            &AccelerationScale,
        ),
        With<Player>,
    >,
    actor_q: Query<&Transform, (With<Actor>, Without<Player>)>,
    input: Res<InputMovement>,
    tick: Res<PhysicsTick>,
) {
    let (mut velocity, ground_state, player_history, speed_scale, acceleration_scale) =
        player_q.single_mut();

    let dt = tick.delta();
    let speed = BASE_SPEED * speed_scale.0;
    let acceleration = BASE_ACCELERATION * acceleration_scale.0 * dt;

    match ground_state {
        GroundState::Forward => {
            let forward_speed = player_history
                .velocity_on_ground_change
                .x0z()
                .length()
                .max(1.0);
            velocity.move_towards_xz(actor_q.single().forward() * forward_speed, acceleration);
        }
        _ => {
            if !input.is_zero() {
                velocity.move_towards_xz(input.x0z() * speed, acceleration);
            }
        }
    }
}

fn rotation(
    mut actor_q: Query<&mut Transform, With<Actor>>,
    input: Res<InputMovement>,
    time: Res<Time>,
) {
    if input.is_zero() {
        return;
    }

    const ROTATION_SPEED: f32 = 15.0;

    let mut transform = actor_q.single_mut();
    transform.rotation = Quat::slerp(
        transform.rotation,
        Quat::from_look(input.x0z(), Vec3::Y),
        ROTATION_SPEED * time.delta_seconds(),
    );
}

fn jump(
    mut player_q: Query<(&mut Impulse, &GravityScale, &JumpHeightScale), With<Player>>,
    input_action: Res<InputAction>,
) {
    if let InputAction::Jump = *input_action {
        let (mut impulse, gravity_scale, jump_height_scale) = player_q.single_mut();

        impulse.y += f32::sqrt(
            2.0 * BASE_GRAVITY * gravity_scale.0 * BASE_JUMP_HEIGHT * jump_height_scale.0,
        );
    }
}

fn apply_physics_scalars(
    mut player_q: Query<(&mut Damping, &mut Gravity, &DampingScale, &GravityScale), With<Player>>,
) {
    let (mut damping, mut gravity, damping_scale, gravity_scale) = player_q.single_mut();

    damping.0 = BASE_DAMPING * damping_scale.0;
    gravity.0 = BASE_GRAVITY * gravity_scale.0;
}
