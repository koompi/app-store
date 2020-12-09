use super::{mutation::RootMutation, query::RootQuery};
use async_graphql::{EmptySubscription, Schema};

pub type MainSchema = Schema<RootQuery, RootMutation, EmptySubscription>;
pub struct Token(pub String);
