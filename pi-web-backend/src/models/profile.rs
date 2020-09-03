use super::user::User;
use crate::graphql::query::QueryRoot;
use async_graphql::{Context, FieldResult, ID};
use bson::{self, oid::ObjectId};
use serde_derive::{Deserialize, Serialize};
// use syn::Fields;
// use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub id: ID,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: String,
    pub gender: String,
    pub address: String,
    pub avatar: String,
    pub cover: String,
    pub owner_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileModel {
    pub _id: ObjectId,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: String,
    pub gender: String,
    pub address: String,
    pub avatar: String,
    pub cover: String,
    pub owner_id: String,
}

impl Profile {
    pub fn new() -> Self {
        Self {
            id: ID::from(""),
            first_name: String::from(""),
            last_name: String::from(""),
            date_of_birth: String::from(""),
            gender: String::from(""),
            address: String::from(""),
            avatar: String::from(""),
            cover: String::from(""),
            owner_id: String::from(""),
        }
    }
}

impl ProfileModel {
    pub fn new() -> Self {
        let converted_id = bson::oid::ObjectId::new();
        Self {
            _id: converted_id,
            first_name: String::from(""),
            last_name: String::from(""),
            date_of_birth: String::from(""),
            gender: String::from(""),
            address: String::from(""),
            avatar: String::from(""),
            cover: String::from(""),
            owner_id: String::from(""),
        }
    }

    pub fn to_norm(&self) -> Profile {
        Profile {
            id: ID::from(self._id.to_string()),
            first_name: self.first_name.to_owned(),
            last_name: self.last_name.to_owned(),
            date_of_birth: self.date_of_birth.to_owned(),
            gender: self.gender.to_owned(),
            address: self.address.to_owned(),
            avatar: self.avatar.to_owned(),
            cover: self.cover.to_owned(),
            owner_id: self.owner_id.to_owned(),
        }
    }
}

#[async_graphql::Object]
impl Profile {
    async fn id(&self) -> &str {
        &self.id
    }
    async fn first_name(&self) -> &str {
        &self.first_name
    }
    async fn last_name(&self) -> &str {
        &self.last_name
    }
    async fn date_of_birth(&self) -> &str {
        &self.date_of_birth
    }
    async fn gender(&self) -> &str {
        &self.gender
    }
    async fn address(&self) -> &str {
        &self.address
    }
    async fn avatar(&self) -> &str {
        &self.avatar
    }
    async fn cover(&self) -> &str {
        &self.cover
    }
    async fn owner_id(&self) -> &str {
        &self.owner_id
    }
    async fn owner_info(&self, ctx: &Context<'_>) -> FieldResult<User> {
        QueryRoot.user_by_id(ctx, self.owner_id.to_string()).await
    }
}
