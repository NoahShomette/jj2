use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;
use bevy_mod_aseprite::{aseprite, Aseprite};
use iyes_loopless::prelude::AppLooplessStateExt;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(GameState::Loading)
            .init_resource::<AsepriteHandles>()
            .init_resource::<CustomerAsepriteHandles>()
            .add_enter_system(GameState::Loading, load_asperite)
            .add_loading_state(
                LoadingState::new(GameState::Loading)
                    .with_collection::<FontAssets>()
                    .with_collection::<AudioAssets>()
                    .with_collection::<TextureAssets>()
                    .continue_to_state(GameState::Setup),
            );
    }
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct AsepriteHandles(Vec<Handle<Aseprite>>);

#[derive(Resource, Deref, DerefMut, Default)]
pub struct CustomerAsepriteHandles(Vec<Handle<Aseprite>>);

aseprite!(pub DapperPanda, "textures/dapper_panda.aseprite");
aseprite!(pub ShopBackground, "textures/shop_bg.aseprite");

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub texture_bevy: Handle<Image>,
}

fn load_asperite(
    mut aseprite_handles: ResMut<AsepriteHandles>,
    mut customer_aseprite_handles: ResMut<CustomerAsepriteHandles>,
    asset_server: Res<AssetServer>,
) {
    //general stuff
    let background: Handle<Aseprite> = asset_server.load(ShopBackground::PATH);
    aseprite_handles.push(background);

    
    //customer stuff
    let dapper_panda: Handle<Aseprite> = asset_server.load(DapperPanda::PATH);
    customer_aseprite_handles.push(dapper_panda);
}
