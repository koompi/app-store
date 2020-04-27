use juniper::{FieldError, FieldResult, RootNode};
use mongodb::db::ThreadedDatabase;
use mongodb::{bson, doc};

use crate::db::Pool;

use super::product::{Product, ProductInput};
use super::user::{User, UserInput, UserModel};
use mongodb::ordered::OrderedDocument;
pub struct Context {
  pub dbpool: Pool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

fn print_type_of<T>(_: &T) {
  println!("{}", std::any::type_name::<T>())
}

#[juniper::object(Context = Context)]
impl QueryRoot {
  #[graphql(description = "Get Single user reference by user ID")]
  async fn user(context: &Context, id: String) -> FieldResult<User> {
    let mut conn = context.dbpool.get().unwrap();

    let cursor = conn
      .collection("users")
      .find_one(
        Some(doc! { "_id":  bson::oid::ObjectId::with_string(&id).expect("Id not valid") }),
        None,
      )?
      .expect("Document not found");

    let user: UserModel = bson::from_bson(bson::Bson::Document(cursor))?;

    let new_id = &user.id;
    let new_name = &user.name;
    Ok(User {
      id: new_id.to_string(),
      name: new_name.to_string(),
    })
  }
}

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
  Schema::new(QueryRoot, MutationRoot)
}
