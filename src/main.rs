use bevy::prelude::*;
mod window;

//MAIN ENTRY, SHOULD BE VERY SPARSE
fn main() {
    App::new()
        .add_plugins(window::Window)
        .run();
}
