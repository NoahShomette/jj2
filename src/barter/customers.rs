use crate::loading::CustomerAsepriteHandles;
use crate::{GameState, PausedState};
use bevy::app::App;
use bevy::math::Vec3;
use bevy::prelude::{
    Assets, Bundle, Commands, Component, Entity, Local, Plugin, Query, Res, ResMut, Resource,
    TextureAtlasSprite, Time, Transform, With,
};
use bevy::time::{Timer, TimerMode};
use bevy::utils::default;
use bevy_mod_aseprite::{Aseprite, AsepriteAnimation, AsepriteBundle};
use iyes_loopless::prelude::ConditionSet;
use rand::{thread_rng, Rng};
use std::time::Duration;

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

pub const MAX_LEFT_BARTERING_BOUNDS: f32 = -80.0;
pub const MAX_RIGHT_BARTERING_BOUNDS: f32 = -85.0;

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
        commands: &mut Commands,
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

    pub fn get_next_customer(&self) -> Option<Entity> {
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
    pub orientation: CustomerState,
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
            orientation: CustomerState::Spawned,
            aseprite_bundle: AsepriteBundle {
                aseprite: aseprite_handle.clone_weak(),
                sprite: TextureAtlasSprite::new(animation.current_frame()),
                animation,
                texture_atlas: sprite.atlas().clone_weak(),
                transform: Transform {
                    translation: Vec3 {
                        x: 0.0,
                        y: 200.0,
                        z: 5.0,
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
pub enum CustomerState {
    MoveLeft,
    MoveRight,
    Spawned,
    Bartering,
    Despawning,
}

pub struct SpawnTimer {
    timer: Timer,
}

impl Default for SpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::new(Duration::from_secs_f32(1.0), TimerMode::Once),
        }
    }
}

pub fn spawn_customers_if_below_max_num(
    mut customers: ResMut<CustomerHandler>,
    mut commands: Commands,
    mut settings: ResMut<CustomerSettings>,
    mut customer_aseprite_handles: ResMut<CustomerAsepriteHandles>,
    aseprites: Res<Assets<Aseprite>>,
    mut spawn_timer: Local<SpawnTimer>,
    time: Res<Time>,
) {
    spawn_timer
        .timer
        .tick(Duration::from_secs_f32(time.delta_seconds()));
    if customers.active_customers.len() < settings.max_num as usize && spawn_timer.timer.finished()
    {
        println!("spawning customers");
        customers.spawn_new_random_customer(
            &mut commands,
            &mut customer_aseprite_handles,
            &aseprites,
        );
        spawn_timer.timer = Timer::new(Duration::from_secs_f32(5.0), TimerMode::Once);
    }
}

pub fn move_customers(
    mut customers: Query<
        (
            Entity,
            &mut Transform,
            &mut CustomerState,
            Option<&IsActiveCustomer>,
        ),
        With<Customer>,
    >,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut transform, mut orientation, is_active_customer) in customers.iter_mut() {
        match &mut *orientation {
            CustomerState::MoveLeft => {
                if let Some(_) = is_active_customer {
                    *orientation = CustomerState::Bartering;
                    return;
                }
                if transform.translation.x > MAX_LEFT_CUSTOMER_BOUNDS {
                    transform.translation.x -= 25.0 * time.delta_seconds();
                } else {
                    *orientation = CustomerState::MoveRight;
                    transform.translation.x += 25.0 * time.delta_seconds();
                }
            }
            CustomerState::MoveRight => {
                if let Some(_) = is_active_customer {
                    *orientation = CustomerState::Bartering;
                    return;
                }
                if transform.translation.x < MAX_RIGHT_CUSTOMER_BOUNDS {
                    transform.translation.x += 25.0 * time.delta_seconds();
                } else {
                    *orientation = CustomerState::MoveLeft;
                    transform.translation.x -= 25.0 * time.delta_seconds();
                }
            }

            CustomerState::Spawned => {
                if transform.translation.y > FLOOR_LEVEL {
                    transform.translation.y -= 250.0 * time.delta_seconds();
                } else {
                    let mut rng = thread_rng();
                    let number = rng.gen_range(0..=1);
                    if number == 0 {
                        *orientation = CustomerState::MoveRight;
                    } else {
                        *orientation = CustomerState::MoveLeft;
                    }
                }
            }
            CustomerState::Bartering => {
                if transform.translation.x > MAX_RIGHT_BARTERING_BOUNDS {
                    transform.translation.x -= 25.0 * time.delta_seconds();
                } else if transform.translation.x < MAX_LEFT_BARTERING_BOUNDS {
                    transform.translation.x += 25.0 * time.delta_seconds();
                }
            }
            CustomerState::Despawning => {
                if transform.translation.y > -200.0 {
                    transform.translation.y -= 250.0 * time.delta_seconds();
                } else {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}
