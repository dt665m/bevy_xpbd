use bevy::prelude::*;
use bevy_xpbd_3d::{
    math::*, prelude::*, PhysicsSchedule, PhysicsStepSet, SubstepSchedule, SubstepSet,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_systems(Startup, setup)
        .add_systems(PhysicsSchedule, movement.before(PhysicsStepSet::BroadPhase))
        .run();
}

#[derive(Component)]
struct Player;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(8.0))),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(8.0, 0.005, 8.0),
    ));

    // Player
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 0.4,
                ..default()
            })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            ..default()
        },
        RigidBody::Kinematic,
        Position(Vector::Y * 1.0),
        Collider::capsule(1.0, 0.4),
        Player,
    ));

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-4.0, 6.5, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<
        (Entity, &mut LinearVelocity, &Position, &Rotation, &Collider),
        With<Player>,
    >,
    spatial_query: SpatialQuery,
) {
    for (entity, mut linear_velocity, pos, rot, collider) in &mut players {
        // Directional movement
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
            direction += Vec3::NEG_Z;
        }
        if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            direction += Vec3::NEG_X;
        }
        if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
            direction += Vec3::Z;
        }
        if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            direction += Vec3::X;
        }

        let mut hits = vec![];
        // Cast shape and get all hits
        spatial_query.shape_hits_callback(
            collider,
            pos.0,
            rot.0,
            direction,
            30.0,
            true,
            default(),
            |hit| {
                // Callback function
                hits.push(hit);
                (hits.len() as u32) < 5
            },
        );

        let mut just_hit = false;
        for hit in hits.iter() {
            just_hit = true;
            println!("Origin: {:?}", pos.0);
            println!("Dir: {:?}", direction);
            println!("Hit: {:?}", hit);
            println!("Entity is self: {:?}", hit.entity == entity);
        }

        if !just_hit && direction != Vec3::ZERO {
            println!("No Hit, Dir: {}", direction);
        }
    }
}
