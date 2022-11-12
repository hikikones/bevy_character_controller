use bevy::{ecs::schedule::ShouldRun, prelude::*};
use bevy_rapier3d::prelude::*;

use bevy_extensions::*;
use bootstrap::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(BootstrapPlugin)
        .add_plugin(PhysicsPlugin)
        .add_startup_system(setup)
        .add_system_set_to_stage(
            CoreStage::Update,
            SystemSet::new()
                .with_system(movement)
                .with_system(rotation)
                .with_system(jump),
        )
        .run();
}

struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RapierConfiguration {
            timestep_mode: TimestepMode::Fixed {
                dt: TICK,
                // time_scale: 1.0,
                substeps: 1,
            },
            // force_update_from_transform_changes: false,
            ..Default::default()
        })
        .insert_resource(FixedTime::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default().with_default_system_setup(false))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_system_to_stage(CoreStage::Last, fixed_time);

        app.add_stage_after(
            CoreStage::Update,
            PhysicsStages::SyncBackend,
            SystemStage::parallel()
                // .with_run_criteria(fixed_run_criteria)
                .with_system_set(RapierPhysicsPlugin::<NoUserData>::get_systems(
                    PhysicsStages::SyncBackend,
                )),
        );

        app.add_stage_after(
            PhysicsStages::SyncBackend,
            PhysicsStages::StepSimulation,
            SystemStage::parallel()
                .with_run_criteria(fixed_run_criteria)
                .with_system_set(RapierPhysicsPlugin::<NoUserData>::get_systems(
                    PhysicsStages::StepSimulation,
                )),
        );

        app.add_stage_after(
            PhysicsStages::StepSimulation,
            PhysicsStages::Writeback,
            SystemStage::parallel()
                .with_run_criteria(fixed_run_criteria)
                // .with_system_set(RapierPhysicsPlugin::<NoUserData>::get_systems(
                //     PhysicsStages::Writeback,
                // )),
                .with_system(sync),
        );

        // app.add_stage_after(
        //     PhysicsStages::StepSimulation,
        //     PhysicsStages::Writeback,
        //     SystemStage::parallel()
        //         // .with_run_criteria(fixed_run_criteria)
        //         .with_system_set(RapierPhysicsPlugin::<NoUserData>::get_systems(
        //             PhysicsStages::Writeback,
        //         )),
        // );

        // NOTE: we run sync_removals at the end of the frame, too, in order to make sure we don’t miss any `RemovedComponents`.
        app.add_stage_before(
            CoreStage::Last,
            PhysicsStages::DetectDespawn,
            SystemStage::parallel()
                // .with_run_criteria(fixed_run_criteria)
                .with_system_set(RapierPhysicsPlugin::<NoUserData>::get_systems(
                    PhysicsStages::DetectDespawn,
                )),
        );
    }
}

const TICK: f32 = 1.0 / 10.0;

#[derive(Default)]
struct FixedTime(f32);

fn fixed_run_criteria(fixed_time: Res<FixedTime>) -> ShouldRun {
    match fixed_time.0 >= TICK {
        true => ShouldRun::Yes,
        false => ShouldRun::No,
    }
}

fn fixed_time(mut fixed_time: ResMut<FixedTime>, time: Res<Time>) {
    if fixed_time.0 >= TICK {
        fixed_time.0 -= TICK;
    }

    fixed_time.0 += time.delta_seconds();
}

fn setup(
    player_q: Query<Entity, With<Player>>,
    platform_q: Query<(Entity, &Platform)>,
    mut commands: Commands,
) {
    // Player
    commands.entity(player_q.single()).insert_bundle((
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

    // Platforms
    for (entity, platform) in platform_q.iter() {
        let friction = match platform {
            Platform::Ground => 1.0,
            Platform::Ice => 0.0,
        };
        commands.entity(entity).insert_bundle((
            Collider::cuboid(0.5, 0.5, 0.5),
            Friction::coefficient(friction),
        ));
    }
}

fn sync(mut q: Query<(Entity, &mut Transform), With<Player>>, ctx: Res<RapierContext>) {
    let (entity, mut transform) = q.single_mut();
    let handle = ctx.entity2body().get(&entity).unwrap();
    let rb = ctx.bodies.get(*handle).unwrap();
    let pos = rb.position();
    transform.translation = pos.translation.into();
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
    let max_delta = MAX_ACCELERATION * TICK;

    velocity.linvel = velocity
        .linvel
        .move_towards(target, max_delta)
        .x_z(velocity.linvel.y);

    // dbg!(velocity.linvel);
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
