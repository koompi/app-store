use crate::schemas::root::Context;
use bson;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct UserModel {
  #[serde(rename = "_id")]
  pub id: bson::oid::ObjectId,
  pub name: String,
}

#[derive(Default, Debug)]
pub struct User {
  pub id: String,
  pub name: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "User Input")]
pub struct UserInput {
  pub name: String,
}

#[juniper::object(Context = Context)]
impl User {
  fn id(&self) -> &str {
    &self.id
  }
  fn name(&self) -> &str {
    &self.name
  }
}
