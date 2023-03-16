
use crate::loading::{AsepriteHandles};
use crate::GameState;
use bevy::prelude::{default, Assets, Commands, Res, ResMut, TextureAtlasSprite, Transform, Vec3, Plugin, App};
use bevy_mod_aseprite::{Aseprite, AsepriteAnimation, AsepriteBundle};
use iyes_loopless::prelude::AppLooplessStateExt;

pub struct ScenePlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::MainMenu, setup_background_scene);
        //.add_system_set(ConditionSet::new().run_in_state(GameState::Playing).into());
    }
}

pub fn setup_background_scene(
    aseprite_handles: ResMut<AsepriteHandles>,
    aseprites: Res<Assets<Aseprite>>,
    mut commands: Commands,
) {
    let aseprite_handle = &aseprite_handles[0];
    let sprite = aseprites.get(aseprite_handle).unwrap();
    
    let animation = AsepriteAnimation::new(sprite.info(), "Loop");

    commands.spawn(
        (AsepriteBundle {
            aseprite: aseprite_handle.clone_weak(),
            sprite: TextureAtlasSprite::new(animation.current_frame()),
            animation,
            texture_atlas: sprite.atlas().clone_weak(),
            transform: Transform {
                translation: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                ..default()
            },
            global_transform: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
        }),
    );
}
