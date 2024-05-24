use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);

// pub const MAIN_MENU_STYLE: Style = Style {
//     display: Display::Flex,
//     flex_direction: FlexDirection::Column,
//     justify_content: JustifyContent::Center,
//     align_items: AlignItems::Center,
//     width: Val::Percent(100.0),
//     height: Val::Percent(100.0),
//     ..Default::default()
//     // position_type: PositionType::Absolute,
//     // overflow: Overflow::clip(),
//     // direction: Direction::LeftToRight,
//     // left: Val::Percent(100.0),
//     // right: Val::Percent(100.0),
//     // top: Val::Percent(100.0),
//     // bottom: Val::Percent(100.0),
//     // aspect_ratio: AspectRatio::new(Val::Auto, Val::Auto),
//     // min_width: Val::Percent(100.0),
//     // min_height: Val::Percent(100.0),
//     // max_width: Val::Percent(100.0),
//     // max_height: Val::Percent(100.0),
// };

// pub const BUTTON_STYLE: Style = Style {
//     justify_content: JustifyContent::Center,
//     align_items: AlignItems::Center,
//     ..Style::DEFAULT
// };

// pub const IMAGE_STYLE: Style = Style {
//     size: Size::new(Val::Px(64.0), Val::Px(64.0)),
//     margin: UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0)),
//     ..Style::DEFAULT
// };

// pub const TITLE_STYLE: Style = Style {
//     flex_direction: FlexDirection::Row,
//     justify_content: JustifyContent::Center,
//     align_items: AlignItems::Center,
//     size: Size::new(Val::Px(300.0), Val::Px(120.0)),
//     ..Style::DEFAULT
// };

pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 64.0,
        color: Color::WHITE,
    }
}

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 32.0,
        color: Color::WHITE,
    }
}