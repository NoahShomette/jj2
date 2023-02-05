use crate::barter::customers::{CustomerHandler, CustomerPlugin};
use crate::ui::game_scene::barter_screen::BarterButtonProps;
use bevy::prelude::{App, Plugin};

pub mod customers;

pub struct BarterPlugin;

impl Plugin for BarterPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CustomerHandler::default())
            .add_event::<BarterResolved>()
            .add_event::<BarterAttemptEvent>()
            .add_event::<BarterAttemptResultEvent>();

        app.add_plugin(CustomerPlugin);
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum BarterTypes {
    Plea,
    Persuade,
    Bully,
}

impl BarterTypes {
    pub fn get_string_name(barter_type: BarterTypes) -> String {
        return match barter_type {
            BarterTypes::Bully => String::from("Bully"),
            BarterTypes::Persuade => String::from("Persuade"),
            BarterTypes::Plea => String::from("Plea"),
        };
    }

    pub fn get_string_name_from_instance(&self) -> String {
        return match self {
            BarterTypes::Bully => String::from("Bully"),
            BarterTypes::Persuade => String::from("Persuade"),
            BarterTypes::Plea => String::from("Plea"),
        };
    }
}

/// This is an attempt. The player clicks the corresponding button and wants to try and plead or whatever
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BarterAttemptEvent {
    pub attempt_type: BarterTypes,
}

/// This is the result of a barter attempt event
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BarterAttemptResultEvent {
    pub result: BarterAttemptResult,
    pub attempt_type: BarterTypes,
    pub new_price: u32,
}

/// This is an enum used to match the the result of a barter attempt event
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum BarterAttemptResult {
    Success,
    Failure,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BarterResolved {
    pub(crate) resolution_type: BarterResolutionTypes,
}

#[derive(Clone, PartialEq, Default, Eq, Hash)]
pub enum BarterResolutionTypes {
    Approve {
        amount: u32,
    },
    #[default]
    Deny,
}
