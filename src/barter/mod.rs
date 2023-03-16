//! Barters are separated between two main things. A Barter and Haggle
//! A Barter is the entire barter, trying to sell an item, attempting to barter, etc
//! A Haggle is an individual attempt to adjust the price

use crate::barter::customers::{CustomerHandler, CustomerPlugin};
use crate::ui::UiState;
use crate::PausedState;
use bevy::prelude::{App, Plugin};
use iyes_loopless::prelude::ConditionSet;

pub mod customers;

pub struct BarterPlugin;

impl Plugin for BarterPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CustomerHandler::default())
            .add_event::<BeginBarter>()
            .add_event::<EndBarter>()
            .add_event::<BarterResolved>()
            .add_event::<HaggleAttemptEvent>()
            .add_event::<HaggleResultEvent>();

        app.add_plugin(CustomerPlugin);

        app.add_system_set(
            ConditionSet::new()
                .run_in_state(PausedState::Playing)
                .with_system(handle_bartering)
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .label("handle_haggle_attempt_events")
                .run_on_event::<HaggleAttemptEvent>()
                .with_system(handle_haggle_attempt_events)
                .into(),
        );
    }
}

pub enum BarterState{
    Bartering,
    NotBartering
}

pub struct BeginBarter;

pub struct EndBarter;

pub struct Barter {
    sell_price: u32,
    haggles: Vec<HaggleResultEvent>,
}

impl Barter {
    pub fn log_result(&mut self, haggle_result: HaggleResultEvent) {
        self.update_price(haggle_result.new_price);
        self.haggles.push(haggle_result);
    }
    pub fn update_price(&mut self, price: u32) {
        self.sell_price = price;
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum HaggleType {
    Plea,
    Persuade,
    Bully,
}

impl HaggleType {
    pub fn get_string_name(barter_type: HaggleType) -> String {
        return match barter_type {
            HaggleType::Bully => String::from("Bully"),
            HaggleType::Persuade => String::from("Persuade"),
            HaggleType::Plea => String::from("Plea"),
        };
    }

    pub fn get_string_name_from_instance(&self) -> String {
        return match self {
            HaggleType::Bully => String::from("Bully"),
            HaggleType::Persuade => String::from("Persuade"),
            HaggleType::Plea => String::from("Plea"),
        };
    }
}

/// This is an attempt. The player clicks the corresponding button and wants to try and plead or whatever
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct HaggleAttemptEvent {
    pub attempt_type: HaggleType,
}

/// This is the result of a haggle attempt event
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct HaggleResultEvent {
    pub result: HaggleResult,
    pub attempt_type: HaggleType,
    pub new_price: u32,
}

/// This is an enum used to match the the result of a haggle attempt event
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum HaggleResult {
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

fn handle_bartering() {}

fn handle_haggle_attempt_events() {}

fn begin_barter() {}

fn end_barter() {}

