use async_graphql::{EmptySubscription, Schema};

use futures::lock::Mutex;
use mongodb::{bson::doc, Client};
use slab::Slab;
use std::sync::Arc;
// MODELS
use super::{mutation::MutationRoot, query::QueryRoot};
use crate::models::user::User;
use serde::{Deserialize, Serialize};
pub type BooksSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[derive(Debug)]
pub struct DB {
    pub pool: Client,
}

#[derive(Debug, Serialize, Deserialize)]
struct Token {
    user_id: String,
}

pub type Storage = Arc<Mutex<Slab<User>>>;
