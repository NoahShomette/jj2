use crate::barter::BarterAttemptResultEvent;
use crate::loading::CustomerAsepriteHandles;
use crate::player::Player;
use crate::{loading, GameState, PausedState};
use bevy::app::App;
use bevy::math::Vec3;
use bevy::prelude::{Assets, Bundle, Commands, Component, Entity, Mut, Plugin, Query, Res, ResMut, Resource, TextureAtlasSprite, Time, Transform, With};
use bevy::utils::default;
use bevy_mod_aseprite::{aseprite, Aseprite, AsepriteAnimation, AsepriteBundle};
use iyes_loopless::prelude::ConditionSet;
use rand::{thread_rng, Rng};


pub struct CustomerPlugin;

impl Plugin for CustomerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CustomerHandler>()
            .init_resource::<CustomerSettings>();

        app.add_system_set(
            ConditionSet::new()
                .label("customer_spawner")
                .run_in_state(GameState::Playing)
                .run_in_state(PausedState::Playing)
                .with_system(spawn_customers_if_below_max_num)
                .with_system(move_customers)
                .into(),
        );
    }
}

pub const MAX_LEFT_CUSTOMER_BOUNDS: f32 = -120.0;
pub const MAX_RIGHT_CUSTOMER_BOUNDS: f32 = 120.0;
pub const FLOOR_LEVEL: f32 = -46.0;

#[derive(Resource)]
pub struct CustomerSettings {
    pub max_num: u32,
}

impl Default for CustomerSettings {
    fn default() -> Self {
        CustomerSettings { max_num: 5 }
    }
}

#[derive(Resource, Default)]
pub struct CustomerHandler {
    pub active_customers: Vec<Entity>,
}

impl CustomerHandler {
    pub fn spawn_new_random_customer(
        &mut self,
        mut commands: &mut Commands,
        mut customer_aseprite_handles: &mut ResMut<CustomerAsepriteHandles>,
        aseprites: &Res<Assets<Aseprite>>,
    ) {
        let entity = commands
            .spawn(CustomerBundle::new_random(
                &mut customer_aseprite_handles,
                &aseprites,
            ))
            .id();
        self.active_customers.push(entity)
    }

    pub fn get_next_customer(&mut self) -> Option<Entity> {
        self.active_customers.first().cloned()
    }

    pub fn remove_customer_at_index(&mut self, i: usize) {
        self.active_customers.remove(i);
    }
}

#[derive(Bundle)]
pub struct CustomerBundle {
    pub customer: Customer,
    pub name: Name,
    pub max_purchase_amount: MaxPurchaseAmount,
    pub customer_difficulty: CustomerDifficulty,
    pub orientation: Orientation,
    pub aseprite_bundle: AsepriteBundle,
}

impl CustomerBundle {
    pub fn new_random(
        customer_aseprite_handles: &mut ResMut<CustomerAsepriteHandles>,
        aseprites: &Res<Assets<Aseprite>>,
    ) -> CustomerBundle {
        let mut rng = thread_rng();

        let handle_index = rng.gen_range(0..customer_aseprite_handles.len());

        let aseprite_handle = &customer_aseprite_handles[handle_index];
        let sprite = aseprites.get(aseprite_handle).unwrap();
        let animation = AsepriteAnimation::new(sprite.info(), "idle");
        
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
            orientation: Orientation::Spawned,
            aseprite_bundle: AsepriteBundle {
                aseprite: aseprite_handle.clone_weak(),
                sprite: TextureAtlasSprite::new(animation.current_frame()),
                animation,
                texture_atlas: sprite.atlas().clone_weak(),
                transform: Transform {
                    translation: Vec3 {
                        x: 0.0,
                        y: 200.0,
                        z: 1.0,
                    },
                    ..default()
                },
                global_transform: Default::default(),
                visibility: Default::default(),
                computed_visibility: Default::default(),
            },
        }
    }
}

#[derive(Component)]
pub struct Customer;

#[derive(Component)]
pub struct IsActiveCustomer;

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

#[derive(Component, Debug, Clone, Copy)]
pub enum Orientation {
    Left,
    Right,
    Spawned,
}

pub fn spawn_customers_if_below_max_num(
    mut customers: ResMut<CustomerHandler>,
    mut commands: Commands,
    mut settings: ResMut<CustomerSettings>,
    mut customer_aseprite_handles: ResMut<CustomerAsepriteHandles>,
    aseprites: Res<Assets<Aseprite>>,
) {

    if customers.active_customers.len() < settings.max_num as usize {
        println!("spawning customers");
        customers.spawn_new_random_customer(
            &mut commands,
            &mut customer_aseprite_handles,
            &aseprites,
        );
    }
}

pub fn move_customers(
    mut customers: Query<(&mut Transform, &mut Orientation), With<Customer>>,
    time: Res<Time>,
) {
    for (mut transform, mut orientation) in customers.iter_mut() {
        
        println!("{}", transform.translation);
        match &mut *orientation {
            Orientation::Left => {
                if transform.translation.x > MAX_LEFT_CUSTOMER_BOUNDS {
                    transform.translation.x -= 35.0 * time.delta_seconds();
                } else {
                    *orientation = Orientation::Right;
                    transform.translation.x += 35.0 * time.delta_seconds();
                }
            }
            Orientation::Right => {
                if transform.translation.x < MAX_RIGHT_CUSTOMER_BOUNDS {
                    transform.translation.x += 35.0 * time.delta_seconds();
                } else {
                    *orientation = Orientation::Left;
                    transform.translation.x -= 35.0 * time.delta_seconds();
                }
            }

            Orientation::Spawned => {
                if transform.translation.y > FLOOR_LEVEL {
                    transform.translation.y -= 250.0 * time.delta_seconds();
                } else {
                    let mut rng = thread_rng();
                    let number = rng.gen_range(0..1);
                    if number == 0 {
                        *orientation = Orientation::Right;
                    } else {
                        *orientation = Orientation::Left;
                    }
                }
            }
        }
    }
}
