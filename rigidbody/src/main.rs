use bevy::prelude::*;

use bevy_extensions::*;
use bootstrap::*;

mod layer;
mod simulation;

use layer::PhysicsLayer;
use simulation::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(BootstrapPlugin)
        .add_plugin(SimulationPlugin)
        .add_startup_system(setup)
        .add_system_set_to_stage(
            CoreStage::Update,
            SystemSet::new()
                .with_system(rotation)
                .with_system(jump)
                .with_system(lerp),
        )
        .add_system_set_to_stage(
            SimulationStage::Update,
            SystemSet::new().with_system(movement),
        )
        .add_system_set_to_stage(
            SimulationStage::PostUpdate,
            SystemSet::new()
                .with_system(set_ground)
                .with_system(on_ground_change.after(set_ground))
                .with_system(lerp_set),
        )
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Speed(f32);

#[derive(Component)]
struct Acceleration(f32);

#[derive(Component)]
struct Resistance(f32);

#[derive(Component)]
struct JumpHeight(f32);

#[derive(Component, Debug, PartialEq, Eq)]
enum GroundState {
    None,
    Normal,
    Slippery,
}

impl GroundState {
    const fn acceleration(&self) -> f32 {
        match self {
            GroundState::None => 10.0,
            GroundState::Normal => 20.0,
            GroundState::Slippery => 40.0,
        }
    }

    const fn resistance(&self) -> f32 {
        match self {
            GroundState::None => 1.0,
            GroundState::Normal => 1.0,
            GroundState::Slippery => 0.01,
        }
    }
}

#[derive(Bundle)]
struct PlayerBundle {
    marker: Player,
    speed: Speed,
    acceleration: Acceleration,
    resistance: Resistance,
    jump_height: JumpHeight,
    ground_state: GroundState,
}

fn setup(platform_q: Query<(Entity, &PlatformName)>, mut commands: Commands) {
    // Player
    commands
        .spawn()
        .insert_bundle(TransformBundle::default())
        .insert_bundle(PlayerBundle {
            marker: Player,
            speed: Speed(10.0),
            acceleration: Acceleration(20.0),
            resistance: Resistance(0.0),
            jump_height: JumpHeight(2.0),
            ground_state: GroundState::Normal,
        })
        .insert_bundle((
            GroundState::Normal,
            RigidBody::Dynamic,
            Collider::capsule((Vec3::Y * 0.5).into(), (Vec3::Y * 1.5).into(), 0.5),
            CollisionGroups {
                memberships: PhysicsLayer::PLAYER.into(),
                filters: PhysicsLayer::all().into(),
            },
            Friction::coefficient(0.0),
            // Restitution::default(),
            // Damping::default(),
            // ColliderMassProperties::default(),
            // GravityScale::default(),
            Velocity::default(),
            // ExternalForce::default(),
            ExternalImpulse::default(),
            Ccd::enabled(),
            Sleeping::disabled(),
            LockedAxes::ROTATION_LOCKED,
        ));

    // Actor
    let actor = commands.spawn_actor(ActorConfig::default());
    commands.entity(actor).insert(Lerp::default());

    // Platforms
    for (entity, platform) in platform_q.iter() {
        let friction = match platform {
            PlatformName::Ground => 1.0,
            PlatformName::Ice => 0.0,
        };
        commands.entity(entity).insert_bundle((
            Collider::cuboid(0.5, 0.5, 0.5),
            Friction::coefficient(0.0),
            CollisionGroups {
                memberships: PhysicsLayer::PLATFORM.into(),
                filters: PhysicsLayer::all().into(),
            },
        ));
    }
}

fn movement(
    mut player_q: Query<(&mut Velocity, &Speed, &Acceleration, &Resistance), With<Player>>,
    input: Res<InputMovement>,
    tick: Res<SimulationTick>,
) {
    if input.is_zero() {
        // return;
    }

    let (mut velocity, speed, acceleration, resistance) = player_q.single_mut();

    let direction = input.x0z();
    let current_velocity = velocity.linvel.x0z();
    let target_velocity = direction * speed.0;
    // let drag = current_velocity.normalize_or_zero() * resistance.0;
    let max_delta = acceleration.0 * tick.rate();

    // let actual_velocity =
    //     current_velocity.move_towards(target_velocity, max_delta) * (1.0 - resistance.0);
    let actual_velocity = current_velocity.move_towards(target_velocity, max_delta * resistance.0);
    velocity.linvel = actual_velocity.x_z(velocity.linvel.y);

    // velocity.linvel = current_velocity
    //     .move_towards(target_velocity, max_delta)
    //     .x_z(velocity.linvel.y);
}

fn resistance(
    mut player_q: Query<(&mut Velocity, &Speed, &Acceleration, &Resistance), With<Player>>,
    input: Res<InputMovement>,
    tick: Res<SimulationTick>,
) {
    if input.is_zero() {
        // return;
    }

    let (mut velocity, speed, acceleration, resistance) = player_q.single_mut();

    let direction = input.x0z();
    let current_velocity = velocity.linvel.x0z();
    let target_velocity = direction * speed.0;
    // let drag = current_velocity.normalize_or_zero() * resistance.0;
    let max_delta = acceleration.0 * tick.rate();

    // let new_velocity = current_velocity.move_towards(target_velocity, max_delta) - drag;

    // velocity.linvel = new_velocity.x_z(velocity.linvel.y);
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
    mut player_q: Query<(&mut ExternalImpulse, &JumpHeight), With<Player>>,
    input_action: Res<InputAction>,
) {
    if let InputAction::Jump = *input_action {
        let (mut impulse, jump_height) = player_q.single_mut();
        impulse.impulse = Vec3::Y * f32::sqrt(2.0 * 9.81 * jump_height.0);
    }
}

fn set_ground(
    mut player_q: Query<(&mut GroundState, &Transform), With<Player>>,
    platform_q: Query<&PlatformName>,
    physics: Res<PhysicsContext>,
) {
    let (mut ground_state, transform) = player_q.single_mut();

    let ray_hit = physics.cast_ray(
        transform.translation + Vec3::Y * 0.1,
        -Vec3::Y,
        0.2,
        true,
        PhysicsLayer::PLATFORM.into(),
    );

    let state = if let Some((platform_entity, _)) = ray_hit {
        match platform_q.get(platform_entity).unwrap() {
            PlatformName::Ground => GroundState::Normal,
            PlatformName::Ice => GroundState::Slippery,
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
        (&GroundState, &mut Acceleration, &mut Resistance),
        (Changed<GroundState>, With<Player>),
    >,
) {
    if let Ok((ground_state, mut acceleration, mut resistance)) = player_q.get_single_mut() {
        dbg!(ground_state);
        acceleration.0 = ground_state.acceleration();
        resistance.0 = ground_state.resistance();
    }
}

#[derive(Default, Component)]
struct Lerp(Vec3, Vec3);

fn lerp(mut actor_q: Query<(&mut Transform, &Lerp), With<Actor>>, tick: Res<SimulationTick>) {
    let (mut transform, lerp) = actor_q.single_mut();
    transform.translation = Vec3::lerp(lerp.0, lerp.1, tick.percent());
}

fn lerp_set(
    mut actor_q: Query<&mut Lerp, (With<Actor>, Without<Player>)>,
    player_q: Query<&Transform, With<Player>>,
) {
    let mut lerp = actor_q.single_mut();
    lerp.0 = lerp.1;
    lerp.1 = player_q.single().translation;
}
