use async_graphql::ID;
use bson::{self, oid::ObjectId};
use serde::{Deserialize, Serialize};
// use syn::Fields;
// use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: ID,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleModel {
    pub _id: ObjectId,
    pub name: String,
    pub description: String,
}

impl Role {
    pub fn new() -> Self {
        Self {
            id: ID::from(""),
            name: String::from(""),
            description: String::from(""),
        }
    }
}

impl RoleModel {
    pub fn new() -> Self {
        let converted_id = bson::oid::ObjectId::new();
        Self {
            _id: converted_id,
            name: String::from(""),
            description: String::from(""),
        }
    }

    pub fn to_norm(&self) -> Role {
        Role {
            id: ID::from(self._id.to_string()),
            name: self.name.to_owned(),
            description: self.description.to_owned(),
        }
    }
}

#[async_graphql::Object]
impl Role {
    async fn id(&self) -> &str {
        &self.id
    }
    async fn name(&self) -> &str {
        &self.name
    }
    async fn description(&self) -> &str {
        &self.description
    }
}
