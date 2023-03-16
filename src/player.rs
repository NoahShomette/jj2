use crate::GameState;
use bevy::prelude::*;
use bevy_pixel_camera::PixelCameraBundle;
use bevy_tiled_camera::{TiledCameraBundle, WorldSpace};
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet};

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Gold>();
        
        app.add_enter_system(GameState::MainMenu, setup_basics)
            .add_system_set(ConditionSet::new().run_in_state(GameState::Playing).into());
    }
}
#[derive(PartialEq, Clone, Copy, Debug, Default, Component)]
pub struct CameraMarker;

#[derive(Bundle)]
pub struct CameraBundle {
    tiled_camera: TiledCameraBundle,
    camera: CameraMarker,
}

impl Default for CameraBundle {
    fn default() -> Self {
        CameraBundle {
            tiled_camera: TiledCameraBundle::new()
                .with_pixels_per_tile([16, 16])
                .with_tile_count([20, 11])
                .with_world_space(WorldSpace::Pixels)
                .with_clear_color(Color::hex("090a14").expect("Color not valid hex")),
            camera: Default::default(),
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug, Default, Resource)]
pub struct Gold {
    pub amount: u32,
}

fn setup_basics(mut commands: Commands) {
    commands
        .spawn(PixelCameraBundle::from_resolution(320, 240))
        .insert(CameraMarker);
}
