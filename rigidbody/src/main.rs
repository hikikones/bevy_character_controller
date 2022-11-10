use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use bevy_extensions::*;
use bootstrap::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            ..Default::default()
        })
        .insert_resource(RapierConfiguration {
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(BootstrapPlugin)
        .add_startup_system(setup)
        .add_system_set_to_stage(
            CoreStage::Update,
            SystemSet::new()
                .with_system(movement)
                .with_system(rotation)
                .with_system(jump),
        )
        .add_system_to_stage(CoreStage::PreUpdate, bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands, assets: Res<MyAssets>) {
    // Ground
    commands
        .spawn_bundle(PbrBundle {
            mesh: assets.mesh(MeshName::Cube),
            material: assets.material(MaterialName::DarkGray),
            transform: Transform {
                translation: -Vec3::Y * 0.5,
                scale: Vec3::new(500.0, 1.0, 500.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle((
            Collider::cuboid(250.0, 0.5, 250.0),
            Friction::coefficient(1.0),
        ));

    // Player
    let player = commands.spawn_actor(ActorConfig::default());
    commands.entity(player).insert_bundle((
        Player,
        RigidBody::Dynamic,
        Collider::capsule((Vec3::Y * 0.5).into(), (Vec3::Y * 1.5).into(), 0.5),
        CollisionGroups::default(),
        Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Average,
        },
        Restitution::default(),
        Damping::default(),
        ColliderMassProperties::default(),
        GravityScale::default(),
        Velocity::default(),
        ExternalForce::default(),
        ExternalImpulse::default(),
        Ccd::default(),
        Sleeping::default(),
        LockedAxes::ROTATION_LOCKED,
    ));
}

const MAX_SPEED: f32 = 10.0;
const MAX_ACCELERATION: f32 = MAX_SPEED * 2.0;
const ROTATION_SPEED: f32 = MAX_SPEED * 1.5;
const JUMP_HEIGHT: f32 = 2.0;

fn movement(
    mut player_q: Query<&mut Velocity, With<Player>>,
    input: Res<InputMovement>,
    time: Res<Time>,
) {
    let mut velocity = player_q.single_mut();

    let input = input.x0z();
    let dt = time.delta_seconds();
    let target = input * MAX_SPEED;
    let max_delta = MAX_ACCELERATION * dt;

    velocity.linvel = velocity
        .linvel
        .move_towards(target, max_delta)
        .x_z(velocity.linvel.y);
}

fn rotation(
    mut player_q: Query<&mut Transform, With<Player>>,
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
