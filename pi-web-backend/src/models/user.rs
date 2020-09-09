use async_graphql::ID;
use bson::{self, doc, oid::ObjectId, Document};
use serde_derive::{Deserialize, Serialize};
// use syn::Fields;
// use uuid::Uuid;

// GQL Struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: ID,
    pub email: String,
    pub password: String,
    pub status: bool,
}

// Mongo Model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserModel {
    pub _id: ObjectId,
    pub email: String,
    pub password: String,
    pub status: bool,
}

// Base implemenation
impl User {
    pub fn new() -> Self {
        User {
            id: ID::from(""),
            email: String::from(""),
            password: String::from(""),
            status: false,
        }
    }
    pub fn to_bson_doc(&self) -> Document {
        let converted_id = bson::oid::ObjectId::with_string(&self.id.to_string()).unwrap();
        doc! {
            "_id": converted_id,
            "email": self.email.to_owned(),
            "password": self.password.to_owned(),
            "status": self.status.to_owned(),
        }
    }
}

// Mongo implementation
impl UserModel {
    pub fn new() -> UserModel {
        let converted_id = bson::oid::ObjectId::new();
        UserModel {
            _id: converted_id,
            email: String::from(""),
            password: String::from(""),
            status: false,
        }
    }
    pub fn to_norm(&self) -> User {
        User {
            id: ID::from(self._id.to_string()),
            email: self.email.to_owned(),
            password: self.password.to_owned(),
            status: self.status.to_owned(),
        }
    }
}

// GraphQL
#[async_graphql::Object]
impl User {
    async fn id(&self) -> &str {
        &self.id
    }
    async fn email(&self) -> &str {
        &self.email
    }
    async fn password(&self) -> &str {
        &self.password
    }
    async fn status(&self) -> &bool {
        &self.status
    }
}
