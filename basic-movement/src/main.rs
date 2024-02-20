use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.build().set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Basic Movement"),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .run();
}

#[derive(Component)]
struct Player;

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
