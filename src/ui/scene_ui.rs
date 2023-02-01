use bevy::hierarchy::{BuildChildren, DespawnRecursiveExt};
use bevy::prelude::{AlignItems, App, AssetServer, BackgroundColor, Button, ButtonBundle, Changed, Color, Commands, Component, Entity, Interaction, JustifyContent, Plugin, Query, Res, ResMut, Resource, Size, Style, Text, TextBundle, TextSection, TextStyle, UiRect, Val, With};
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet};
use kayak_ui::prelude::{widgets::*, *};
use kayak_ui::UICameraBundle;
use kayak_ui::widgets::KayakWidgetsContextPlugin;
use crate::GameState;
use crate::loading::FontAssets;
use crate::ui::ButtonColors;

pub struct SceneUiPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for SceneUiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SceneUiState::default())
            .add_enter_system(GameState::Playing, setup_scene_ui)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Playing)
                    .with_system(handle_barter_button)
                    .into(),
            )
            .add_exit_system(GameState::Playing, cleanup_menu);
    }
}

#[derive(Resource, Default)]
pub struct SceneUiState{
    ui_state: UiState,
}

#[derive(Default)]
pub enum UiState{
    #[default]
    Normal,
    Barter,
    Purchase,
    Pause,
}


#[derive(Component)]
pub struct BarterUi;

fn setup_scene_ui(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    font_mapping.set_default(asset_server.load("abaddon_bold.kayak_font"));

    let mut widget_context = KayakRootContext::new();
    widget_context.add_plugin(KayakWidgetsContextPlugin);
    let parent_id = None;
    rsx! {
        <KayakAppBundle>
            <TextWidgetBundle
                text={TextProps {
                    content: "Hello World".into(),
                    size: 20.0,
                    ..Default::default()
                }}
            />
        </KayakAppBundle>
    };

    commands.spawn(UICameraBundle::new(widget_context));
    
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(120.0), Val::Px(120.0)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: button_colors.normal.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Barter".to_string(),
                        style: TextStyle {
                            font: font_assets.fira_sans.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    alignment: Default::default(),
                },
                ..Default::default()
            });
        });
}

fn handle_barter_button(
    button_colors: Res<ButtonColors>,
    mut state: ResMut<SceneUiState>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                state.ui_state = UiState::Barter;
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, button: Query<Entity, With<Button>>) {
    commands.entity(button.single()).despawn_recursive();
}