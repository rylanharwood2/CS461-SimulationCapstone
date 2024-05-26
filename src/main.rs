use bevy::prelude::*;
use bevy_obj::ObjPlugin;
use dotenv::dotenv;
use bevy::DefaultPlugins;
use bevy_third_person_camera::*;
mod camera;
mod player;
mod scene;
mod ui;

//MAIN ENTRY, SHOULD BE VERY SPARSE
fn main() {
    dotenv().ok();
    App::new()
        .add_plugins((
            DefaultPlugins,
            ThirdPersonCameraPlugin,
            ui::UiPlugin,
            camera::CameraPlugin,
            player::PlayerPlugin,
        ))
        .add_plugins(ObjPlugin)
        .add_systems(Startup, (
            scene::setup,
            scene::generate_pre_chunks,
        ))
        .add_systems(Update, (
            scene::generate_chunks_update,
            scene::handle_terrain_data_threads,
            scene::update_sky_box,
        ))
        .run();
}

