mod audio;
mod barter;
mod loading;
mod player;
mod ui;

use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::player::PlayerPlugin;

use crate::barter::BarterPlugin;
use crate::ui::UiPlugin;
use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet, NextState};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum PausedState {
    // During the loading State the LoadingPlugin will load our assets
    Paused,
    // During this State the actual game logic is executed
    Playing,
}

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // Used for anything that needs to be setup between loading and when we start to render and use stuff
    Setup,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    MainMenu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LoadingPlugin)
            .add_plugin(UiPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(BarterPlugin);

        app.add_loopless_state(PausedState::Playing);

        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Setup)
                .with_system(exit_setup)
                .into(),
        );

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}

fn exit_setup(mut commands: Commands) {
    commands.insert_resource(NextState(GameState::MainMenu));
}
