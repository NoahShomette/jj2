use bevy::prelude::{Bundle, Component, Entity, Resource};
use rand::Rng;


#[derive(Resource, Default)]
pub struct CustomerHandler{
    pub active_customers: Vec<Entity>,
}

#[derive(Bundle)]
pub struct CustomerBundle {
    pub customer: Customer,
    pub name: Name,
    pub max_purchase_amount: MaxPurchaseAmount,
    pub customer_difficulty: CustomerDifficulty,
}

impl CustomerBundle {
    pub fn new_random() -> CustomerBundle {
        let mut rng = rand::thread_rng();

        CustomerBundle {
            customer: Customer,
            name: Name {
                // TODO - generate random names somehow
                name: "Bob".to_string(),
            },
            max_purchase_amount: MaxPurchaseAmount {
                max: rng.gen_range(100..500),
            },
            customer_difficulty: CustomerDifficulty {
                bully_resistance: rng.gen_range(25..75),
                persuade_resistance: rng.gen_range(25..75),
                plead_resistance: rng.gen_range(25..75),
            },
        }
    }
}

#[derive(Component)]
pub struct Customer;

#[derive(Component)]
pub struct Name {
    pub name: String,
}

#[derive(Component)]
pub struct MaxPurchaseAmount {
    pub max: u32,
}

#[derive(Component)]
pub struct CustomerDifficulty {
    pub bully_resistance: i32,
    pub persuade_resistance: i32,
    pub plead_resistance: i32,
}
