use super::user::User;
use crate::graphql::RootQuery;
use async_graphql::{Context, FieldResult, ID};
use bson::{self, oid::ObjectId};
use serde::{Deserialize, Serialize};
// use syn::Fields;
// use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Org {
    pub id: ID,
    pub name: String,
    pub description: String,
    pub owner_id: String,
    pub members: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgModel {
    pub _id: ObjectId,
    pub name: String,
    pub description: String,
    pub owner_id: String,
    pub members: Vec<String>,
}

impl Org {
    pub fn new() -> Self {
        Self {
            id: ID::from(""),
            name: String::from(""),
            description: String::from(""),
            owner_id: String::from(""),
            members: Vec::new(),
        }
    }
}

impl OrgModel {
    pub fn new() -> Self {
        let converted_id = bson::oid::ObjectId::new();
        Self {
            _id: converted_id,
            name: String::from(""),
            description: String::from(""),
            owner_id: String::from(""),
            members: Vec::new(),
        }
    }

    pub fn to_norm(&self) -> Org {
        Org {
            id: ID::from(self._id.to_string()),
            name: self.name.to_owned(),
            description: self.description.to_owned(),
            owner_id: self.owner_id.to_owned(),
            members: self.members.to_owned(),
        }
    }
}

#[async_graphql::Object]
impl Org {
    async fn id(&self) -> &str {
        &self.id
    }
    async fn name(&self) -> &str {
        &self.name
    }
    async fn description(&self) -> &str {
        &self.description
    }
    async fn owner_id(&self) -> &str {
        &self.owner_id
    }
    async fn members(&self) -> &Vec<String> {
        &self.members
    }
    async fn owner_info(&self, ctx: &Context<'_>) -> FieldResult<User> {
        RootQuery.user_by_id(ctx, self.owner_id.to_string()).await
    }
    async fn members_info(&self, ctx: &Context<'_>) -> FieldResult<Vec<User>> {
        RootQuery.users_by_ids(ctx, self.members.to_owned()).await
    }
}
