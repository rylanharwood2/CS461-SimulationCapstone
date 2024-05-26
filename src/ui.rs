use bevy::prelude::*;
use crate::player::MovementSettings;

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
struct InformationTextBox;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(PostUpdate, text_update_system);
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
) {
    for mut text in &mut query {
        let current_force = player.thrust_force;
        let percent_force = ((current_force / player.thrust_force_max) * 100.) as i32;
        let speed = f32::round(player.velocity.length());
        
        let output = format!("
            Throttle {}\n
            Speed(m/s) {}\n
            Flaps {}\n
            Flaps Angle {}\n", 
        percent_force, speed, player.flaps_enabled, player.flaps_angle);

        text.sections[0].value = output.to_string();
    }
}