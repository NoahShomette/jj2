use bevy::prelude::{App, Color, Plugin, Resource};
use crate::GameState;
use crate::ui::menu::MenuPlugin;
use crate::ui::scene_ui::SceneUiPlugin;

pub mod scene_ui;
pub mod menu;

pub struct UiPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonColors>();
        app.add_plugin(MenuPlugin).add_plugin(SceneUiPlugin);
    }
}

#[derive(Resource)]
pub struct ButtonColors {
    normal: Color,
    hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::rgb(0.15, 0.15, 0.15),
            hovered: Color::rgb(0.25, 0.25, 0.25),
        }
    }
}