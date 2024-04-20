use bevy::prelude::*;
use bevy_third_person_camera::ThirdPersonCameraTarget;
use bevy::window::{CursorGrabMode, PrimaryWindow};
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .init_resource::<MovementSettings>()
            .add_systems(Update, player_movement);
    }
}

/// Mouse sensitivity and movement speed
#[derive(Resource)]
pub struct MovementSettings {
    pub sensitivity: f32,
    pub speed: f32,
    pub accel: f32,
    pub decel: f32,
    pub accel_max: f32,
    pub decel_min: f32,
    pub velocity: Vec3,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.0005,
            speed: 12.,
            accel: 0.0,
            decel: 0.0,
            accel_max: 1.0,
            decel_min: -1.0,
            velocity: Vec3::ZERO,
        }
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Speed(f32);

fn spawn_player(
    mut commands: Commands,
    assets: Res<AssetServer>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
 ) {
     let player = (
//         PbrBundle {
//             mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
//             material: materials.add(Color::BLUE.into()),
//             transform: Transform::from_xyz(0.0, 0.5, 0.0),
//             ..default()
//         },
        SceneBundle {
            scene: assets.load("./plane/xwing.gltf#Scene0"),
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
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut player_q: Query<(&mut Transform, &Speed), With<Player>>,
    mut settings: ResMut<MovementSettings>,
    cam_q: Query<&Transform, (With<Camera3d>, Without<Player>)>,
) {
    if let Ok(window) = primary_window.get_single() {
        for (mut player_transform, player_speed) in player_q.iter_mut() {
            let cam = match cam_q.get_single() {
                Ok(c) => c,
                Err(e) => Err(format!("Error retrieving camera: {}", e)).unwrap(),
            };
            let local_z = player_transform.local_z();
            let forward = -Vec3::new(local_z.x, local_z.y, local_z.z);
            let window_scale = window.height().min(window.width());
            let (mut yaw, mut pitch, mut turn) = player_transform.rotation.to_euler(EulerRot::YXZ);
            let mut direction: Vec3 = Vec3::ZERO;

            // forward
            if keys.pressed(KeyCode::KeyW) {
                pitch += ((settings.sensitivity*3.0) * window_scale).to_radians();
                player_transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
            }

            // back
            if keys.pressed(KeyCode::KeyS) {
                pitch -= ((settings.sensitivity*3.0) * window_scale).to_radians();
                player_transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
            }

            if keys.pressed(KeyCode::KeyQ) {
                turn += ((settings.sensitivity*3.0) * window_scale).to_radians();
                player_transform.rotation = Quat::from_axis_angle(Vec3::X, pitch) * Quat::from_axis_angle(Vec3::Z, turn);
            }

            if keys.pressed(KeyCode::KeyE) {
                turn -= ((settings.sensitivity*3.0) * window_scale).to_radians();
                player_transform.rotation = Quat::from_axis_angle(Vec3::X, pitch) * Quat::from_axis_angle(Vec3::Z, turn);
            }  

            // left
            if keys.pressed(KeyCode::KeyA) {
                yaw += ((settings.sensitivity*3.0) * window_scale).to_radians();
                player_transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
            }

            // right
            if keys.pressed(KeyCode::KeyD) {
                yaw -= ((settings.sensitivity*3.0) * window_scale).to_radians();
                player_transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
            }

            if keys.pressed(KeyCode::Space){
                settings.velocity += forward;
                settings.accel = settings.accel + 0.001;
                settings.decel = 1.0;
                if settings.accel <= 0.0{
                    settings.accel = f32::max(settings.decel_min,settings.accel);
                }
                else{
                    settings.accel = f32::min(settings.accel_max,settings.accel);
                }
            }
            if keys.pressed(KeyCode::ShiftLeft) {
                settings.velocity -= forward;
                settings.decel = -1.0;
                settings.accel = settings.accel- 0.001;
                settings.accel= f32::max(settings.decel_min,settings.accel);
            }
            direction.y = 0.0;
            let movement = direction.normalize_or_zero() * player_speed.0 * time.delta_seconds();
            player_transform.translation += movement;

            
            settings.velocity = settings.velocity.normalize_or_zero();

            if(settings.accel > 0.0){
                settings.accel = settings.accel-0.00035;
                settings.velocity = settings.velocity.normalize_or_zero();
                player_transform.translation += settings.velocity * time.delta_seconds() * settings.speed * (settings.accel) * (settings.decel)
            }
            else if (settings.accel < 0.0){
                settings.accel = settings.accel+0.00035;
                settings.velocity = settings.velocity.normalize_or_zero();
                player_transform.translation += settings.velocity * time.delta_seconds() * settings.speed * (settings.accel) * (settings.decel)
            }
        }
        
    }

}