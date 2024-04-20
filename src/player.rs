use bevy::{math::vec3, prelude::*};
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
    pub thrust_strength: f32,
    pub velocity: Vec3,
    pub thrust_force: Vec3,
    pub gravity_force: Vec3,
    pub mass: f32
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            velocity: Vec3::ZERO,
            thrust_force: Vec3::ZERO,
            gravity_force: Vec3::new(0., -9.81, 0.),
            mass: 1.,
            thrust_strength: 1000.,
        }
    }
}

#[derive(Component)]
pub struct Player;

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
    );

    commands.spawn(player);
}

fn player_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut player_q: Query<&mut Transform, With<Player>>,
    mut settings: ResMut<MovementSettings>,
    mut cam_q: Query<&Transform, (With<Camera3d>, Without<Player>)>,
) {
    if let Ok(window) = primary_window.get_single() {
        for mut player_transform in player_q.iter_mut() {

            let delta = time.delta().as_secs_f32();

            //https://courses.lumenlearning.com/suny-physics/chapter/5-2-drag-forces/
            let c = 0.05;   //drag coefficient of aircraft, this is equal to the coefficient of a sphere
            let a = 1.;     //cross section area, 1m^3 for now
            let p = 1.225;  //density of air at sea level


            let mut cur_velocity = settings.velocity;
            let mut cur_thrust = settings.thrust_force;
            let affine = player_transform.compute_affine();

            // let terminal_v = f32::sqrt(2. * settings.thrust_strength / (p * a * c));
            // let mut cur_speed  = cur_velocity.length();
            // println!("{cur_speed}, {terminal_v}");

            //compute this in local space
            //increase thrust force only if current speed less than the 90% of terminal velocity
            if keys.pressed(KeyCode::ShiftLeft){
                cur_thrust.z += settings.thrust_strength * delta;
            }

            //decrease thrust force
            if keys.pressed(KeyCode::ControlLeft){
                cur_thrust.z -= settings.thrust_strength * delta;
                if cur_thrust.z < 0. {
                    cur_thrust.z = 0.;
                }
            }

            settings.thrust_force = cur_thrust;

            //calculate thrust in global space
            let mut potential_velocity = cur_velocity;
            let mut thrust_force = affine.transform_vector3(cur_thrust); 
            thrust_force.y *= -1.;
            thrust_force.x *= -1.;
            potential_velocity += thrust_force * delta;

            //calculate gravity in global space
            let gravity_force = settings.gravity_force;
            potential_velocity += gravity_force * delta;

            //calculate drag
            let drag_direction = potential_velocity.normalize_or_zero();
            let drag_force = (0.5 * p * potential_velocity * potential_velocity * c * a).length();

            let total_input_force = thrust_force + gravity_force;
            let mut net_force = total_input_force - (drag_direction * drag_force);
            if net_force.dot(total_input_force) < 0.0 {
                net_force *= 0.0;
            }

            //apply net force
            cur_velocity += net_force * delta;
            println!("{cur_velocity}");

            //set final velocity
            settings.velocity = cur_velocity;

            //flip z
            cur_velocity.z *= -1.0;
            player_transform.translation += cur_velocity * delta;

            // let temp_force = thrust_force.normalize_or_zero();
            // let look_at = player_transform.translation + Vec3::new(temp_force.x, temp_force.y, -temp_force.z);
            // player_transform.rotation = player_transform.looking_at(look_at, Vec3::Y).rotation;

            if keys.pressed(KeyCode::KeyW){
                player_transform.rotate_local_axis(Vec3::X, -1.0 * delta);
            }
            if keys.pressed(KeyCode::KeyS){
                player_transform.rotate_local_axis(Vec3::X, 1.0 * delta);
            }
            if keys.pressed(KeyCode::KeyD){
                player_transform.rotate_local_axis(Vec3::Y, -1.0 * delta);
            }
            if keys.pressed(KeyCode::KeyA){
                player_transform.rotate_local_axis(Vec3::Y, 1.0 * delta);
            }
            if keys.pressed(KeyCode::KeyQ){
                player_transform.rotate_local_axis(Vec3::Z, 1.0 * delta);
            }
            if keys.pressed(KeyCode::KeyE){
                player_transform.rotate_local_axis(Vec3::Z, -1.0 * delta);
            }

        }
        
    }

}