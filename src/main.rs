use bevy::prelude::*;
use CS461_SimulationCapstone::PlayerPlugin;
use CS461_SimulationCapstone::MovementSettings;
mod scene;
mod window;

//MAIN ENTRY, SHOULD BE VERY SPARSE
fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(window::Window)
        .add_plugins(PlayerPlugin)
        .insert_resource(MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 12.0,          // default: 12.0
            accel: 0.0,
            decel: 0.0,
            accel_max: 1.0,
            decel_min: -1.0,
            velocity: Vec3::ZERO,
        })
        .add_systems(Startup, scene::setup)
        .run();
}

