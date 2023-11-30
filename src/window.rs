use bevy::{prelude::*, window::{Cursor, close_on_esc}};

pub struct Window;

impl Plugin for Window {
    
    fn build(&self, app: &mut App) {

        app.add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(bevy::window::Window{
                title: "cs461-SimulationCapstone".to_string(),
                resolution: (1920 as f32, 1080 as f32).into(),
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                mode: bevy::window::WindowMode::Windowed,
                ..default()
            }),
            ..default()
        })).add_systems(Update, close_on_esc);
        //to do add ability to change window settings
        
        // add things to your app here
    }
}