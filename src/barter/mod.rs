//! Barters are separated between two main things. A Barter and Haggle
//! A Barter is the entire barter, trying to sell an item, attempting to barter, etc
//! A Haggle is an individual attempt to adjust the price

use crate::barter::customers::{CustomerHandler, CustomerPlugin, CustomerState};
use crate::ui::UiState;
use bevy::prelude::{App, Commands, EventReader, EventWriter, Plugin, ResMut, Resource};
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet, NextState};
use rand::{thread_rng, Rng};

pub mod customers;

pub struct BarterPlugin;

impl Plugin for BarterPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CustomerHandler::default())
            .add_loopless_state(BarterState::NotBartering)
            .add_event::<BeginBarter>()
            .add_event::<EndBarter>()
            .add_event::<BarterResolved>()
            .add_event::<HaggleAttemptEvent>()
            .add_event::<HaggleResultEvent>();

        app.add_plugin(CustomerPlugin);

        app.add_system_set(
            ConditionSet::new()
                .run_in_state(BarterState::Bartering)
                .with_system(handle_bartering)
                .into(),
        )
        .add_enter_system(BarterState::Bartering, begin_barter)
        .add_exit_system(BarterState::Bartering, end_barter)
        .add_system_set(
            ConditionSet::new()
                .label("handle_haggle_attempt_events")
                .run_on_event::<HaggleAttemptEvent>()
                .with_system(handle_haggle_attempt_events)
                .into(),
        );
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum BarterState {
    Bartering,
    NotBartering,
}

pub struct BeginBarter;

pub struct EndBarter;

#[derive(Clone, PartialEq, Eq, Hash, Resource)]
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
    
    pub fn get_price(&self) -> u32{
        self.sell_price
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

fn handle_haggle_attempt_events(
    mut events: EventReader<HaggleAttemptEvent>,
    mut results: EventWriter<HaggleResultEvent>,
    mut barter: ResMut<Barter>,
) {
    for event in events.iter() {
        let mut rng = thread_rng();
        let chance = rng.gen_range(0..=1);
        let result = match chance {
            0 => {
                barter.sell_price = barter.sell_price.saturating_add(10);
                HaggleResultEvent {
                    result: HaggleResult::Success,
                    attempt_type: event.attempt_type.clone(),
                    new_price: barter.sell_price,
                }
            }
            _ => {
                barter.sell_price = barter.sell_price.saturating_sub(10);
                HaggleResultEvent {
                    result: HaggleResult::Failure,
                    attempt_type: event.attempt_type.clone(),
                    new_price: barter.sell_price,
                }
            }
        };

        barter.log_result(result.clone());
        results.send(result)
    }
}

fn begin_barter(mut commands: Commands) {
    let mut rng = thread_rng();
    let price = rng.gen_range(35..50);

    commands.insert_resource(Barter {
        sell_price: price,
        haggles: vec![],
    });
    commands.insert_resource(NextState(UiState::Barter));
}

fn end_barter(mut commands: Commands, mut customer_handler: ResMut<CustomerHandler>) {
    commands.insert_resource(NextState(UiState::Normal));
    commands
        .entity(
            customer_handler
                .get_next_customer()
                .expect("If we are in barter we should always have a customer"),
        )
        .insert(CustomerState::Despawning);
    customer_handler.remove_customer_at_index(0);
}
