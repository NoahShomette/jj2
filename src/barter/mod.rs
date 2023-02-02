use crate::barter::customers::CustomerHandler;
use bevy::prelude::{App, Plugin};

pub mod customers;

pub struct BarterPlugin;

impl Plugin for BarterPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CustomerHandler::default())
            .add_event::<BarterResolved>();
    }
}



#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BarterResolved{
    pub(crate) resolution_type: BarterResolutionTypes,
}

#[derive(Clone, PartialEq, Default, Eq, Hash)]
pub enum BarterResolutionTypes {
    Approve{
        amount: u32,
    },
    #[default]
    Deny,
}