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
        .add_physics_system_set(
            SystemSet::new()
                .with_system(set_ground_state)
                .with_system(on_ground_change.after(set_ground_state))
                .with_system(movement.after(on_ground_change))
                .with_system(apply_physics_scalars.after(on_ground_change)),
        )
        .run();
}

#[derive(Bundle)]
struct PlayerBundle {
    marker: Player,
    ground_state: GroundState,
    speed_scale: SpeedScale,
    acceleration_scale: AccelerationScale,
    damping_scale: DampingScale,
    friction_scale: FrictionScale,
    gravity_scale: GravityScale,
    jump_height_scale: JumpHeightScale,
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
struct FrictionScale(f32);

#[derive(Component)]
struct JumpHeightScale(f32);

#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq)]
enum GroundState {
    None,
    #[default]
    Normal,
    Slippery,
}

struct Scalars {
    speed: f32,
    acceleration: f32,
    damping: f32,
    friction: f32,
    gravity: f32,
    jump_height: f32,
}

const BASE_SPEED: f32 = 10.0;
const BASE_ACCELERATION: f32 = BASE_SPEED * 3.0;
const BASE_DAMPING: f32 = 1.0;
const BASE_FRICTION: f32 = 1.0;
const BASE_JUMP_HEIGHT: f32 = 3.0;

impl GroundState {
    fn scalars(&self) -> Scalars {
        match self {
            GroundState::None => Scalars {
                speed: 1.0,
                acceleration: 0.2,
                damping: 0.1,
                friction: 0.0,
                gravity: 1.1,
                jump_height: 1.0,
            },
            GroundState::Normal => Scalars {
                speed: 1.0,
                acceleration: 1.0,
                damping: 2.0,
                friction: 0.5,
                gravity: 1.0,
                jump_height: 1.0,
            },
            GroundState::Slippery => Scalars {
                speed: 1.5,
                acceleration: 0.2,
                damping: 0.0,
                friction: 0.0,
                gravity: 1.0,
                jump_height: 1.0,
            },
        }
    }
}

fn setup(mut commands: Commands) {
    // Player
    let player = commands
        .spawn((
            TransformBundle::default(),
            PlayerBundle {
                marker: Player,
                ground_state: GroundState::Normal,
                speed_scale: SpeedScale(1.0),
                acceleration_scale: AccelerationScale(1.0),
                damping_scale: DampingScale(1.0),
                friction_scale: FrictionScale(1.0),
                gravity_scale: GravityScale(1.0),
                jump_height_scale: JumpHeightScale(1.0),
            },
            RigidBody::Dynamic,
            Collider::capsule((Vec3::Y * 0.5).into(), (Vec3::Y * 1.5).into(), 0.5),
            CollisionGroups::from(PhysicsLayer::PLAYER),
            Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Multiply,
            },
            Restitution {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Multiply,
            },
            Damping::default(),
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
            &mut SpeedScale,
            &mut AccelerationScale,
            &mut DampingScale,
            &mut FrictionScale,
            &mut GravityScale,
            &mut JumpHeightScale,
        ),
        (Changed<GroundState>, With<Player>),
    >,
) {
    if let Ok((
        ground_state,
        mut speed_scale,
        mut acceleration_scale,
        mut damping_scale,
        mut friction_scale,
        mut gravity_scale,
        mut jump_height_scale,
    )) = player_q.get_single_mut()
    {
        let scalars = ground_state.scalars();
        speed_scale.0 = scalars.speed;
        acceleration_scale.0 = scalars.acceleration;
        damping_scale.0 = scalars.damping;
        friction_scale.0 = scalars.friction;
        gravity_scale.0 = scalars.gravity;
        jump_height_scale.0 = scalars.jump_height;
    }
}

fn movement(
    mut player_q: Query<
        (&mut Velocity, &GroundState, &SpeedScale, &AccelerationScale),
        With<Player>,
    >,
    input: Res<InputMovement>,
    tick: Res<PhysicsTick>,
) {
    let (mut velocity, _ground_state, speed_scale, acceleration_scale) = player_q.single_mut();

    if input.is_zero() {
        return;
    }

    let direction = input.x0z();
    let current_velocity = velocity.linvel.x0z();
    let target_velocity = direction * BASE_SPEED * speed_scale.0;
    let max_delta = BASE_ACCELERATION * acceleration_scale.0 * tick.delta();

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
        impulse.impulse.y +=
            f32::sqrt(2.0 * 9.81 * gravity_scale.0 * BASE_JUMP_HEIGHT * jump_height_scale.0);
    }
}

fn apply_physics_scalars(
    mut player_q: Query<(&mut Damping, &mut Friction, &DampingScale, &FrictionScale), With<Player>>,
) {
    let (mut damping, mut friction, damping_scale, friction_scale) = player_q.single_mut();

    damping.linear_damping = BASE_DAMPING * damping_scale.0;
    friction.coefficient = BASE_FRICTION * friction_scale.0;
}
