use super::user::User;
use crate::graphql::RootQuery;
use async_graphql::{Context, FieldResult, ID};
use bson::{self, oid::ObjectId};
use serde::{Deserialize, Serialize};
// use syn::Fields;
// use uuid::Uuid;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: ID,
    pub name: String,
    pub description: String,
    pub product_type: String,
    pub owner_id: String,
    pub date_created: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductModel {
    pub _id: ObjectId,
    pub name: String,
    pub description: String,
    pub product_type: String,
    pub owner_id: String,
    pub date_created: String,
    pub status: String,
}

impl Product {
    pub fn new() -> Self {
        Self {
            id: ID::from(""),
            name: String::from(""),
            description: String::from(""),
            product_type: String::from(""),
            owner_id: String::from(""),
            date_created: String::from(""),
            status: String::from(""),
        }
    }
}

impl ProductModel {
    pub fn new() -> Self {
        let converted_id = bson::oid::ObjectId::new();
        Self {
            _id: converted_id,
            name: String::from(""),
            description: String::from(""),
            product_type: String::from(""),
            owner_id: String::from(""),
            date_created: String::from(""),
            status: String::from(""),
        }
    }
    pub fn to_norm(&self) -> Product {
        Product {
            id: ID::from(self._id.to_string()),
            name: self.name.to_owned(),
            description: self.description.to_owned(),
            product_type: self.product_type.to_owned(),
            owner_id: self.owner_id.to_owned(),
            date_created: self.date_created.to_owned(),
            status: self.status.to_owned(),
        }
    }
}

#[async_graphql::Object]
impl Product {
    async fn id(&self) -> &str {
        &self.id
    }
    async fn name(&self) -> &str {
        &self.name
    }
    async fn description(&self) -> &str {
        &self.description
    }
    async fn product_type(&self) -> &str {
        &self.product_type
    }
    async fn owner_id(&self) -> &str {
        &self.owner_id
    }
    async fn date_created(&self) -> &str {
        &self.date_created
    }
    async fn status(&self) -> &str {
        &self.status
    }
    async fn owner_info(&self, ctx: &Context<'_>) -> FieldResult<User> {
        RootQuery.user_by_id(ctx, self.owner_id.to_string()).await
    }
}
