use crate::ui::game_scene::barter_screen::BarterUiPlugin;
use crate::ui::game_scene::scene_ui::{setup_scene_ui, GameSceneUiPlugin};
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
        app.insert_resource(SceneUiState::default())
            .add_loopless_state(UiState::Normal);
        app.add_plugin(BarterUiPlugin)
            .add_plugin(GameSceneUiPlugin);
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
