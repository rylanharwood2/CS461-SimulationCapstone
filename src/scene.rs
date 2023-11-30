use bevy::prelude::*;
// use crate::materials::BasicTerrainMaterial;


pub fn setup(
    mut commands: Commands,
) {
    // light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight{
            color: Color::Rgba { red: (1.0), green: (1.0), blue: (1.0), alpha: (1.0) },
            illuminance: 10000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -1.2, -1.2, 0.)),
        ..Default::default()
    });

    info!("Move camera around by using WASD for lateral movement");
    info!("Use Left Shift and Spacebar for vertical movement");
    info!("Use the mouse to look around");
    info!("Press Esc to hide or show the mouse cursor");
}


pub fn setup_terrain(    
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>){

    // terrain
    //loads a simple terrain mesh
    //todo: generate the terrain mesh
    let mesh_handle = asset_server.load("mesh/temp_terrain.obj");

    commands.spawn(PbrBundle {
        mesh: mesh_handle,
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform{
            translation: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            scale: Vec3 { x: 10.0, y: 10.0, z: 10.0 },
            ..Default::default()
        },
        ..Default::default()
    });
}