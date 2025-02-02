use bevy::prelude::*;

pub const MAIN_MENU_NODE: Node = {
    let mut node = Node::DEFAULT;
    node.flex_direction = FlexDirection::Column;
    node.justify_content = JustifyContent::Center;
    node.align_items = AlignItems::Center;
    node.width = Val::Percent(100.0);
    node.height = Val::Percent(100.0);
    node.row_gap = Val::Px(8.0);
    node.column_gap = Val::Px(8.0);
    node
};

pub const BUTTON_NODE: Node = {
    let mut node = Node::DEFAULT;
    node.justify_content = JustifyContent::Center;
    node.align_items = AlignItems::Center;
    node.width = Val::Px(200.0);
    node.height = Val::Px(80.0);
    node
};

pub const TITLE_NODE: Node = {
    let mut node = Node::DEFAULT;
    node.flex_direction = FlexDirection::Row;
    node.justify_content = JustifyContent::Center;
    node.align_items = AlignItems::Center;
    node.width = Val::Px(600.0);
    node.height = Val::Px(120.0);
    node
};

pub fn get_title_text_style(asset_server: &AssetServer) -> (TextFont, TextColor) {
    (
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 64.0,
            ..Default::default()
        },
        TextColor(Color::WHITE),
    )
}

pub fn get_button_text_style(asset_server: &AssetServer) -> (TextFont, TextColor) {
    (
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 32.0,
            ..Default::default()
        },
        TextColor(Color::WHITE),
    )
}
