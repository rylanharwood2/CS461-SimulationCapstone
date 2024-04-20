use bevy::{core::Zeroable, math::vec3, prelude::*, utils::RandomState};
use bevy_third_person_camera::ThirdPersonCameraTarget;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use rand::Rng;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .init_resource::<MovementSettings>()
            .add_systems(PreUpdate, player_movement);
    }
}

/// Mouse sensitivity and movement speed
#[derive(Resource)]
pub struct MovementSettings {
    pub thrust_strength: f32,
    pub velocity: Vec3,
    pub thrust_force: f32,
    pub lift_direction: Vec3,
    pub gravity_force: Vec3,
    pub mass: f32,
    pub cross_section_body_area: f32,
    pub wing_area: f32,
    pub roll: f32,
}

//https://www.grc.nasa.gov/www/k-12/BGP/Donna/t_w_ratio_answers.htm
//settings for boeing 747
impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            velocity: Vec3::new(250., 0., 0.),
            thrust_force: 0.,               //in Newtons
            gravity_force: Vec3::new(0., -9.81, 0.),  //m/s^2
            mass: 340_000.,                 //in KG
            lift_direction: Vec3::ZERO,
            thrust_strength: 1_008_000.,    //in Newtons
            cross_section_body_area: 24.,   //M^2
            wing_area: 520.,                //M^2
            roll: 0.,
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
            scene: assets.load("./plane/boeing_787.gltf#Scene0"),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()

        },
        Player,
        ThirdPersonCameraTarget,
    );
    commands.spawn(player);
}
static mut TIMER: f32 = 0.;

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
            if keys.just_pressed(KeyCode::KeyR){
                let numx = (rand::thread_rng().gen_range(0, 1) as f32 * 2. - 1.) * 1000000.;
                let numz = (rand::thread_rng().gen_range(0, 1) as f32 * 2. - 1.) * 1000000.;
                player_transform.translation = Vec3::new(numx as f32, player_transform.translation.y, numz as f32)
            }
            let delta = time.delta().as_secs_f32();

            unsafe{
                TIMER += delta;
            }

            //https://courses.lumenlearning.com/suny-physics/chapter/5-2-drag-forces/
            let c = 0.031;   //drag coefficient of boeing 747, this is equal to the coefficient of a sphere
            let p = 1.225;  //density of air at sea level kg/m^3

            let a = settings.cross_section_body_area;     //cross section area of plane, 1m^3
            let a1 = settings.wing_area;    //cross section area of wing, 1m^3

            let mut cur_velocity = settings.velocity;
            let mut cur_thrust = settings.thrust_force;
            let forward = player_transform.forward().xyz();

            // let terminal_v = f32::sqrt(2. * settings.thrust_strength / (p * a * c));
            // let mut cur_speed  = cur_velocity.length();
            // println!("{cur_speed}, {terminal_v}");

            //compute this in local space
            //increase thrust force only if current speed less than the 90% of terminal velocity
            if keys.pressed(KeyCode::ShiftLeft){
                cur_thrust += settings.thrust_strength * delta;
                if cur_thrust >= settings.thrust_strength {
                    cur_thrust = settings.thrust_strength;
                }
            }

            //decrease thrust force
            if keys.pressed(KeyCode::ControlLeft){
                cur_thrust -= settings.thrust_strength * delta;
                if cur_thrust <= 0.{
                    cur_thrust = 0.;
                }
            }
            let drag_bias = 85.;
            let lift_bias = 25.;
            //thrust force
            let cur_thrust_vec = forward * cur_thrust;

            //calculate lift
            let mut angle_of_attack = f32::max(Vec3::dot(cur_velocity.normalize_or_zero(), settings.lift_direction.normalize_or_zero()), 0.);
            angle_of_attack = f32::powf(angle_of_attack, 1.);

            let lift_direction = (settings.lift_direction - player_transform.forward().xyz()) * angle_of_attack;
            let lift_strength = 0.5 * p * f32::powf(cur_velocity.length(), 2.) * c * a1 * lift_bias;
            let lift = lift_direction * lift_strength;
            
            //calculate drag
            let drag_dir = -cur_velocity.normalize_or_zero();
            let drag_str = 0.5 * p * f32::powf(cur_velocity.length(), 2.) * c * a;
            let drag_frc = drag_dir * drag_str * f32::lerp(drag_bias, 1., angle_of_attack);

            let net_accel = 
                cur_thrust_vec / settings.mass 
                + settings.gravity_force 
                + drag_frc / settings.mass 
                + lift / settings.mass;

            //apply net force
            cur_velocity += net_accel * delta;

            unsafe{
                let t = cur_velocity.length();
                if TIMER > 1.0 {
                    println!("vel {cur_velocity}");
                    println!("accel {net_accel}");
                    println!("speed {t}");
                    println!("aoa {angle_of_attack}");
                    println!("drag accel {}", drag_frc / settings.mass);
                    println!("lift accel {}", lift / settings.mass);
                    println!("thrust {}", cur_thrust / settings.mass);
                    TIMER = 0.;
                }
            }

            //set final velocity
            settings.velocity = cur_velocity;
            settings.thrust_force = cur_thrust;

            player_transform.translation += cur_velocity * delta;

            let look_at = Vec3::new(cur_velocity.x, cur_velocity.y, cur_velocity.z).normalize_or_zero();
            let axis = Vec3::Y;
            player_transform.rotation = player_transform.looking_to(look_at, axis).rotation;
            player_transform.rotate_local_axis(Vec3::Z, settings.roll);

            let lift_strength = 2.;
            let mut temp_lift_direction = -Vec3::Z + Vec3::Y * 0.5;
            if keys.pressed(KeyCode::KeyW){
                temp_lift_direction += -Vec3::Y * lift_strength;
            }
            if keys.pressed(KeyCode::KeyS){
                temp_lift_direction += Vec3::Y * lift_strength;
            }
            if keys.pressed(KeyCode::KeyD){
                temp_lift_direction += Vec3::X * lift_strength;
            }
            if keys.pressed(KeyCode::KeyA){
                temp_lift_direction += -Vec3::X * lift_strength;
            }
            settings.lift_direction = player_transform.compute_affine().transform_vector3(temp_lift_direction);

            if keys.pressed(KeyCode::KeyQ){
                settings.roll += delta * angle_of_attack;
            }
            if keys.pressed(KeyCode::KeyE){
                settings.roll -= delta * angle_of_attack;
            }
        }
        
    }

}