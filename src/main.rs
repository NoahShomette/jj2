// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::window::WindowId;
use bevy::winit::WinitWindows;
use bevy::DefaultPlugins;
use bevy_mod_aseprite::AsepritePlugin;
use bevy_tiled_camera::TiledCameraPlugin;
use bevy_tweening::TweeningPlugin;
use seldom_state::StateMachinePlugin;
use std::io::Cursor;
use vampire_shopire::GamePlugin;
use winit::window::Icon;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(Color::rgb(
            (62 - 0) as f32 / (255 - 0) as f32,
            (35 - 0) as f32 / (255 - 0) as f32,
            (71 - 0) as f32 / (255 - 0) as f32,
        )))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: 1920.,
                        height: 1080.,
                        title: "Vampire Shopire".to_string(), // ToDo
                        canvas: Some("#bevy".to_owned()),
                        ..Default::default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(AsepritePlugin)
        .add_plugin(TiledCameraPlugin)
        .add_plugin(TweeningPlugin)
        .add_plugin(StateMachinePlugin)
        .add_plugin(GamePlugin)
        .add_startup_system(set_window_icon)
        .run();
}

// Sets the icon on windows and X11
fn set_window_icon(windows: NonSend<WinitWindows>) {
    let primary = windows.get_window(WindowId::primary()).unwrap();
    let icon_buf = Cursor::new(include_bytes!(
        "../build/macos/AppIcon.iconset/icon_256x256.png"
    ));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}
