//! This is a very simple project to test basic movement in a 3D environment with Bevy.

use bevy::prelude::*;

/// A simple tag to identify the player entity.
#[derive(Component)]
struct Player;

/// A tag to identify star entities.
#[derive(Component)]
struct Star {
    alive_timer: Timer,
}

/// The timer that we use to know when to spawn a new star.
#[derive(Resource)]
struct StarSpawnTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.build().set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Basic Movement"),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(StarSpawnTimer(Timer::from_seconds(
            2.,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player, spawn_star, despawn_stars))
        .run();
}

/// Initialise the environment and the objects in it.
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let camera_pos = Transform::from_xyz(0., 16., 22.).looking_at(Vec3::ZERO, Vec3::Y);

    // Camera
    commands.spawn(Camera3dBundle {
        transform: camera_pos,
        ..default()
    });

    // Plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(20., 20.)),
        material: materials.add(Color::rgb(0.3, 0.8, 0.3)),
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    });

    // Light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: camera_pos,
        ..default()
    });

    // Player
    commands.spawn((
        Player,
        PbrBundle {
            mesh: meshes.add(Capsule3d::new(0.5, 1.2).mesh()),
            material: materials.add(Color::rgb(0.8, 0.3, 0.3)),
            transform: Transform::from_xyz(0., 0.95, 0.),
            ..default()
        },
    ));
}

fn move_player(
    mut player_pos: Query<&mut Transform, With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    const MOVE_DISTANCE: f32 = 0.1;

    for mut position in &mut player_pos {
        if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
            position.translation -= MOVE_DISTANCE * Vec3::X;
        }

        if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
            position.translation += MOVE_DISTANCE * Vec3::X;
        }

        if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
            position.translation -= MOVE_DISTANCE * Vec3::Z;
        }

        if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) {
            position.translation += MOVE_DISTANCE * Vec3::Z;
        }

        position.translation.x = position.translation.x.clamp(-9., 9.);
        position.translation.z = position.translation.z.clamp(-9., 9.);
    }
}

/// Spawn a new star in a random place if the timer has finished.
fn spawn_star(
    time: Res<Time>,
    mut spawn_timer: ResMut<StarSpawnTimer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !spawn_timer.0.tick(time.delta()).finished() {
        return;
    }

    // TODO: Randomise this and dodge already existing stars and player
    let position = (0., 0.);

    // TODO: Use star model
    let transform = Transform::from_xyz(position.0, 1., position.1);
    commands.spawn((
        Star {
            alive_timer: Timer::from_seconds(3.5, TimerMode::Once),
        },
        PbrBundle {
            mesh: meshes.add(Sphere::new(0.4)),
            material: materials.add(Color::rgb(0.9, 0.75, 0.2)),
            transform,
            ..default()
        },
        PointLight {
            color: Color::rgb(0.9, 0.75, 0.2),
            ..default()
        },
    ));
}

/// Despawn all the old stars.
fn despawn_stars(time: Res<Time>, mut stars: Query<(&mut Star, Entity)>, mut commands: Commands) {
    for (mut star, entity) in &mut stars {
        if star.alive_timer.tick(time.delta()).finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
