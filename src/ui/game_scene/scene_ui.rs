use crate::ui::game_scene::UiState;
use crate::ui::UiColors;
use crate::{GameState, PausedState};
use bevy::app::AppExit;
use bevy::prelude::{
    App, Bundle, Color, Commands, Component, Entity, EventWriter, In, Plugin, Query, Res,
};
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet};
use iyes_loopless::state::NextState;
use kayak_ui::prelude::{widgets::*, *};

pub struct GameSceneUiPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for GameSceneUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::Playing, setup_scene_ui)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Playing)
                    .run_in_state(PausedState::Playing)
                    .into(),
            );
    }
}

#[derive(Component, Clone, PartialEq, Default)]
pub struct GameMainButtonProps {
    game_button_type: GameStateButtons,
    background_color: Color,
}

impl Widget for GameMainButtonProps {}

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

#[derive(Bundle)]
pub struct GameMainButtonBundle {
    pub props: GameMainButtonProps,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub children: KChildren,
    // This allows us to hook into on click events!
    pub on_event: OnEvent,
    // Widget name is required by Kayak UI!
    pub widget_name: WidgetName,
}

impl Default for GameMainButtonBundle {
    fn default() -> Self {
        Self {
            props: GameMainButtonProps::default(),
            styles: KStyle::default(),
            computed_styles: ComputedStyles::default(),
            children: KChildren::default(),
            on_event: OnEvent::default(),
            // Kayak uses this component to find out more information about your widget.
            // This is done because bevy does not have the ability to query traits.
            widget_name: GameMainButtonProps::default().get_name(),
        }
    }
}

fn main_button_render(
    In((mut widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    mut commands: Commands,
    colors: Res<UiColors>,
    mut menu_button_query: Query<&mut GameMainButtonProps>,
    state_query: Query<&ButtonState>,
) -> bool {
    let state_entity =
        widget_context.use_state(&mut commands, entity, ButtonState { hovering: false });

    let mut button_props = menu_button_query.get_mut(entity).unwrap();
    let button_text = button_props.get_button_text();

    let on_event = OnEvent::new(
        move |In((event_dispatcher_context, _, mut event, _entity)): In<(
            EventDispatcherContext,
            WidgetState,
            Event,
            Entity,
        )>,
              mut query: Query<&mut ButtonState>,
              mut menu_button_query: Query<&mut GameMainButtonProps>,
              mut commands: Commands| {
            let button_props = menu_button_query.get_mut(entity).unwrap();

            if let Ok(mut button) = query.get_mut(state_entity) {
                match event.event_type {
                    EventType::MouseIn(..) => {
                        event.stop_propagation();
                        button.hovering = true;
                    }
                    EventType::MouseOut(..) => {
                        button.hovering = false;
                    }
                    EventType::Click(event) => {
                        match button_props.game_button_type {
                            GameStateButtons::Barter => {
                                commands.insert_resource(NextState(UiState::Barter));
                            }
                            GameStateButtons::Purchase => {
                                commands.insert_resource(NextState(UiState::Purchase));
                            }
                            GameStateButtons::Options => {
                                commands.insert_resource(NextState(PausedState::Paused));
                                commands.insert_resource(NextState(UiState::Pause));
                            }
                        };
                    }
                    _ => {}
                }
            }
            (event_dispatcher_context, event)
        },
    );

    if let Ok(button_state) = state_query.get(state_entity) {
        if button_state.hovering {
            button_props.background_color = colors.button_hovered
        } else {
            button_props.background_color = colors.button_standard
        };

        let button_style = KStyle {
            // Lets use red for our button background!
            background_color: StyleProp::Value(button_props.background_color),
            // 50 pixel border radius.
            border_radius: Corner::all(5.0).into(),
            width: StyleProp::from(Units::Pixels(120.0)),
            height: StyleProp::from(Units::Pixels(50.0)),
            left: StyleProp::from(Units::Stretch(1.0)),
            right: StyleProp::from(Units::Stretch(1.0)),
            top: StyleProp::from(Units::Stretch(1.0)),
            bottom: StyleProp::from(Units::Stretch(1.0)),
            layout_type: StyleProp::from(LayoutType::Column),
            ..Default::default()
        };

        let parent_id = Some(entity);

        rsx! {
            <BackgroundBundle
                styles={button_style}
                on_event={on_event}
            >
               <TextWidgetBundle
                    styles={KStyle {
                        top: Units::Stretch(1.0).into(),
                        bottom: Units::Stretch(1.0).into(),
                        ..Default::default()
                    }}
                    text={TextProps {
                        content: button_text,
                        alignment: Alignment::Middle,
                        size: 20.0,
                        ..Default::default()
                    }}
                />
            </BackgroundBundle>
        };
    }

    // The boolean returned here tells kayak UI to update the tree. You can avoid tree updates by
    // returning false, but in practice this should be done rarely. As kayak diff's the tree and
    // will avoid tree updates if nothing has changed!
    true
}

pub fn setup_scene_ui(
    mut commands: Commands,
    colors: Res<UiColors>,
    mut widget_context: Query<&mut KayakRootContext>,
) {
    let mut widget_context = widget_context.single_mut();

    widget_context.add_widget_data::<GameMainButtonProps, ButtonState>();
    // Next we need to add the systems
    widget_context.add_widget_system(
        // We are registering these systems with a specific WidgetName.
        GameMainButtonProps::default().get_name(),
        // widget_update auto diffs props and state.
        // Optionally if you have context you can use: widget_update_with_context
        // otherwise you will need to create your own widget update system!
        widget_update::<GameMainButtonProps, ButtonState>,
        // Add our render system!
        main_button_render,
    );
    let parent_id = None;
    rsx! {
        <KayakAppBundle>
            <GameMainButtonBundle
                props={
                    GameMainButtonProps{
                        game_button_type: GameStateButtons::Barter,
                        background_color: colors.button_standard,
                    }
                }
            />
            <GameMainButtonBundle
                props={
                    GameMainButtonProps{
                        game_button_type: GameStateButtons::Purchase,
                        background_color: colors.button_standard,
                    }
                }
            />
            <GameMainButtonBundle
                props={
                    GameMainButtonProps{
                        game_button_type: GameStateButtons::Options,
                        background_color: colors.button_standard,
                    }
                }
            />
        </KayakAppBundle>
    };
}
