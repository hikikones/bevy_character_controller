use bevy::prelude::*;

use bevy_bootstrap::{
    ActorConfig, CameraFollowExt, InputAction, InputMovement, MaterialName, MeshName, MyAssets,
    SpawnActorExt,
};
use bevy_extensions::*;

mod board;

use board::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_bootstrap::AssetsPlugin)
        .add_plugin(bevy_bootstrap::CameraPlugin)
        .add_plugin(bevy_bootstrap::InputPlugin)
        .add_plugin(board::BoardPlugin)
        .add_startup_system(setup)
        .add_system_set_to_stage(
            CoreStage::PreUpdate,
            SystemSet::new().with_system(bevy::window::close_on_esc),
        )
        .add_system_set_to_stage(
            CoreStage::Update,
            SystemSet::new()
                .with_system(movement)
                .with_system(rotation)
                .with_system(jump),
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .before(bevy::transform::transform_propagate_system)
                .with_system(apply_velocity)
                .with_system(set_ground_state.before(apply_velocity))
                .with_system(on_ground_change.after(set_ground_state)),
        )
        .run();
}

const BASE_GRAVITY: f32 = 9.81;

const BASE_SPEED: f32 = 8.0;
const BASE_ACCELERATION: f32 = BASE_SPEED * 0.5;
const BASE_DRAG: f32 = 0.25;
const BASE_JUMP_HEIGHT: f32 = 2.0;

#[derive(Component, Default)]
struct Player;

#[derive(Component)]
struct Velocity(Vec3);

#[derive(Component, Debug, PartialEq, Eq)]
enum GroundState {
    None,
    Normal,
    Slippery,
}

#[derive(Component)]
struct SpeedScale(f32);

#[derive(Component)]
struct AccelerationScale(f32);

#[derive(Component)]
struct DragScale(f32);

#[derive(Component)]
struct JumpHeightScale(f32);

#[derive(Component)]
struct GravityScale(f32);

#[derive(Bundle)]
struct PlayerBundle {
    marker: Player,
    velocity: Velocity,
    ground_state: GroundState,
    #[bundle]
    scales: ScaleBundle,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            marker: Player,
            velocity: Velocity(Vec3::ZERO),
            ground_state: GroundState::Normal,
            scales: ScaleBundle::default(),
        }
    }
}

#[derive(Bundle)]
struct ScaleBundle {
    speed: SpeedScale,
    acceleration: AccelerationScale,
    drag: DragScale,
    jump_height: JumpHeightScale,
    gravity: GravityScale,
}

impl Default for ScaleBundle {
    fn default() -> Self {
        Self {
            speed: SpeedScale(1.0),
            acceleration: AccelerationScale(1.0),
            drag: DragScale(1.0),
            jump_height: JumpHeightScale(1.0),
            gravity: GravityScale(1.0),
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
    mut player_q: Query<(&mut Velocity, &SpeedScale, &AccelerationScale), With<Player>>,
    input: Res<InputMovement>,
) {
    if input.is_zero() {
        return;
    }

    let (mut velocity, speed_scale, acceleration_scale) = player_q.single_mut();

    let direction = input.x0z();
    let current_velocity = velocity.0.x0z();
    let target_velocity = direction * BASE_SPEED * speed_scale.0;
    let max_delta = BASE_ACCELERATION * acceleration_scale.0;

    velocity.0 = current_velocity
        .move_towards(target_velocity, max_delta)
        .x_z(velocity.0.y);
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
    }
}

fn apply_velocity(
    mut player_q: Query<(&mut Transform, &mut Velocity, &DragScale), With<Player>>,
    time: Res<Time>,
) {
    let (mut transform, mut velocity, drag_scale) = player_q.single_mut();

    transform.translation += velocity.0 * time.delta_seconds();

    let drag = velocity.0.x0z() * (1.0 - (BASE_DRAG * drag_scale.0));
    let y = velocity.0.y - 9.81 * time.delta_seconds();
    velocity.0 = drag.x_z(y);

    if transform.translation.y < 0.0 {
        transform.translation.y = 0.0;
        velocity.0.y = 0.0;
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
            &mut DragScale,
            &mut GravityScale,
        ),
        (Changed<GroundState>, With<Player>),
    >,
) {
    if let Ok((
        ground_state,
        mut speed_scale,
        mut acceleration_scale,
        mut drag_scale,
        mut gravity_scale,
    )) = player_q.get_single_mut()
    {
        match ground_state {
            GroundState::None => {
                speed_scale.0 = 1.0;
                acceleration_scale.0 = 0.05;
                drag_scale.0 = 0.05;
                gravity_scale.0 = 1.0;
            }
            GroundState::Normal => {
                speed_scale.0 = 1.0;
                acceleration_scale.0 = 1.0;
                drag_scale.0 = 1.0;
                gravity_scale.0 = 1.0;
            }
            GroundState::Slippery => {
                speed_scale.0 = 2.0;
                acceleration_scale.0 = 0.01;
                drag_scale.0 = 0.01;
                gravity_scale.0 = 0.0;
            }
        }
    }
}
