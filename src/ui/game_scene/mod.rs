use crate::ui::game_scene::barter_screen::BarterUiPlugin;
use crate::ui::game_scene::scene_ui::{GameSceneUiPlugin};
use crate::GameState;
use bevy::prelude::{App, Plugin, Resource};
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet};

pub mod barter_screen;
pub mod scene_ui;

pub struct SceneUiPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for SceneUiPlugin {
    fn build(&self, app: &mut App) {

        app.add_plugin(BarterUiPlugin)
            .add_plugin(GameSceneUiPlugin);
    }
}
