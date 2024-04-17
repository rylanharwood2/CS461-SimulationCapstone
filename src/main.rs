use bevy::prelude::*;
use bevy_obj::ObjPlugin;
use CS461_SimulationCapstone::PlayerPlugin;
use CS461_SimulationCapstone::MovementSettings;
mod scene;
mod window;
use dotenv::dotenv;
use std::env;

//MAIN ENTRY, SHOULD BE VERY SPARSE
fn main() {
    dotenv().ok();
    App::new()
        .add_plugins((
            window::Window,
            PlayerPlugin,
            ObjPlugin,
        ))
        .insert_resource(MovementSettings {
            sensitivity: 0.00015, // default: 0.00012
            speed: 12.0,          // default: 12.0
        })
        .add_systems(Startup, (
            scene::setup,
            scene::generate_pre_chunks,
        ))
        .add_systems(Update, (
            scene::generate_chunks_update,
            scene::handle_terrain_data_threads,
            scene::update_sky_box,
            scene::terrain_controls
        ))
        .run();
}

