use bevy::prelude::*;

use bevy_actions::*;
use bevy_bootstrap::*;
use bevy_extensions::*;
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
            SystemSet::new().with_system(rotation).with_system(jump),
        )
        .add_system_set_to_stage(PhysicsStage::Update, SystemSet::new().with_system(movement))
        .add_system_set_to_stage(
            PhysicsStage::PreUpdate,
            SystemSet::new()
                .with_system(set_ground_state)
                .with_system(on_ground_change.after(set_ground_state)),
        )
        .run();
}

const BASE_SPEED: f32 = 10.0;
const BASE_ACCELERATION: f32 = BASE_SPEED * 3.0;
const BASE_RESISTANCE: f32 = 0.4;
const BASE_JUMP_HEIGHT: f32 = 3.0;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct SpeedScale(f32);

#[derive(Component)]
struct AccelerationScale(f32);

#[derive(Component)]
struct DragScale(f32);

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
    speed_scale: SpeedScale,
    acceleration_scale: AccelerationScale,
    resistance_scale: DragScale,
    jump_height_scale: JumpHeightScale,
    gravity_scale: GravityScale,
    ground_state: GroundState,
}

fn setup(mut commands: Commands) {
    // Player
    let player = commands
        .spawn((
            TransformBundle::default(),
            PlayerBundle {
                marker: Player,
                speed_scale: SpeedScale(1.0),
                acceleration_scale: AccelerationScale(1.0),
                resistance_scale: DragScale(1.0),
                jump_height_scale: JumpHeightScale(1.0),
                gravity_scale: GravityScale(1.0),
                ground_state: GroundState::Normal,
            },
            RigidBody::Dynamic,
            Collider::capsule((Vec3::Y * 0.5).into(), (Vec3::Y * 1.5).into(), 0.5),
            CollisionGroups::from(PhysicsLayer::PLAYER),
            Friction::coefficient(0.0),
            Restitution::coefficient(0.0),
            // Damping::default(),
            // ColliderMassProperties::default(),
            Velocity::default(),
            // ExternalForce::default(),
            ExternalImpulse::default(),
            Ccd::enabled(),
            Sleeping::disabled(),
            LockedAxes::ROTATION_LOCKED,
        ))
        .id();

    // Actor
    let actor = commands.spawn_actor(ActorConfig::default());
    commands.entity(actor).insert(PhysicsInterpolation {
        target: player,
        translate: true,
        rotate: false,
    });

    // Camera follow
    commands.camera_follow(actor);
}

fn movement(
    mut player_q: Query<(&mut Velocity, &SpeedScale, &AccelerationScale, &DragScale), With<Player>>,
    input: Res<InputMovement>,
    tick: Res<PhysicsTick>,
) {
    let (mut velocity, speed_scale, acceleration_scale, drag_scale) = player_q.single_mut();

    if input.is_zero() {
        let drag_scalar = 1.0 - BASE_RESISTANCE * drag_scale.0;
        let drag_velocity = velocity.linvel * drag_scalar;
        velocity.linvel = drag_velocity.x_z(velocity.linvel.y);
        return;
    }

    let direction = input.x0z();
    let current_velocity = velocity.linvel.x0z();
    let target_velocity = direction * BASE_SPEED * speed_scale.0;
    let max_delta = BASE_ACCELERATION * acceleration_scale.0 * tick.rate();

    velocity.linvel = current_velocity
        .move_towards(target_velocity, max_delta)
        .x_z(velocity.linvel.y);
}

fn rotation(
    mut player_q: Query<&mut Transform, With<Actor>>,
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
    mut player_q: Query<(&mut ExternalImpulse, &GravityScale, &JumpHeightScale), With<Player>>,
    input_action: Res<InputAction>,
) {
    if let InputAction::Jump = *input_action {
        let (mut impulse, gravity_scale, jump_height_scale) = player_q.single_mut();
        impulse.impulse = Vec3::Y
            * f32::sqrt(2.0 * 9.81 * gravity_scale.0 * BASE_JUMP_HEIGHT * jump_height_scale.0);
    }
}

fn set_ground_state(
    mut player_q: Query<(Entity, &mut GroundState, &Transform), With<Player>>,
    block_q: Query<&Block>,
    physics: Res<PhysicsContext>,
) {
    let (player, mut ground_state, transform) = player_q.single_mut();

    let ray_hit = physics.cast_ray(
        transform.translation + Vec3::Y * 0.1,
        -Vec3::Y,
        0.2,
        true,
        QueryFilter {
            exclude_rigid_body: Some(player),
            ..Default::default()
        },
    );

    let state = if let Some((hit_entity, _)) = ray_hit {
        if let Ok(block) = block_q.get(hit_entity) {
            match block {
                Block::Ice => GroundState::Slippery,
                _ => GroundState::Normal,
            }
        } else {
            GroundState::Normal
        }
    } else {
        GroundState::None
    };

    if *ground_state != state {
        *ground_state = state;
    }
}

fn on_ground_change(
    mut player_q: Query<
        (
            &GroundState,
            &mut AccelerationScale,
            &mut DragScale,
            &mut GravityScale,
        ),
        (Changed<GroundState>, With<Player>),
    >,
) {
    if let Ok((ground_state, mut acceleration_scale, mut drag_scale, mut gravity_scale)) =
        player_q.get_single_mut()
    {
        match ground_state {
            GroundState::None => {
                acceleration_scale.0 = 0.5;
                drag_scale.0 = 0.0;
                gravity_scale.0 = 2.0;
            }
            GroundState::Normal => {
                acceleration_scale.0 = 1.0;
                drag_scale.0 = 1.0;
                gravity_scale.0 = 2.0;
            }
            GroundState::Slippery => {
                acceleration_scale.0 = 0.2;
                drag_scale.0 = 0.01;
                gravity_scale.0 = 2.0;
            }
        }
    }
}
