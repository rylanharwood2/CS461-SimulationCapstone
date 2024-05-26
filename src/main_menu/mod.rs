mod components;
mod styles;
mod systems;

use systems::interactions::*;
use systems::layout::*;

use::bevy::prelude::*;

use crate::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu) //spawn menu when entering menu state
        
        //OTHER BUTTON FUNCTIONS GO HERE VVV
        .add_systems(Update, interact_with_play_button)
        // OnExit State Systems
        .add_systems(OnExit(AppState::MainMenu), despawn_main_menu); //despawn menu when exiting menu state
    }
}




// fn setup_menu(mut commands: Commands, assets: Res<AssetServer>) {
//     commands.spawn_bundle(UiCameraBundle::default());
//     commands.spawn_bundle(ButtonBundle {
//         style: Style {
//             align_self: AlignSelf::Center,
//             align_items: AlignItems::Center,
//             justify_content: JustifyContent::Center,
//             size: Size::new(width: Val::Percent(20.0),height: Val::Percent(10.0)),
//             margin: Rect::all(Val::Auto),
//             ..Default::default()
//         },
//         ..Default::default()
//     }).with_children(|parent: &mut ChildBuilder| {
//         parent.spawn_bundle( ImageBundle {
//              style: Style {
//                 size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
//                 justify_content: JustifyContent::Center,
//                 align_items: AlignItems::Center,
//                 ..Default::default()
//              },
             
//         });
//     });
// }