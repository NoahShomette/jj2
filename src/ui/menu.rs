use crate::barter::BarterResolutionTypes;
use crate::loading::FontAssets;
use crate::ui::game_scene::barter_screen::{BarterControlButtonProps, BarterUi};
use crate::ui::UiColors;
use crate::GameState;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::render::render_graph::Edge;
use bevy_tiled_camera::WorldSpace::Units;
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet};
use iyes_loopless::state::{CurrentState, NextState};

pub struct MenuPlugin;

///
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::MainMenu, setup_menu)
            .add_exit_system(GameState::MainMenu, cleanup_menu)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::MainMenu)
                    .with_system(click_play_button)
                    .into(),
            );
    }
}

#[derive(Component, Clone, PartialEq, Default)]
pub struct MenuUi;

#[derive(Component, Clone, PartialEq, Default)]
pub struct MenuButtonProps {
    menu_button_type: MenuButtonType,
}

#[derive(Clone, PartialEq, Default)]
pub enum MenuButtonType {
    #[default]
    NewGame,
    Options,
    Exit,
}

impl MenuButtonProps {
    pub fn get_button_text(&self) -> String {
        return match self.menu_button_type {
            MenuButtonType::NewGame => String::from("New Game"),
            MenuButtonType::Options => String::from("Options"),
            MenuButtonType::Exit => String::from("Exit"),
        };
    }
}

fn setup_menu(mut commands: Commands, font_assets: Res<FontAssets>, colors: Res<UiColors>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        })
        .insert(MenuUi)
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(30.0), Val::Percent(50.0)),
                        justify_content: JustifyContent::SpaceBetween,
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    background_color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Auto, Val::Auto),
                                padding: UiRect::all(Val::Px(15.0)),
                                margin: UiRect::all(Val::Auto),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            background_color: colors.button_standard.into(),
                            ..Default::default()
                        })
                        .insert(MenuButtonProps {
                            menu_button_type: MenuButtonType::NewGame,
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text {
                                    sections: vec![TextSection {
                                        value: "Play".to_string(),
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
                });
        });
}

fn click_play_button(
    mut commands: Commands,
    button_colors: Res<UiColors>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                commands.insert_resource(NextState(GameState::Playing));
            }
            Interaction::Hovered => {
                *color = button_colors.button_hovered.into();
            }
            Interaction::None => {
                *color = button_colors.button_standard.into();
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, button: Query<Entity, With<MenuUi>>) {
    for button in button.iter() {
        commands.entity(button).despawn_recursive();
    }
}
