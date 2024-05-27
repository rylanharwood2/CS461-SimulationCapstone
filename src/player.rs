use bevy::{core::Zeroable, gizmos, math::vec3, prelude::*, scene::ron::de, utils::{detailed_trace, RandomState}};
use bevy_third_person_camera::ThirdPersonCameraTarget;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use rand::{distributions::Normal, Rng};

use crate::ui::PauseState;
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
    pub thrust_force_max: f32,
    pub velocity: Vec3,
    pub thrust_force: f32,
    pub lift_direction: Vec3,
    pub gravity_force: Vec3,
    pub mass: f32,
    pub cross_section_body_area: f32,
    pub wing_area: f32,
    pub flaps_enabled: bool,
    pub flaps_angle: f32,
    pub angle_of_attack: f32,
    pub display_aero_forces: bool,
}

//https://www.grc.nasa.gov/www/k-12/BGP/Donna/t_w_ratio_answers.htm
//settings for boeing 747
impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            velocity: Vec3::new(300., 0., 0.),
            thrust_force: 1_008_000.,               //in Newtons
            gravity_force: Vec3::new(0., -9.81, 0.),  //m/s^2
            mass: 340_000.,                 //in KG
            lift_direction: Vec3::ZERO,
            thrust_force_max: 1_008_000.,    //in Newtons
            cross_section_body_area: 24.,   //M^2
            wing_area: 520.,                //M^2
            flaps_enabled: true,
            flaps_angle: 0.2,
            angle_of_attack: 0.0,
            display_aero_forces: true,
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
    mut pause: ResMut<PauseState>,
    mut cam_q: Query<&Transform, (With<Camera3d>, Without<Player>)>,
    mut gizmos: Gizmos,
) {

    if pause.is_paused {
        return;
    }

    if let Ok(window) = primary_window.get_single() {
        for mut player_transform in player_q.iter_mut() {
            if keys.just_pressed(KeyCode::KeyR){
                let x = ((rand::random::<u32>() as f32 / u32::MAX as f32) * 2. - 1.) * 10000.;
                let y = 100.0 as f32;
                let z = ((rand::random::<u32>() as f32 / u32::MAX as f32) * 2. - 1.) * 10000.;
                player_transform.translation = Vec3::new(x, y, z);
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

            //compute this in local space
            //increase thrust force only if current speed less than the 90% of terminal velocity
            if keys.pressed(KeyCode::ShiftLeft){
                cur_thrust += settings.thrust_force_max * delta;
                if cur_thrust >= settings.thrust_force_max {
                    cur_thrust = settings.thrust_force_max;
                }
            }

            //decrease thrust force
            if keys.pressed(KeyCode::ControlLeft){
                cur_thrust -= settings.thrust_force_max * delta;
                if cur_thrust <= 0.{
                    cur_thrust = 0.;
                }
            }
            //thrust force
            let cur_thrust_vec = forward * cur_thrust;

            let angle_of_attack = settings.angle_of_attack;

            //scale to apply to drag coeff when aoa is high
            let drag_coeff_scale = 1.2;

            //scale to artificially scale lift force
            let lift_bias = 1.2;

            //calculate lift

            let pre_lift_speed = cur_velocity.length();
            let lift_direction = settings.lift_direction;
            let lift_scaler = 1. - angle_of_attack;
            let lift_strength = (0.5 * p * f32::powf(cur_velocity.length(), 2.) * a1 * lift_bias) * lift_scaler;
            let lift = lift_direction * lift_strength;
            let lift_accel = lift / settings.mass; 
            cur_velocity += lift_accel * delta;
            cur_velocity = Vec3::clamp_length(cur_velocity, 0., pre_lift_speed);

            //calculate drag
            let drag_scaler = (1. - angle_of_attack).powf(2.);
            let drag_dir = -cur_velocity.normalize_or_zero();
            let drag_str = 0.5 * p * f32::powf(cur_velocity.length(), 2.) * (f32::lerp(c, c * drag_coeff_scale, drag_scaler)) * a;
            let drag_frc = drag_dir * drag_str;

            let mut net_accel = 
                cur_thrust_vec / settings.mass 
                + settings.gravity_force 
                + drag_frc / settings.mass;

            //apply net force
            cur_velocity += net_accel * delta;

            unsafe{
                let t = cur_velocity.length();
                net_accel += lift_accel;
                if TIMER > 1.0 {
                    println!("vel {cur_velocity}");
                    println!("accel {net_accel}");
                    println!("speed {t}");
                    println!("aoa {angle_of_attack}");
                    println!("lift dir {}", lift);
                    println!("thrust {}", cur_thrust / settings.mass);
                    // println!("lift delta {}", post_speed - speed);
                    println!("lift strength {}", lift_strength);
                    TIMER = 0.;
                }
            }

            //set final velocity
            settings.velocity = cur_velocity;
            settings.thrust_force = cur_thrust;

            player_transform.translation += cur_velocity * delta;

            //make plane face velocity
            let look_at = cur_velocity.normalize_or_zero();
            let axis = player_transform.up().xyz();
            player_transform.rotation = player_transform.looking_to(look_at, axis).rotation;

            //apply rolling
            let mut roll = 0.;
            if keys.pressed(KeyCode::KeyQ){
                roll += delta * (angle_of_attack);
            }
            if keys.pressed(KeyCode::KeyE){
                roll -= delta * (angle_of_attack);
            }
            player_transform.rotate_local_axis(Vec3::Z, roll);

            //adjust flap
            if keys.just_pressed(KeyCode::KeyF){
                settings.flaps_enabled = !settings.flaps_enabled;
            }
            if keys.pressed(KeyCode::ArrowUp){
                settings.flaps_angle += delta;
            }
            if keys.pressed(KeyCode::ArrowDown){
                settings.flaps_angle -= delta;
            }

            let half_pi = 3.14 / 2.;
            let quarter_pi = half_pi / 2.;

            //clamp angle
            settings.flaps_angle = f32::clamp(settings.flaps_angle, -quarter_pi, quarter_pi);

            //do flap offsets
            if settings.flaps_enabled == false {
                settings.flaps_angle = 0.;
            }
            let mut cur_flap_angle = settings.flaps_angle;

            //pitch offsets
            if keys.pressed(KeyCode::KeyW){
                cur_flap_angle += -quarter_pi * 0.5;
            }
            else if keys.pressed(KeyCode::KeyS){
                cur_flap_angle += quarter_pi * 0.5;
            }

            //clamp angle
            cur_flap_angle = f32::clamp(cur_flap_angle, -quarter_pi, quarter_pi);

            let dot = f32::sin(cur_flap_angle);
            
            let mut temp_lift_direction = Vec3::lerp(Vec3::Z, -Vec3::Y * dot / dot.abs(), dot.abs()).normalize();
            if dot == 0.{
                temp_lift_direction = Vec3::Z;
            }


            let dir = -cur_velocity.normalize_or_zero();
            let normal = player_transform.compute_affine().transform_vector3(temp_lift_direction).normalize_or_zero();


            settings.lift_direction = (dir - 2. * Vec3::dot(dir, normal) * normal).normalize_or_zero();
            settings.angle_of_attack = normal.dot(dir).max(0.0);

            if keys.just_pressed(KeyCode::KeyG) {
                settings.display_aero_forces = !settings.display_aero_forces;
            }

            if settings.display_aero_forces {
                gizmos.arrow(player_transform.translation, player_transform.translation + normal * 50., Color::GREEN);
                gizmos.arrow(player_transform.translation, player_transform.translation + settings.lift_direction * 50., Color::RED);
                gizmos.arrow(player_transform.translation, player_transform.translation + dir * 50., Color::BLUE);
            }
        }
        
    }

}