use crate::ui::game_scene::SceneUiPlugin;
use crate::ui::menu::MenuPlugin;
use crate::GameState;
use bevy::prelude::{
    App, AssetServer, Bundle, Color, Commands, Component, Plugin, Res, ResMut, Resource,
};
use iyes_loopless::prelude::AppLooplessStateExt;
use kayak_ui::prelude::{widgets::*, *};
use kayak_ui::widgets::KayakWidgetsContextPlugin;
use kayak_ui::UICameraBundle;

pub mod game_scene;
pub mod menu;

pub struct UiPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiColors>()
            .add_enter_system(GameState::Setup, ui_setup);
        app.add_plugin(MenuPlugin).add_plugin(SceneUiPlugin);
    }
}

pub fn ui_setup(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    font_mapping.set_default(asset_server.load("abaddon_bold.kayak_font"));

    let mut widget_context = KayakRootContext::new();
    widget_context.add_plugin(KayakWidgetsContextPlugin);

    commands.spawn(UICameraBundle::new(widget_context));

    println!("{}", ((104 - 0) as f32 / (255 - 0) as f32));
}

#[derive(Resource, Clone, PartialEq)]
pub struct UiColors {
    background_standard: Color,

    button_standard: Color,
    button_hovered: Color,
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
        }
    }
}
