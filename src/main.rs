use bevy::prelude::*;
use style::{get_button_text_style, get_title_text_style, BUTTON_NODE, MAIN_MENU_NODE, TITLE_NODE};

mod style;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Match,
    InGame,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_systems(Startup, spawn_camera)
        .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
        .run();
}

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct QuitButton;

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((MAIN_MENU_NODE, MainMenu))
        .with_children(|parent| {
            parent.spawn(TITLE_NODE).with_children(|parent| {
                // Text
                parent.spawn((Text::new("Geister"), get_title_text_style(&asset_server)));
            });

            parent
                .spawn((BUTTON_NODE, Button, PlayButton))
                .with_children(|parent| {
                    parent.spawn((Text::new("Play"), get_button_text_style(&asset_server)));
                });

            parent
                .spawn((BUTTON_NODE, Button, QuitButton))
                .with_children(|parent| {
                    parent.spawn((Text::new("Quit"), get_button_text_style(&asset_server)));
                });
        });
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3d::default());
}
