use bevy::app::AppExit;
use crate::loading::FontAssets;
use crate::ui::{UiColors};
use crate::GameState;
use bevy::prelude::*;
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet};
use iyes_loopless::state::NextState;
use kayak_ui::prelude::{widgets::*, *};

pub struct MenuPlugin;

/// 
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::MainMenu, setup_menu)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::MainMenu)
                    .into(),
            );
    }
}

#[derive(Component, Clone, PartialEq, Default)]
pub struct MenuButtonProps {
    menu_button_type: MenuButtonType,
    background_color: Color,
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

impl Widget for MenuButtonProps {}

#[derive(Bundle)]
pub struct MenuButtonBundle {
    pub props: MenuButtonProps,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub children: KChildren,
    // This allows us to hook into on click events!
    pub on_event: OnEvent,
    // Widget name is required by Kayak UI!
    pub widget_name: WidgetName,
}

impl Default for MenuButtonBundle {
    fn default() -> Self {
        Self {
            props: MenuButtonProps::default(),
            styles: KStyle::default(),
            computed_styles: ComputedStyles::default(),
            children: KChildren::default(),
            on_event: OnEvent::default(),
            // Kayak uses this component to find out more information about your widget.
            // This is done because bevy does not have the ability to query traits.
            widget_name: MenuButtonProps::default().get_name(),
        }
    }
}

pub fn menu_button_render(
    In((mut widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    mut commands: Commands,
    colors: Res<UiColors>,
    mut menu_button_query: Query<&mut MenuButtonProps>,
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
              mut menu_button_query: Query<&mut MenuButtonProps>,
              mut commands: Commands,
              mut exit_event: EventWriter<AppExit>,
        | {
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
                        match button_props.menu_button_type {
                            MenuButtonType::NewGame => {
                                commands.insert_resource(NextState(GameState::Playing))
                            }
                            MenuButtonType::Options => {}
                            MenuButtonType::Exit => {
                                exit_event.send(AppExit);
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
            width: StyleProp::from(Units::Pixels(100.0)),
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

fn setup_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    colors: Res<UiColors>,
    mut widget_context: Query<&mut KayakRootContext>,
) {
    let mut widget_context = widget_context.single_mut();

    widget_context.add_widget_data::<MenuButtonProps, ButtonState>();
    // Next we need to add the systems
    widget_context.add_widget_system(
        // We are registering these systems with a specific WidgetName.
        MenuButtonProps::default().get_name(),
        // widget_update auto diffs props and state.
        // Optionally if you have context you can use: widget_update_with_context
        // otherwise you will need to create your own widget update system!
        widget_update::<MenuButtonProps, ButtonState>,
        // Add our render system!
        menu_button_render,
    );
    let parent_id = None;
    let background_styles = KStyle {
        background_color: StyleProp::Value(colors.background_standard),
        border_radius: Corner::all(50.0).into(),
        width: StyleProp::from(Units::Pixels(350.0)),
        height: StyleProp::from(Units::Pixels(512.0)),
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

    // We can now create our widget like:
    rsx! {
        <KayakAppBundle>
            <BackgroundBundle
                styles={background_styles}
            >
                <MenuButtonBundle
                    props={
                        MenuButtonProps{
                            menu_button_type: MenuButtonType::NewGame,
                            background_color: colors.button_standard,
                        }
                    }
                />
                <MenuButtonBundle
                    props={
                        MenuButtonProps{
                            menu_button_type: MenuButtonType::Options,
                            background_color: colors.button_standard,

                        }
                    }
                />
                <MenuButtonBundle
                    props={
                        MenuButtonProps{
                            menu_button_type: MenuButtonType::Exit,
                            background_color: colors.button_standard,

                        }
                    }
                />
            </BackgroundBundle>
        </KayakAppBundle>
    };
}