use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_mod_picking::{PickableBundle, PickingCameraBundle};
use bevy_tiled_camera::{TiledCameraBundle, WorldSpace};
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet};

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
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
    picking_camera_bundle: PickingCameraBundle,
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
            picking_camera_bundle: PickingCameraBundle::default(),
        }
    }
}

fn setup_basics(mut commands: Commands) {
    //commands.spawn(Camera2dBundle::default());
    commands.spawn((CameraBundle::default()));
}
