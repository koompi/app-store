use super::user::User;
use crate::graphql::root::QueryRoot;
use async_graphql::{Context, FieldResult, ID};
use bson::{self, oid::ObjectId};
use serde_derive::{Deserialize, Serialize};
use syn::Fields;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub id: ID,
    pub name: String,
    pub asset_type: String,
    pub date_created: String,
    pub modified_date: String,
    pub owner_id: String,
    pub privacy: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetModel {
    pub _id: ObjectId,
    pub name: String,
    pub asset_type: String,
    pub date_created: String,
    pub modified_date: String,
    pub owner_id: String,
    pub privacy: String,
    pub description: String,
}

impl Asset {
    pub fn new() -> Self {
        Asset {
            id: ID::from(""),
            name: String::from(""),
            asset_type: String::from(""),
            date_created: String::from(""),
            modified_date: String::from(""),
            owner_id: String::from(""),
            privacy: String::from(""),
            description: String::from(""),
        }
    }
}

impl AssetModel {
    pub fn new() -> Self {
        let converted_id = bson::oid::ObjectId::new();
        AssetModel {
            _id: converted_id,
            name: String::from(""),
            asset_type: String::from(""),
            date_created: String::from(""),
            modified_date: String::from(""),
            owner_id: String::from(""),
            privacy: String::from(""),
            description: String::from(""),
        }
    }

    pub fn to_norm(&self) -> Asset {
        Asset {
            id: ID::from(self._id.to_string()),
            name: self.name.to_owned(),
            asset_type: self.asset_type.to_owned(),
            date_created: self.date_created.to_owned(),
            modified_date: self.modified_date.to_owned(),
            owner_id: self.owner_id.to_owned(),
            privacy: self.privacy.to_owned(),
            description: self.description.to_owned(),
        }
    }
}

#[async_graphql::Object]
impl Asset {
    async fn id(&self) -> &str {
        &self.id
    }
    async fn name(&self) -> &str {
        &self.name
    }
    async fn asset_type(&self) -> &str {
        &self.asset_type
    }
    async fn date_created(&self) -> &str {
        &self.date_created
    }
    async fn modified_date(&self) -> &str {
        &self.modified_date
    }
    async fn owner_id(&self) -> &str {
        &self.owner_id
    }
    async fn privacy(&self) -> &str {
        &self.privacy
    }
    async fn description(&self) -> &str {
        &self.description
    }
    async fn owner_info(&self, ctx: &Context<'_>) -> FieldResult<User> {
        QueryRoot.user_by_id(ctx, self.owner_id.to_string()).await
    }
}
