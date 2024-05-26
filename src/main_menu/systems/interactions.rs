use bevy::prelude::*;
use rand::random;

use crate::main_menu::components::*;
use crate::main_menu::styles::{HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR};
use crate::player::Player;
use crate::ui::PauseState;
use crate::AppState;


//function to handle play button clicks. similar functions must be made for other buttons
pub fn interact_with_play_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut player_q: Query<&mut Transform, With<Player>>,
    mut pause_state: ResMut<PauseState>
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                for mut player_transform in player_q.iter_mut() {
                    let x = ((rand::random::<u32>() as f32 / u32::MAX as f32) * 2. - 1.) * 10000.;
                    let y = 100.0 as f32;
                    let z = ((rand::random::<u32>() as f32 / u32::MAX as f32) * 2. - 1.) * 10000.;
                    player_transform.translation = Vec3::new(x, y, z);
                }
                
                app_state_next_state.set(AppState::Game);
                pause_state.is_paused = false;
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}