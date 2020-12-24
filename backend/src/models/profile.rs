use super::user::User;
use crate::graphql::RootQuery;
use async_graphql::{Context, FieldResult, ID};
use bson::{self, doc, oid::ObjectId, Document};
use serde::{Deserialize, Serialize};
// use syn::Fields;
// use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub id: ID,
    pub name: String,
    pub bio_desc: String,
    pub address: String,
    pub avatar: String,
    pub owner_id: String,
    pub website: String,
    pub company: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileModel {
    pub _id: ObjectId,
    pub name: String,
    pub bio_desc: String,
    pub address: String,
    pub avatar: String,
    pub owner_id: String,
    pub website: String,
    pub company: String,
}

impl Profile {
    pub fn new() -> Self {
        Self {
            id: ID::from(""),
            name: String::from(""),
            bio_desc: String::from(""),
            address: String::from(""),
            avatar: String::from(""),
            owner_id: String::from(""),
            website: String::from(""),
            company: String::from(""),
        }
    }
    pub fn to_bson_doc(&self) -> Document {
        let converted_id = bson::oid::ObjectId::with_string(&self.id.to_string()).unwrap();
        doc! {
            // "_id": converted_id,
            // "email": self.email.to_owned(),
            // "password": self.password.to_owned(),
            // "status": self.status.to_owned(),
            "_id": converted_id,
            "name": self.name.to_owned(),
            "bio_desc": self.bio_desc.to_owned(),
            "address": self.address.to_owned(),
            "avatar": self.avatar.to_owned(),
            "owner_id": self.owner_id.to_owned(),
            "website": self.website.to_owned(),
            "company": self.company.to_owned(),
        }
    }
}

impl ProfileModel {
    pub fn new() -> Self {
        let converted_id = bson::oid::ObjectId::new();
        Self {
            _id: converted_id,
            name: String::from(""),
            bio_desc: String::from(""),
            address: String::from(""),
            avatar: String::from(""),
            owner_id: String::from(""),
            website: String::from(""),
            company: String::from(""),
        }
    }

    pub fn to_norm(&self) -> Profile {
        Profile {
            id: ID::from(self._id.to_string()),
            name: self.name.to_owned(),
            bio_desc: self.bio_desc.to_owned(),
            address: self.address.to_owned(),
            avatar: self.avatar.to_owned(),
            owner_id: self.owner_id.to_owned(),
            website: self.website.to_owned(),
            company: self.company.to_owned(),
        }
    }
}

#[async_graphql::Object]
impl Profile {
    async fn id(&self) -> &str {
        &self.id
    }
    async fn name(&self) -> &str {
        &self.name
    }

    async fn bio_desc(&self) -> &str {
        &self.bio_desc
    }
    async fn address(&self) -> &str {
        &self.address
    }
    async fn avatar(&self) -> &str {
        &self.avatar
    }
    async fn owner_id(&self) -> &str {
        &self.owner_id
    }
    async fn website(&self) -> &str {
        &self.website
    }
    async fn company(&self) -> &str {
        &self.company
    }
    async fn owner_info(&self, ctx: &Context<'_>) -> FieldResult<User> {
        RootQuery.user_by_id(ctx, self.owner_id.to_string()).await
    }
}
