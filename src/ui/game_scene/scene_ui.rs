use crate::loading::FontAssets;
use crate::ui::menu::{MenuButtonProps, MenuButtonType, MenuUi};
use crate::ui::{UiColors, UiState};
use crate::{GameState, PausedState};
use bevy::prelude::*;
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet};
use iyes_loopless::state::NextState;

pub struct GameSceneUiPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for GameSceneUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::Playing, setup_scene_ui)
            .add_exit_system(GameState::Playing, cleanup_game_ui)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Playing)
                    .run_in_state(PausedState::Playing)
                    .with_system(click_play_button)
                    .into(),
            );
    }
}

#[derive(Component, Clone, PartialEq, Default)]
pub struct GameUi;

#[derive(Component, Clone, PartialEq, Default)]
pub struct GameMainButtonProps {
    game_button_type: GameStateButtons,
}

impl GameMainButtonProps {
    pub fn get_button_text(&self) -> String {
        return match self.game_button_type {
            GameStateButtons::Barter => String::from("Barter"),
            GameStateButtons::Purchase => String::from("Purchase"),
            GameStateButtons::Options => String::from("Options"),
        };
    }
}

#[derive(Clone, PartialEq, Default)]
pub enum GameStateButtons {
    #[default]
    Barter,
    Purchase,
    Options,
}

fn setup_scene_ui(mut commands: Commands, font_assets: Res<FontAssets>, colors: Res<UiColors>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        })
        .insert(MenuUi)
        .with_children(|parent| {
            // left vertical fill (border)
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
                .insert(GameUi)
                .insert(GameMainButtonProps {
                    game_button_type: GameStateButtons::Barter,
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
        });
}

fn click_play_button(
    mut commands: Commands,
    button_colors: Res<UiColors>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &GameMainButtonProps),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, props) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                *color = button_colors.button_hovered.into();
            }
            Interaction::None => {
                *color = button_colors.button_standard.into();
            }
            _ => {}
        }
        match props.game_button_type {
            GameStateButtons::Barter => {
                if let Interaction::Clicked = interaction {
                    commands.insert_resource(NextState(UiState::Barter));
                }
            }
            GameStateButtons::Purchase => {}
            GameStateButtons::Options => {}
        }
    }
}

fn cleanup_game_ui(mut commands: Commands, button: Query<Entity, With<GameUi>>) {
    for button in button.iter() {
        commands.entity(button).despawn_recursive();
    }
}
