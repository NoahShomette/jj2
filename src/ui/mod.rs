use crate::ui::game_scene::SceneUiPlugin;
use crate::ui::menu::MenuPlugin;
use crate::GameState;
use bevy::prelude::{
    App, AssetServer, Bundle, Color, Commands, Component, Plugin, Res, ResMut, Resource,
};
use iyes_loopless::prelude::AppLooplessStateExt;

pub mod game_scene;
pub mod menu;

pub struct UiPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiColors>()
            .add_enter_system(GameState::Setup, ui_setup);
        app.insert_resource(SceneUiState::default())
            .add_loopless_state(UiState::Normal);
        app.add_plugin(MenuPlugin).add_plugin(SceneUiPlugin);
    }
}

#[derive(Resource, Default)]
pub struct SceneUiState {
    ui_state: UiState,
}

#[derive(Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum UiState {
    #[default]
    Normal,
    Barter,
    Purchase,
    Pause,
}

pub fn ui_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("{}", ((104 - 0) as f32 / (255 - 0) as f32));
}

#[derive(Resource, Clone, PartialEq)]
pub struct UiColors {
    background_standard: Color,

    button_standard: Color,
    button_hovered: Color,
    
    success: Color,
    success_hovered: Color,
    
    failure: Color,
    failure_hovered: Color,
}

impl Default for UiColors {
    fn default() -> Self {
        UiColors {
            background_standard: Color::rgb(
                (104 - 0) as f32 / (255 - 0) as f32,
                (56 - 0) as f32 / (255 - 0) as f32,
                (108 - 0) as f32 / (255 - 0) as f32,
            ),
            button_standard: Color::rgb(
                (190 - 0) as f32 / (255 - 0) as f32,
                (74 - 0) as f32 / (255 - 0) as f32,
                (47 - 0) as f32 / (255 - 0) as f32,
            ),
            button_hovered: Color::rgb(
                (215 - 0) as f32 / (255 - 0) as f32,
                (118 - 0) as f32 / (255 - 0) as f32,
                (67 - 0) as f32 / (255 - 0) as f32,
            ),
            success:  Color::rgb(
                (60 - 0) as f32 / (255 - 0) as f32,
                (163 - 0) as f32 / (255 - 0) as f32,
                (112 - 0) as f32 / (255 - 0) as f32,
            ),
            success_hovered:  Color::rgb(
                (62 - 0) as f32 / (255 - 0) as f32,
                (137 - 0) as f32 / (255 - 0) as f32,
                (72 - 0) as f32 / (255 - 0) as f32,
            ),
            failure:  Color::rgb(
                (186 - 0) as f32 / (255 - 0) as f32,
                (59 - 0) as f32 / (255 - 0) as f32,
                (70 - 0) as f32 / (255 - 0) as f32,
            ),
            failure_hovered:  Color::rgb(
                (162 - 0) as f32 / (255 - 0) as f32,
                (38 - 0) as f32 / (255 - 0) as f32,
                (51 - 0) as f32 / (255 - 0) as f32,
            ),
        }
    }
}
