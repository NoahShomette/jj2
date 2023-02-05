use crate::barter::{
    BarterAttemptResult, BarterAttemptResultEvent, BarterResolutionTypes,
    BarterTypes,
};
use crate::loading::FontAssets;
use crate::ui::{UiColors, UiState};
use crate::{PausedState};
use bevy::prelude::*;
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet, NextState};

use bevy_tweening::lens::UiPositionLens;
use bevy_tweening::{Animator, EaseFunction, Tween, TweenCompleted};

pub struct BarterUiPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for BarterUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CloseBarterUi>()
            .add_enter_system(UiState::Barter, setup_barter_ui)
            .add_exit_system(UiState::Barter, cleanup_barter_ui)
            .add_system_set(
                ConditionSet::new()
                    .label("spawn_cards")
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
                    .with_system(tween_out_barter_ui)
                    .with_system(cleanup_barter_ui)
                    .into(),
            );
    }
}

const BARTER_UI_TRANSITION_DONE: u64 = 1;
const BARTER_RESOLUTION_CARD_TRANSITION_DONE: u64 = 2;

#[derive(Component, Clone, PartialEq, Default)]
pub struct ResolutionUiParent;

#[derive(Clone, PartialEq, Default)]
pub struct CloseBarterUi;

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

#[derive(Component, Default, PartialEq, Clone)]
pub struct BarterResultCard;

fn setup_barter_ui(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    colors: Res<UiColors>,
    mut windows: ResMut<Windows>,
) {
    let window = windows.primary();
    let tween = Tween::new(
        EaseFunction::QuadraticIn,
        std::time::Duration::from_secs_f32(0.7),
        UiPositionLens {
            start: UiRect {
                left: Val::Auto,
                top: Val::Auto,
                right: Val::Auto,
                bottom: Val::Px(-window.height()),
            },
            end: UiRect {
                left: Val::Auto,
                top: Val::Auto,
                right: Val::Auto,
                bottom: Val::Px(0.0),
            },
        },
    );

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
        .insert(Animator::new(tween))
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
                    setup_middle_barter_screen(parent);

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

fn setup_middle_barter_screen(parent: &mut ChildBuilder) -> Entity {
    parent
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(60.0), Val::Percent(100.0)),
                justify_content: JustifyContent::FlexEnd,
                position_type: PositionType::Relative,
                flex_direction: FlexDirection::Column,
                overflow: Overflow::Hidden,
                flex_shrink: 0.0,
                ..default()
            },
            background_color: Color::rgb(0.1, 0.1, 0.1).into(),
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
                            background_color: colors.success.into(),
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
                            background_color: colors.failure.into(),
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
    mut query: Query<(Entity, &mut Style), With<BarterResultCard>>,
) {
    let parent = parent.single();

    for event in result.iter() {
        for (entity, mut style) in query.iter_mut() {
            let tween = Tween::new(
                EaseFunction::QuadraticIn,
                std::time::Duration::from_secs_f32(0.3),
                UiPositionLens {
                    start: UiRect {
                        left: Val::Auto,
                        top: Val::Auto,
                        right: Val::Auto,
                        bottom: Val::Px(-170.0),
                    },
                    end: UiRect {
                        left: Val::Auto,
                        top: Val::Auto,
                        right: Val::Auto,
                        bottom: Val::Px(0.0),
                    },
                },
            );

            style.position = UiRect {
                left: Val::Auto,
                top: Val::Auto,
                right: Val::Auto,
                bottom: Val::Px(-170.0),
            };

            commands.entity(entity).insert(Animator::new(tween));
        }

        commands.entity(parent).with_children(|parent| {
            let attempt_type_text = event.attempt_type.get_string_name_from_instance();
            let price_text = event.new_price;
            let mut color = colors.success;
            let mut result_text = String::from("Success");
            let mut justify_content_type: JustifyContent = JustifyContent::FlexStart;

            if let BarterAttemptResult::Failure = event.result {
                color = colors.failure;
                result_text = String::from("Failure");
                justify_content_type = JustifyContent::FlexEnd;
            }

            let tween = Tween::new(
                EaseFunction::QuadraticIn,
                std::time::Duration::from_secs_f32(0.3),
                UiPositionLens {
                    start: UiRect {
                        left: Val::Auto,
                        top: Val::Auto,
                        right: Val::Auto,
                        bottom: Val::Px(-170.0),
                    },
                    end: UiRect {
                        left: Val::Auto,
                        top: Val::Auto,
                        right: Val::Auto,
                        bottom: Val::Px(0.0),
                    },
                },
            );

            // spawn in two parts so that its split up and we have a border
            parent
                // main holder node
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Auto, Val::Px(130.0)),
                        margin: UiRect::all(Val::Px(20.0)),
                        justify_content: justify_content_type,
                        align_items: AlignItems::Center,
                        position_type: PositionType::Relative,
                        flex_shrink: 0.0,
                        position: UiRect {
                            left: Val::Auto,
                            top: Val::Auto,
                            right: Val::Auto,
                            bottom: Val::Px(-170.0),
                        },
                        ..default()
                    },
                    ..default()
                })
                .insert(BarterResultCard)
                .insert(Animator::new(tween))
                .with_children(|parent| {
                    parent
                        // big holder that is used as our border
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(65.0), Val::Percent(100.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                position_type: PositionType::Relative,
                                ..default()
                            },
                            background_color: color.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // actual interior that we want to use
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(95.0), Val::Percent(80.0)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
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
                                                padding: UiRect {
                                                    left: Val::Px(20.0),
                                                    right: Val::Px(20.0),
                                                    top: Val::Px(10.0),
                                                    bottom: Val::Px(20.0),
                                                },
                                                justify_content: JustifyContent::FlexStart,
                                                align_items: AlignItems::Center,
                                                position_type: PositionType::Relative,
                                                ..default()
                                            },
                                            background_color: color.into(),
                                            ..default()
                                        })
                                        .with_children(|parent| {
                                            parent.spawn(TextBundle {
                                                text: Text {
                                                    sections: vec![TextSection {
                                                        value: String::from(format!(
                                                            "{} {}!",
                                                            attempt_type_text, result_text
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
                                                padding: UiRect::all(Val::Px(20.0)),
                                                align_items: AlignItems::Center,
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
                                                            price_text
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
        });
    }
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
    mut close_ui: EventWriter<CloseBarterUi>,
) {
    for (interaction, mut color, props) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => match props.control_button_type {
                BarterResolutionTypes::Approve { .. } => {
                    *color = button_colors.success_hovered.into();
                }
                BarterResolutionTypes::Deny => {
                    *color = button_colors.failure_hovered.into();
                }
            },
            Interaction::None => match props.control_button_type {
                BarterResolutionTypes::Approve { .. } => {
                    *color = button_colors.success.into();
                }
                BarterResolutionTypes::Deny => {
                    *color = button_colors.failure.into();
                }
            },
            _ => {}
        }
        match props.control_button_type {
            BarterResolutionTypes::Approve { .. } => {
                if let Interaction::Clicked = interaction {
                    close_ui.send_default();
                }
            }
            BarterResolutionTypes::Deny => {
                if let Interaction::Clicked = interaction {
                    close_ui.send_default();
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

fn tween_out_barter_ui(
    mut commands: Commands,
    button: Query<Entity, With<BarterUi>>,
    mut close_ui: EventReader<CloseBarterUi>,
    mut windows: ResMut<Windows>,
) {
    let window = windows.primary();

    for event in close_ui.iter() {
        for button in button.iter() {
            let tween = Tween::new(
                EaseFunction::QuadraticIn,
                std::time::Duration::from_secs_f32(0.7),
                UiPositionLens {
                    start: UiRect {
                        left: Val::Auto,
                        top: Val::Auto,
                        right: Val::Auto,
                        bottom: Val::Px(0.0),
                    },
                    end: UiRect {
                        left: Val::Auto,
                        top: Val::Auto,
                        right: Val::Auto,
                        bottom: Val::Px(-window.height()),
                    },
                },
            )
            .with_completed_event(BARTER_UI_TRANSITION_DONE);
            commands.entity(button).insert(Animator::new(tween));
        }
    }
}

fn cleanup_barter_ui(
    mut commands: Commands,
    button: Query<Entity, With<BarterUi>>,
    mut reader: EventReader<TweenCompleted>,
) {
    for event in reader.iter() {
        if event.user_data == BARTER_UI_TRANSITION_DONE {
            for button in button.iter() {
                commands.entity(button).despawn_recursive();
                commands.insert_resource(NextState(UiState::Normal));
            }
        }
    }
}
