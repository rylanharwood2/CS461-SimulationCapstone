use crate::player::MovementSettings;
use bevy::window::PrimaryWindow;
use bevy::{prelude::*, window::CursorGrabMode};
use bevy_third_person_camera::ThirdPersonCamera;
// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
struct InformationTextBox;

pub struct UiPlugin;
#[derive(Resource)]
pub struct PauseState {
    pub is_paused: bool,
}
impl Default for PauseState {
    fn default() -> Self {
        Self { is_paused: true }
    }
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(PostUpdate, text_update_system)
            .add_systems(PreUpdate, pause_update)
            .init_resource::<PauseState>();
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Text with one section
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "hello\nbevy!",
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font: asset_server.load("fonts/Montserrat-VariableFont_wght.ttf"),
                font_size: 40.0,
                ..default()
            },
        ) // Set the justification of the Text
        .with_text_justify(JustifyText::Left)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Relative,
            top: Val::Px(32.0),
            left: Val::Px(32.0),
            ..default()
        }),
        InformationTextBox,
    ));
}

fn text_update_system(
    player: Res<MovementSettings>,
    mut query: Query<&mut Text, With<InformationTextBox>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for mut text in &mut query {
        let current_force = player.thrust_force;
        let percent_force = ((current_force / player.thrust_force_max) * 100.) as i32;
        let speed = f32::round(player.velocity.length());

        let output = format!(
            "
            Throttle {}\n
            Speed(m/s) {}\n
            Flaps {}\n
            Flaps Angle {}\n
            Angle Up/Down: W / S
            Roll Angle: Q / E
            Flaps Angle Control: Arrows
            Pause: Escape
            Throttle: LShift / Ctrl",
            percent_force,
            speed,
            player.flaps_enabled,
            player.flaps_angle * 180.0 / 3.14
        );

        text.sections[0].value = output.to_string();
    }
}
fn pause_update(
    keys: Res<ButtonInput<KeyCode>>,
    mut pause: ResMut<PauseState>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
    mut tpc: Query<&mut ThirdPersonCamera>,
    mut query: Query<&mut Text, With<InformationTextBox>>,
) {
    if pause.is_paused {
        let mut window = &mut primary_window.single_mut();
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
        tpc.single_mut().cursor_lock_active = false;
        tpc.single_mut().cursor_lock_toggle_enabled = false;
        for mut text in &mut query {
            let output = format!(
                "
            Pause",
            );
            text.sections[0].value = output.to_string();
        }
    } else {
        tpc.single_mut().cursor_lock_toggle_enabled = true;
    }

    if keys.just_pressed(KeyCode::Escape) {
        if pause.is_paused == false {
            pause.is_paused = true;
        } else {
            pause.is_paused = false;
            tpc.single_mut().cursor_lock_active = true;
        }
    }
}
