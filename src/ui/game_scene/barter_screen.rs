use crate::barter::{BarterResolutionTypes, BarterResolved};
use crate::ui::game_scene::scene_ui::{GameMainButtonProps, GameStateButtons};
use crate::ui::game_scene::UiState;
use crate::ui::UiColors;
use crate::PausedState;
use bevy::app::AppExit;
use bevy::prelude::{
    App, Bundle, Color, Commands, Component, Entity, EventWriter, In, Plugin, Query, Res,
};
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet, NextState};
use kayak_ui::prelude::{widgets::*, *};

pub struct BarterUiPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for BarterUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(UiState::Barter, setup_barter_ui)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(UiState::Barter)
                    .run_in_state(PausedState::Playing)
                    .into(),
            );
    }
}

#[derive(Component, Clone, PartialEq, Default)]
pub struct BarterButtonProps {
    barter_button_type: BarterButtonType,
    background_color: Color,
}

impl Widget for BarterButtonProps {}

impl BarterButtonProps {
    pub fn get_button_text(&self) -> String {
        return match self.barter_button_type {
            BarterButtonType::Bully => String::from("Bully"),
            BarterButtonType::Persuade => String::from("Persuade"),
            BarterButtonType::Plea => String::from("Plea"),
        };
    }
}

#[derive(Clone, PartialEq, Default)]
pub enum BarterButtonType {
    #[default]
    Bully,
    Persuade,
    Plea,
}

#[derive(Bundle)]
pub struct BarterButtonBundle {
    pub props: BarterButtonProps,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub children: KChildren,
    // This allows us to hook into on click events!
    pub on_event: OnEvent,
    // Widget name is required by Kayak UI!
    pub widget_name: WidgetName,
}

impl Default for BarterButtonBundle {
    fn default() -> Self {
        Self {
            props: BarterButtonProps::default(),
            styles: KStyle::default(),
            computed_styles: ComputedStyles::default(),
            children: KChildren::default(),
            on_event: OnEvent::default(),
            // Kayak uses this component to find out more information about your widget.
            // This is done because bevy does not have the ability to query traits.
            widget_name: BarterButtonProps::default().get_name(),
        }
    }
}

fn barter_button_render(
    In((mut widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    mut commands: Commands,
    colors: Res<UiColors>,
    mut menu_button_query: Query<&mut BarterButtonProps>,
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
              mut menu_button_query: Query<&mut BarterButtonProps>,
              mut commands: Commands,
              mut exit_event: EventWriter<AppExit>| {
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
                        match button_props.barter_button_type {
                            BarterButtonType::Bully => {}
                            BarterButtonType::Persuade => {}
                            BarterButtonType::Plea => {}
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

#[derive(Component, Clone, PartialEq, Default)]
pub struct BarterControlButtonProps {
    control_button_type: BarterResolutionTypes,
    background_color: Color,
}

impl Widget for BarterControlButtonProps {}

impl BarterControlButtonProps {
    pub fn get_button_text(&self) -> String {
        return match self.control_button_type {
            BarterResolutionTypes::Approve { amount } => String::from("Approve"),
            BarterResolutionTypes::Deny => String::from("Deny"),
        };
    }
}

#[derive(Bundle)]
pub struct BarterControlButtonBundle {
    pub props: BarterControlButtonProps,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub children: KChildren,
    // This allows us to hook into on click events!
    pub on_event: OnEvent,
    // Widget name is required by Kayak UI!
    pub widget_name: WidgetName,
}

impl Default for BarterControlButtonBundle {
    fn default() -> Self {
        Self {
            props: BarterControlButtonProps::default(),
            styles: KStyle::default(),
            computed_styles: ComputedStyles::default(),
            children: KChildren::default(),
            on_event: OnEvent::default(),
            // Kayak uses this component to find out more information about your widget.
            // This is done because bevy does not have the ability to query traits.
            widget_name: BarterControlButtonProps::default().get_name(),
        }
    }
}

fn control_button_render(
    In((mut widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    mut commands: Commands,
    colors: Res<UiColors>,
    mut menu_button_query: Query<&mut BarterControlButtonProps>,
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
              mut menu_button_query: Query<&mut BarterControlButtonProps>,
              mut commands: Commands,
              mut barter_resolution_event: EventWriter<BarterResolved>,
              mut exit_event: EventWriter<AppExit>| {
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
                        match button_props.control_button_type {
                            BarterResolutionTypes::Approve { amount } => {
                                barter_resolution_event.send(BarterResolved {
                                    resolution_type: BarterResolutionTypes::Approve {
                                        amount: amount,
                                    },
                                });
                                commands.insert_resource(NextState(UiState::Normal));
                            }
                            BarterResolutionTypes::Deny => {
                                barter_resolution_event.send(BarterResolved {
                                    resolution_type: BarterResolutionTypes::Deny {},
                                });
                                commands.insert_resource(NextState(UiState::Normal));
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

pub fn setup_barter_ui(
    mut commands: Commands,
    colors: Res<UiColors>,
    mut widget_context: Query<&mut KayakRootContext>,
) {
    let mut widget_context = widget_context.single_mut();

    widget_context.add_widget_data::<BarterButtonProps, ButtonState>();
    // Next we need to add the systems
    widget_context.add_widget_system(
        // We are registering these systems with a specific WidgetName.
        BarterButtonProps::default().get_name(),
        // widget_update auto diffs props and state.
        // Optionally if you have context you can use: widget_update_with_context
        // otherwise you will need to create your own widget update system!
        widget_update::<BarterButtonProps, ButtonState>,
        // Add our render system!
        barter_button_render,
    );

    widget_context.add_widget_data::<BarterControlButtonProps, ButtonState>();
    // Next we need to add the systems
    widget_context.add_widget_system(
        // We are registering these systems with a specific WidgetName.
        BarterControlButtonProps::default().get_name(),
        // widget_update auto diffs props and state.
        // Optionally if you have context you can use: widget_update_with_context
        // otherwise you will need to create your own widget update system!
        widget_update::<BarterControlButtonProps, ButtonState>,
        // Add our render system!
        control_button_render,
    );

    let background_style = KStyle {
        background_color: StyleProp::Value(colors.background_standard),
        border_radius: Corner::all(50.0).into(),
        width: StyleProp::from(Units::Percentage(75.)),
        height: StyleProp::from(Units::Percentage(75.)),
        left: StyleProp::from(Units::Stretch(1.0)),
        right: StyleProp::from(Units::Stretch(1.0)),
        top: StyleProp::from(Units::Stretch(1.0)),
        bottom: StyleProp::from(Units::Stretch(1.0)),
        padding: Edge::new(
            Units::Pixels(20.0),
            Units::Pixels(20.0),
            Units::Pixels(20.0),
            Units::Pixels(20.0),
        )
        .into(),
        ..KStyle::default()
    };

    let parent_id = None;
    rsx! {
        <KayakAppBundle>
        <BackgroundBundle
                styles={background_style.clone()}
            >
                <BarterControlButtonBundle
                    props={
                        BarterControlButtonProps{
                            //TODO Update this to take the current price and use it to display how much
                            control_button_type: BarterResolutionTypes::Approve {amount: 0},
                            background_color: colors.button_standard,
                        }
                    }
                />
                <BarterControlButtonBundle
                    props={
                        BarterControlButtonProps{
                            control_button_type: BarterResolutionTypes::Deny{},
                            background_color: colors.button_standard,
                        }
                    }
                />
            </BackgroundBundle>
           <BackgroundBundle
                styles={background_style.clone()}
            >
                <BarterButtonBundle
                    props={
                        BarterButtonProps{
                            barter_button_type: BarterButtonType::Bully,
                            background_color: colors.button_standard,
                        }
                    }
                />
                <BarterButtonBundle
                    props={
                        BarterButtonProps{
                            barter_button_type: BarterButtonType::Persuade,
                            background_color: colors.button_standard,
                        }
                    }
                />
                <BarterButtonBundle
                    props={
                        BarterButtonProps{
                            barter_button_type: BarterButtonType::Plea,
                            background_color: colors.button_standard,
                        }
                    }
                />
            </BackgroundBundle>

        </KayakAppBundle>
    };
}
