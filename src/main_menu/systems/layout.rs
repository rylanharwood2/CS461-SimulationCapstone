use bevy::prelude::*;

use crate::main_menu::components::*;
use crate::main_menu::styles::*;

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    //main menu screen bundle/style
    let main_menu_entity = commands.spawn((NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            background_color: Color::BLACK.into(),
            ..default()
            },
        MainMenu {},
    ))

    .with_children(|parent| {

        parent.spawn(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(500.0),
                left: Val::Px(500.0),
                
                ..default()
            },
            text: Text {
                sections: vec![TextSection::new(
                    "Xtreme Flight Simulator",
                    get_button_text_style(&asset_server),
                    )],
                    justify: JustifyText::Center,
                    
                    ..default()
                },
                ..default()
            });
            
            })
    //play button
    .with_children(|parent| {
        parent.spawn((
            ButtonBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(200.0),
                        height: Val::Px(80.0),
                        ..default()
                    },
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    ..default()
            },
            PlayButton {},
        ))
        //play button text
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Play",
                        get_button_text_style(&asset_server),
                    )],
                    justify: JustifyText::Center,
                    ..default()
                },
                ..default()
            });
            
            });
        })
    .id();

    main_menu_entity
}