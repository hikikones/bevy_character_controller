use bevy::prelude::*;

use bevy_bootstrap::{
    ActorConfig, CameraFollowExt, InputAction, InputMovement, MaterialName, MeshName, MyAssets,
    SpawnActorExt,
};
use bevy_extensions::*;

mod board;
mod physics;

use board::*;
use physics::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_bootstrap::AssetsPlugin)
        .add_plugin(bevy_bootstrap::CameraPlugin)
        .add_plugin(bevy_bootstrap::InputPlugin)
        .add_plugin(board::BoardPlugin)
        .add_plugin(physics::PhysicsPlugin)
        .add_startup_system(setup)
        .add_system_set_to_stage(
            CoreStage::PreUpdate,
            SystemSet::new()
                .with_system(set_ground_state)
                .with_system(on_ground_change.after(set_ground_state))
                .with_system(bevy::window::close_on_esc),
        )
        .add_system_set_to_stage(
            CoreStage::Update,
            SystemSet::new()
                .with_system(movement)
                .with_system(rotation)
                .with_system(jump.after(movement))
                .with_system(apply_physics_scalars.after(jump)),
        )
        .run();
}

const BASE_SPEED: f32 = 2.5;
const BASE_ACCELERATION: f32 = BASE_SPEED * 0.5;
const BASE_FRICTION: f32 = 0.25;
const BASE_GRAVITY: f32 = 9.81;
const BASE_JUMP_HEIGHT: f32 = 2.0;

#[derive(Component)]
struct Player;

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
}

#[derive(Bundle)]
struct PlayerBundle {
    marker: Player,

    #[bundle]
    physics_bundle: PhysicsBundle,

    speed_scale: SpeedScale,
    acceleration_scale: AccelerationScale,
    friction_scale: FrictionScale,
    gravity_scale: GravityScale,
    jump_height_scale: JumpHeightScale,
    ground_state: GroundState,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            marker: Player,
            physics_bundle: PhysicsBundle::default(),
            speed_scale: SpeedScale(1.0),
            acceleration_scale: AccelerationScale(1.0),
            friction_scale: FrictionScale(1.0),
            gravity_scale: GravityScale(1.0),
            jump_height_scale: JumpHeightScale(1.0),
            ground_state: GroundState::Normal,
        }
    }
}

fn setup(mut commands: Commands, assets: Res<MyAssets>) {
    // Light
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 25000.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(10.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    // Floor
    commands.spawn_bundle(PbrBundle {
        mesh: assets.mesh(MeshName::Cube),
        material: assets.material(MaterialName::DarkGray),
        transform: Transform {
            translation: -Vec3::Y * 0.6,
            scale: Vec3::new(500.0, 1.0, 500.0),
            ..Default::default()
        },
        ..Default::default()
    });

    // Player
    let player = commands.spawn_actor(ActorConfig::default());
    commands
        .entity(player)
        .insert_bundle(PlayerBundle::default());

    // Camera follow
    commands.camera_follow(player);
}

fn movement(
    mut player_q: Query<(&mut Velocity, &SpeedScale), With<Player>>,
    input: Res<InputMovement>,
) {
    let (mut velocity, speed_scale) = player_q.single_mut();
    velocity.linear = input.x0z() * BASE_SPEED * speed_scale.0;
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
        velocity.linear.y += f32::sqrt(
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
            &GroundState,
            &mut SpeedScale,
            &mut AccelerationScale,
            &mut FrictionScale,
            &mut GravityScale,
        ),
        (Changed<GroundState>, With<Player>),
    >,
) {
    if let Ok((
        ground_state,
        mut speed_scale,
        mut acceleration_scale,
        mut friction_scale,
        mut gravity_scale,
    )) = player_q.get_single_mut()
    {
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
        }
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
