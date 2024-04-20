use bevy::prelude::*;
use bevy_third_person_camera::{camera::{Offset, Zoom}, ThirdPersonCamera};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    let camera = (
        Camera3dBundle {
            transform: Transform::from_xyz(0.,0.,0.),
            projection: Projection::Perspective(PerspectiveProjection { fov: (1.22173), aspect_ratio: (16./9.), near: (0.1), far: (10000.) }),
            ..default()
        },
        ThirdPersonCamera{
            zoom: Zoom::new(5.0, 1000.0),
            offset_enabled: true,
            offset: Offset::new(0., 8.),
            ..default()
        },
        FogSettings {
            color: Color::rgba_u8(61, 151, 255, 255) * 2.0,
            directional_light_color: Color::rgba(1.0, 0.95, 0.85, 0.5) * 2.5,
            directional_light_exponent: 30.0,
            falloff: FogFalloff::from_visibility_colors(
                200.0, // distance in world units up to which objects retain visibility (>= 5% contrast)
                Color::rgb(1.0,1.0,1.0), // atmospheric extinction color (after light is lost due to absorption by atmospheric particles)
                Color::rgb(0.8, 0.844, 0.86) * 0.5, // atmospheric inscattering color (light gained due to scattering from the sun)
            ),
        },
    );
    commands.spawn(camera);
}