use bevy::prelude::*;
use bevy_third_person_camera::ThirdPersonCameraTarget;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Speed(f32);

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
            material: materials.add(Color::BLUE),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Player,
        ThirdPersonCameraTarget,
        Speed(2.5),
    );

    commands.spawn(player);
}

fn player_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut player_q: Query<(&mut Transform, &Speed), With<Player>>,
    cam_q: Query<&Transform, (With<Camera3d>, Without<Player>)>,
) {
    for (mut player_transform, player_speed) in player_q.iter_mut() {
        let cam = match cam_q.get_single() {
            Ok(c) => c,
            Err(e) => Err(format!("Error retrieving camera: {}", e)).unwrap(),
        };

        let mut direction: Vec3 = Vec3::ZERO;

        // forward
        if keys.pressed(KeyCode::KeyW) {
            let forward: Vec3 = cam.forward().into();
            direction += forward;
        }

        // back
        if keys.pressed(KeyCode::KeyS) {
            let back: Vec3 = cam.back().into();
            direction += back;
        }

        // left
        if keys.pressed(KeyCode::KeyA) {
            let left: Vec3 = cam.left().into();
            direction += left;
        }

        // right
        if keys.pressed(KeyCode::KeyD) {
            let right: Vec3 = cam.right().into();
            direction += right;
        }

        direction.y = 0.0;
        let movement = direction.normalize_or_zero() * player_speed.0 * time.delta_seconds();
        player_transform.translation += movement;
    }
}
