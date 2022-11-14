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
            SystemSet::new().with_system(lerp_set),
        )
        .run();
}

#[derive(Component)]
struct Player;

fn setup(platform_q: Query<(Entity, &Platform)>, mut commands: Commands) {
    // Player
    commands
        .spawn_bundle(TransformBundle::default())
        .insert_bundle((
            Player,
            RigidBody::Dynamic,
            Collider::capsule((Vec3::Y * 0.5).into(), (Vec3::Y * 1.5).into(), 0.5),
            CollisionGroups {
                memberships: PhysicsLayer::PLAYER.into(),
                filters: PhysicsLayer::all().into(),
            },
            // Friction {
            //     coefficient: 0.0,
            //     combine_rule: CoefficientCombineRule::Average,
            // },
            // Restitution::default(),
            Damping::default(),
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
            Platform::Ground => 1.0,
            Platform::Ice => 0.0,
        };
        commands.entity(entity).insert_bundle((
            Collider::cuboid(0.5, 0.5, 0.5),
            Friction::coefficient(friction),
            CollisionGroups {
                memberships: PhysicsLayer::PLATFORM.into(),
                filters: PhysicsLayer::all().into(),
            },
        ));
    }
}

const MAX_SPEED: f32 = 10.0;
const MAX_ACCELERATION: f32 = MAX_SPEED * 2.0;
const ROTATION_SPEED: f32 = MAX_SPEED * 1.5;
const JUMP_HEIGHT: f32 = 2.0;

fn movement(
    mut player_q: Query<&mut Velocity, With<Player>>,
    input: Res<InputMovement>,
    tick: Res<SimulationTick>,
) {
    let mut velocity = player_q.single_mut();

    let input = input.x0z();
    let target = input * MAX_SPEED;
    let max_delta = MAX_ACCELERATION * tick.rate();

    velocity.linvel = velocity
        .linvel
        .move_towards(target, max_delta)
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

    let mut transform = player_q.single_mut();
    transform.rotation = Quat::slerp(
        transform.rotation,
        Quat::from_look(input.x0z(), Vec3::Y),
        ROTATION_SPEED * time.delta_seconds(),
    );
}

fn jump(mut player_q: Query<&mut ExternalImpulse, With<Player>>, input_action: Res<InputAction>) {
    if let InputAction::Jump = *input_action {
        let force = Vec3::Y * f32::sqrt(2.0 * 9.81 * JUMP_HEIGHT);
        player_q.single_mut().impulse = force;
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
