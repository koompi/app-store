// Library imports
use async_graphql::{EmptySubscription, Schema};

// Local imports
use crate::graphql::{RootMutation, RootQuery};

pub type MainSchema = Schema<RootQuery, RootMutation, EmptySubscription>;
pub struct Token(pub String);
