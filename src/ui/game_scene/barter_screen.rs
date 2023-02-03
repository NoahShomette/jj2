use crate::barter::{
    BarterAttemptEvent, BarterAttemptResult, BarterAttemptResultEvent, BarterResolutionTypes,
    BarterResolved, BarterTypes,
};
use crate::loading::FontAssets;
use crate::ui::game_scene::scene_ui::GameStateButtons::Barter;
use crate::ui::game_scene::scene_ui::{GameMainButtonProps, GameStateButtons, GameUi};
use crate::ui::{UiColors, UiState};
use crate::{GameState, PausedState};
use bevy::app::AppExit;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet, CurrentState, NextState};
use std::ops::DerefMut;
use std::process::id;
use std::thread::spawn;

pub struct BarterUiPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for BarterUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(UiState::Barter, setup_barter_ui)
            .add_exit_system(UiState::Barter, cleanup_barter_ui)
            .add_system_set(
                ConditionSet::new()
                    .run_on_event::<BarterAttemptResultEvent>()
                    .with_system(spawn_barter_result)
                    .into(),
            )
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(UiState::Barter)
                    .run_in_state(PausedState::Playing)
                    .with_system(click_barter_control_button)
                    .with_system(click_barter_button)
                    .into(),
            );
    }
}

#[derive(Component, Clone, PartialEq, Default)]
pub struct ResolutionUiParent;

#[derive(Component, Clone, PartialEq)]
pub struct BarterButtonProps {
    barter_button_type: BarterTypes,
}

#[derive(Component, Clone, PartialEq, Default)]
pub struct BarterControlButtonProps {
    control_button_type: BarterResolutionTypes,
}

impl BarterControlButtonProps {
    pub fn get_button_text(&self) -> String {
        return match self.control_button_type {
            BarterResolutionTypes::Approve { amount } => String::from("Approve"),
            BarterResolutionTypes::Deny => String::from("Deny"),
        };
    }
}

#[derive(Component, Default, PartialEq, Clone)]
pub struct BarterUi;

fn setup_barter_ui(mut commands: Commands, font_assets: Res<FontAssets>, colors: Res<UiColors>) {
    // root bundle
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
        .insert(BarterUi)
        .with_children(|parent| {
            // main background and holder for barter ui

            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(75.0), Val::Percent(80.0)),
                        justify_content: JustifyContent::SpaceBetween,
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    background_color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // left side of barter screen
                    setup_left_barter_screen(parent, &font_assets, &colors);

                    // center of barter screen
                    setup_middle_barter_screen(parent, &font_assets, &colors);

                    // right side of barter screen
                    setup_right_barter_screen(parent, &font_assets, &colors);
                });
        });
}

fn setup_left_barter_screen(
    parent: &mut ChildBuilder,
    font_assets: &Res<FontAssets>,
    colors: &Res<UiColors>,
) -> Entity {
    parent
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(20.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                position_type: PositionType::Relative,
                ..default()
            },
            background_color: Color::rgb(0.65, 0.65, 0.65).into(),
            ..default()
        })
        .with_children(|parent| {
            // Bottom meta options
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        justify_content: JustifyContent::FlexEnd,
                        position_type: PositionType::Relative,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    background_color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Auto),
                                padding: UiRect::all(Val::Px(15.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            background_color: colors.button_standard.into(),
                            ..Default::default()
                        })
                        .insert(BarterButtonProps {
                            barter_button_type: BarterTypes::Bully,
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text {
                                    sections: vec![TextSection {
                                        value: BarterTypes::get_string_name(BarterTypes::Bully),
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

                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Auto),
                                padding: UiRect::all(Val::Px(15.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            background_color: colors.button_standard.into(),
                            ..Default::default()
                        })
                        .insert(BarterButtonProps {
                            barter_button_type: BarterTypes::Plea,
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text {
                                    sections: vec![TextSection {
                                        value: BarterTypes::get_string_name(BarterTypes::Plea),
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

                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Auto),
                                padding: UiRect::all(Val::Px(15.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            background_color: colors.button_standard.into(),
                            ..Default::default()
                        })
                        .insert(BarterButtonProps {
                            barter_button_type: BarterTypes::Persuade,
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text {
                                    sections: vec![TextSection {
                                        value: BarterTypes::get_string_name(BarterTypes::Persuade),
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
        })
        .id()
}

fn setup_middle_barter_screen(
    parent: &mut ChildBuilder,
    font_assets: &Res<FontAssets>,
    colors: &Res<UiColors>,
) -> Entity {
    parent
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(60.0), Val::Percent(100.0)),
                justify_content: JustifyContent::FlexEnd,
                position_type: PositionType::Relative,
                flex_direction: FlexDirection::Column,
                overflow: Overflow::Hidden,
                ..default()
            },
            background_color: Color::rgb(0.0, 0.65, 0.65).into(),
            ..default()
        })
        .insert(ResolutionUiParent)
        .id()
}

fn setup_right_barter_screen(
    parent: &mut ChildBuilder,
    font_assets: &Res<FontAssets>,
    colors: &Res<UiColors>,
) -> Entity {
    let entity = parent
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(20.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                position_type: PositionType::Relative,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::rgb(0.65, 0.65, 0.65).into(),
            ..default()
        })
        .with_children(|parent| {
            // top right side stuff
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        justify_content: JustifyContent::FlexStart,
                        position_type: PositionType::Relative,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    background_color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Auto),
                                padding: UiRect::all(Val::Px(15.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            background_color: colors.button_standard.into(),
                            ..Default::default()
                        })
                        .insert(BarterControlButtonProps {
                            control_button_type: BarterResolutionTypes::Approve { amount: 0 },
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text {
                                    sections: vec![TextSection {
                                        value: "Approve".to_string(),
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

                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Auto),
                                padding: UiRect::all(Val::Px(15.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            background_color: colors.button_standard.into(),
                            ..Default::default()
                        })
                        .insert(BarterControlButtonProps {
                            control_button_type: BarterResolutionTypes::Deny,
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text {
                                    sections: vec![TextSection {
                                        value: "Deny".to_string(),
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
            // Bottom meta options
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        justify_content: JustifyContent::FlexEnd,
                        position_type: PositionType::Relative,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    background_color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Auto),
                                padding: UiRect::all(Val::Px(15.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            background_color: colors.button_standard.into(),
                            ..Default::default()
                        })
                        .insert(BarterControlButtonProps {
                            control_button_type: BarterResolutionTypes::Approve { amount: 0 },
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text {
                                    sections: vec![TextSection {
                                        value: "Approve".to_string(),
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

                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Auto),
                                padding: UiRect::all(Val::Px(15.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            background_color: colors.button_standard.into(),
                            ..Default::default()
                        })
                        .insert(BarterControlButtonProps {
                            control_button_type: BarterResolutionTypes::Deny,
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text {
                                    sections: vec![TextSection {
                                        value: "Deny".to_string(),
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
        })
        .id();

    entity
}

fn spawn_barter_result(
    parent: Query<Entity, With<ResolutionUiParent>>,
    mut commands: Commands,
    mut result: EventReader<BarterAttemptResultEvent>,
    font_assets: Res<FontAssets>,
    colors: Res<UiColors>,
) {
    let parent = parent.single();

    commands.entity(parent).with_children(|parent| {
        for event in result.iter() {
            match event.result {
                BarterAttemptResult::Success => {
                    // spawn in two parts so that its split up and we have a border
                    parent
                        // main holder node
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Auto),
                                margin: UiRect::all(Val::Px(20.0)),
                                justify_content: JustifyContent::FlexStart,
                                align_items: AlignItems::Center,
                                position_type: PositionType::Relative,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                // big holder that is hidden for background border
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(65.0), Val::Percent(100.0)),
                                        justify_content: JustifyContent::SpaceBetween,
                                        position_type: PositionType::Relative,
                                        ..default()
                                    },
                                    background_color: Color::rgb(1.0, 1.0, 1.0).into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    // actual interior that we want to use
                                    parent
                                        .spawn(NodeBundle {
                                            style: Style {
                                                size: Size::new(Val::Percent(95.0), Val::Percent(95.0)),
                                                justify_content: JustifyContent::SpaceBetween,
                                                flex_direction: FlexDirection::Column,
                                                position_type: PositionType::Relative,
                                                ..default()
                                            },
                                            background_color: Color::rgb(1.0, 1.0, 1.0).into(),
                                            ..default()
                                        })
                                        .with_children(|parent| {
                                            // Result text
                                            parent
                                                .spawn(NodeBundle {
                                                    style: Style {
                                                        size: Size::new(Val::Percent(100.0), Val::Auto),
                                                        justify_content: JustifyContent::FlexStart,
                                                        position_type: PositionType::Relative,
                                                        ..default()
                                                    },
                                                    background_color: colors.success.into(),
                                                    ..default()
                                                })
                                                .with_children(|parent| {
                                                    parent.spawn(TextBundle {
                                                        text: Text {
                                                            sections: vec![TextSection {
                                                                value: String::from(format!(
                                                                    "{} Success!",
                                                                    event
                                                                        .attempt_type
                                                                        .get_string_name_from_instance()
                                                                )),
                                                                style: TextStyle {
                                                                    font: font_assets.fira_sans.clone(),
                                                                    font_size: 40.0,
                                                                    color: Color::rgb(1.0, 1.0, 1.0),
                                                                },
                                                            }],
                                                            alignment: Default::default(),
                                                        },
                                                        ..Default::default()
                                                    });
                                                });

                                            // amount
                                            parent
                                                .spawn(NodeBundle {
                                                    style: Style {
                                                        size: Size::new(Val::Percent(100.0), Val::Auto),
                                                        justify_content: JustifyContent::FlexStart,
                                                        position_type: PositionType::Relative,
                                                        ..default()
                                                    },
                                                    background_color: Color::rgb(1.0, 1.0, 1.0).into(),
                                                    ..default()
                                                })
                                                .with_children(|parent| {
                                                    parent.spawn(TextBundle {
                                                        text: Text {
                                                            sections: vec![TextSection {
                                                                value: String::from(format!(
                                                                    "New Price: {} gold",
                                                                    event.new_price
                                                                )),
                                                                style: TextStyle {
                                                                    font: font_assets.fira_sans.clone(),
                                                                    font_size: 40.0,
                                                                    color: Color::rgb(0.0, 0.0, 0.0),
                                                                },
                                                            }],
                                                            alignment: Default::default(),
                                                        },
                                                        ..Default::default()
                                                    });
                                                });
                                        });
                                });
                        });
                }
                BarterAttemptResult::Failure => {}
            }
        }
    });
}

fn click_barter_control_button(
    mut commands: Commands,
    button_colors: Res<UiColors>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &BarterControlButtonProps,
        ),
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
        match props.control_button_type {
            BarterResolutionTypes::Approve { .. } => {
                if let Interaction::Clicked = interaction {
                    commands.insert_resource(NextState(UiState::Normal));
                }
            }
            BarterResolutionTypes::Deny => {
                if let Interaction::Clicked = interaction {
                    commands.insert_resource(NextState(UiState::Normal));
                }
            }
        }
    }
}

fn click_barter_button(
    mut commands: Commands,
    button_colors: Res<UiColors>,
    mut barter_attempt: EventWriter<BarterAttemptResultEvent>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &BarterButtonProps),
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
        match props.barter_button_type {
            BarterTypes::Bully => {
                if let Interaction::Clicked = interaction {
                    barter_attempt.send(BarterAttemptResultEvent {
                        result: BarterAttemptResult::Success,
                        attempt_type: BarterTypes::Bully,
                        new_price: 50,
                    })
                }
            }
            BarterTypes::Persuade => {
                if let Interaction::Clicked = interaction {
                    barter_attempt.send(BarterAttemptResultEvent {
                        result: BarterAttemptResult::Success,
                        attempt_type: BarterTypes::Persuade,
                        new_price: 50,
                    })
                }
            }
            BarterTypes::Plea => {
                if let Interaction::Clicked = interaction {
                    barter_attempt.send(BarterAttemptResultEvent {
                        result: BarterAttemptResult::Failure,
                        attempt_type: BarterTypes::Plea,
                        new_price: 40,
                    })
                }
            }
        }
    }
}

fn cleanup_barter_ui(mut commands: Commands, button: Query<Entity, With<BarterUi>>) {
    for button in button.iter() {
        commands.entity(button).despawn_recursive();
    }
}
