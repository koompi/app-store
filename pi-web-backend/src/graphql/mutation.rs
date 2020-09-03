use async_graphql::{Context, FieldError, FieldResult};
use bcrypt::{hash, verify, DEFAULT_COST};
use bson;
use futures::lock::Mutex;
use mongodb::bson::{doc, Bson};
use slab::Slab;
use std::sync::Arc;
// MODELS
use super::query::QueryRoot;
use crate::models::{
    asset::Asset,
    org::Org,
    product::Product,
    profile::Profile,
    role::{Role, RoleModel},
    user::User,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use super::root::DB;

#[derive(Debug, Serialize, Deserialize)]
struct Token {
    user_id: String,
}

pub type Storage = Arc<Mutex<Slab<User>>>;

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    async fn create_user(
        &self,
        ctx: &Context<'_>,
        name: String,
        email: String,
        password: String,
    ) -> FieldResult<User> {
        let db = ctx.data_unchecked::<DB>().pool.clone();
        let collection = db.database("actix-juniper").collection("users");
        // hash password
        let hashed = hash(password, DEFAULT_COST).unwrap();
        // create doc object
        let new_user = doc! {
            "name": name.to_string(),
            "email": email.to_string(),
            "password": hashed.to_string(),
        };

        let mut new_user_id: String = String::from("");

        match collection.insert_one(new_user, None).await {
            Ok(data) => {
                let result = data.inserted_id.as_object_id();
                new_user_id = result.unwrap().to_string();
            }
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
        QueryRoot.user_by_id(ctx, new_user_id).await
    }

    async fn delete_user(&self, ctx: &Context<'_>, id: String) -> FieldResult<String> {
        let db = ctx.data_unchecked::<DB>().pool.clone();
        let collection = db.database("actix-juniper").collection("users");
        let converted_id = match bson::oid::ObjectId::with_string(&id) {
            Ok(data) => data,
            Err(_) => return Err(FieldError::from("Not a valid id")),
        };

        // create query
        let cursor = collection
            .delete_one(doc! { "_id": converted_id }, None)
            .await
            .unwrap();
        println!("{:?}", cursor.deleted_count);
        match cursor.deleted_count {
            1 => Ok(String::from("User deleted")),
            _ => Err(FieldError::from("User not delete")),
        }
    }
    // ========================== ASSET =============================
    async fn create_asset(
        &self,
        ctx: &Context<'_>,
        name: String,
        asset_type: String,
        date_created: String,
        modified_date: String,
        owner_id: String,
        privacy: String,
        description: String,
    ) -> FieldResult<Asset> {
        let db = ctx.data_unchecked::<DB>().pool.clone();
        let collection = db.database("actix-juniper").collection("assets");

        // create doc object
        let new_asset = doc! {
            "name": name.to_string(),
            "asset_type": asset_type.to_string(),
            "date_created": date_created.to_string(),
            "modified_date": modified_date.to_string(),
            "owner_id": owner_id.to_string(),
            "privacy": privacy.to_string(),
            "description": description.to_string(),
        };

        let mut new_asset_id: String = String::from("");

        match collection.insert_one(new_asset, None).await {
            Ok(data) => {
                let result = data.inserted_id.as_object_id();
                new_asset_id = result.unwrap().to_string();
            }
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
        QueryRoot.asset_by_id(ctx, new_asset_id).await
    }

    async fn delete_asset(&self, ctx: &Context<'_>, id: String) -> FieldResult<String> {
        let db = ctx.data_unchecked::<DB>().pool.clone();
        let collection = db.database("actix-juniper").collection("assets");
        let converted_id = match bson::oid::ObjectId::with_string(&id) {
            Ok(data) => data,
            Err(_) => return Err(FieldError::from("Not a valid id")),
        };

        // create query
        let cursor = collection
            .delete_one(doc! { "_id": converted_id }, None)
            .await
            .unwrap();
        println!("{:?}", cursor.deleted_count);
        match cursor.deleted_count {
            1 => Ok(String::from("Asset deleted")),
            _ => Err(FieldError::from("Asset not delete")),
        }
    }
    // ========================== ORG =============================
    async fn create_org(
        &self,
        ctx: &Context<'_>,
        name: String,
        description: String,
        owner_id: String,
        members: Vec<String>,
    ) -> FieldResult<Org> {
        let db = ctx.data_unchecked::<DB>().pool.clone();
        let collection = db.database("actix-juniper").collection("orgs");

        // create doc object
        let new_org = doc! {
            "name" : name.to_owned(),
            "description" : description.to_owned(),
            "owner_id" : owner_id.to_owned(),
            "members" : members.to_owned(),
        };

        let mut new_org_id: String = String::from("");

        match collection.insert_one(new_org, None).await {
            Ok(data) => {
                let result = data.inserted_id.as_object_id();
                new_org_id = result.unwrap().to_string();
            }
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
        QueryRoot.org_by_id(ctx, new_org_id).await
    }
    async fn delete_org(&self, ctx: &Context<'_>, id: String) -> FieldResult<String> {
        let db = ctx.data_unchecked::<DB>().pool.clone();
        let collection = db.database("actix-juniper").collection("orgs");
        let converted_id = match bson::oid::ObjectId::with_string(&id) {
            Ok(data) => data,
            Err(_) => return Err(FieldError::from("Not a valid id")),
        };

        // create query
        let cursor = collection
            .delete_one(doc! { "_id": converted_id }, None)
            .await
            .unwrap();
        println!("{:?}", cursor.deleted_count);
        match cursor.deleted_count {
            1 => Ok(String::from("Org deleted")),
            _ => Err(FieldError::from("Org not delete")),
        }
    }
    // ========================== PRODUCT =============================
    async fn create_product(
        &self,
        ctx: &Context<'_>,
        name: String,
        description: String,
        product_type: String,
        owner_id: String,
        date_created: String,
        status: String,
    ) -> FieldResult<Product> {
        let db = ctx.data_unchecked::<DB>().pool.clone();
        let collection = db.database("actix-juniper").collection("products");

        // create doc object
        let new_product = doc! {
            "name" : name.to_owned(),
            "description" : description.to_owned(),
            "product_type" : product_type.to_owned(),
            "owner_id" : owner_id.to_owned(),
            "date_created" : date_created.to_owned(),
            "status" : status.to_owned(),
        };

        let mut new_product_id: String = String::from("");

        match collection.insert_one(new_product, None).await {
            Ok(data) => {
                let result = data.inserted_id.as_object_id();
                new_product_id = result.unwrap().to_string();
            }
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
        QueryRoot.product_by_id(ctx, new_product_id).await
    }
    async fn delete_product(&self, ctx: &Context<'_>, id: String) -> FieldResult<String> {
        let db = ctx.data_unchecked::<DB>().pool.clone();
        let collection = db.database("actix-juniper").collection("products");
        let converted_id = match bson::oid::ObjectId::with_string(&id) {
            Ok(data) => data,
            Err(_) => return Err(FieldError::from("Not a valid id")),
        };

        // create query
        let cursor = collection
            .delete_one(doc! { "_id": converted_id }, None)
            .await
            .unwrap();
        println!("{:?}", cursor.deleted_count);
        match cursor.deleted_count {
            1 => Ok(String::from("Product deleted")),
            _ => Err(FieldError::from("Product not delete")),
        }
    }
    // ========================== PROFILE =============================
    async fn create_profile(
        &self,
        ctx: &Context<'_>,
        first_name: String,
        last_name: String,
        date_of_birth: String,
        gender: String,
        address: String,
        avatar: String,
        cover: String,
        owner_id: String,
    ) -> FieldResult<Profile> {
        let db = ctx.data_unchecked::<DB>().pool.clone();
        let collection = db.database("actix-juniper").collection("profiles");

        // create doc object
        let new_profile = doc! {
            "first_name": first_name.to_owned(),
            "last_name": last_name.to_owned(),
            "date_of_birth": date_of_birth.to_owned(),
            "gender": gender.to_owned(),
            "address": address.to_owned(),
            "avatar": avatar.to_owned(),
            "cover": cover.to_owned(),
            "owner_id": owner_id.to_owned(),
        };

        let mut new_profile_id: String = String::from("");

        match collection.insert_one(new_profile, None).await {
            Ok(data) => {
                let result = data.inserted_id.as_object_id();
                new_profile_id = result.unwrap().to_string();
            }
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
        QueryRoot.profile_by_id(ctx, new_profile_id).await
    }
    async fn delete_profile(&self, ctx: &Context<'_>, id: String) -> FieldResult<String> {
        let db = ctx.data_unchecked::<DB>().pool.clone();
        let collection = db.database("actix-juniper").collection("profiles");
        let converted_id = match bson::oid::ObjectId::with_string(&id) {
            Ok(data) => data,
            Err(_) => return Err(FieldError::from("Not a valid id")),
        };

        // create query
        let cursor = collection
            .delete_one(doc! { "_id": converted_id }, None)
            .await
            .unwrap();
        println!("{:?}", cursor.deleted_count);
        match cursor.deleted_count {
            1 => Ok(String::from("Profile deleted")),
            _ => Err(FieldError::from("Profile not delete")),
        }
    }
    // ========================== ROLE =============================
    async fn create_role(
        &self,
        ctx: &Context<'_>,
        name: String,
        description: String,
    ) -> FieldResult<Role> {
        let db = ctx.data_unchecked::<DB>().pool.clone();
        let collection = db.database("actix-juniper").collection("roles");

        // create doc object
        let new_role = doc! {
            "name": name.to_owned(),
            "description": description.to_owned(),
        };

        let mut new_role_id: String = String::from("");

        match collection.insert_one(new_role, None).await {
            Ok(data) => {
                let result = data.inserted_id.as_object_id();
                new_role_id = result.unwrap().to_string();
            }
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
        QueryRoot.role_by_id(ctx, new_role_id).await
    }

    async fn delete_role(&self, ctx: &Context<'_>, id: String) -> FieldResult<String> {
        let db = ctx.data_unchecked::<DB>().pool.clone();
        let collection = db.database("actix-juniper").collection("roles");
        let converted_id = match bson::oid::ObjectId::with_string(&id) {
            Ok(data) => data,
            Err(_) => return Err(FieldError::from("Not a valid id")),
        };

        // create query
        let cursor = collection
            .delete_one(doc! { "_id": converted_id }, None)
            .await
            .unwrap();
        println!("{:?}", cursor.deleted_count);
        match cursor.deleted_count {
            1 => Ok(String::from("Role deleted")),
            _ => Err(FieldError::from("Role not delete")),
        }
    }

    async fn update_role(
        &self,
        ctx: &Context<'_>,
        id: String,
        name: Option<String>,
    ) -> FieldResult<Role> {
        let db = ctx.data_unchecked::<DB>().pool.clone();

        let collection = db.database("actix-juniper").collection("roles");
        let converted_id = match bson::oid::ObjectId::with_string(&id) {
            Ok(data) => data,
            Err(_) => return Err(FieldError::from("Not a valid id")),
        };

        // create query
        let cursor = collection
            .find_one_and_update(
                doc! { "_id": converted_id },
                doc! { "$set" : { "name" : name.unwrap_or(String::from("")) }},
                None,
            )
            .await?;
        let mut role: RoleModel = RoleModel::new();
        // iterate over query to get result
        for doc in cursor {
            role = bson::from_bson(Bson::Document(doc))?;
        }

        // return data
        match role._id.to_string() == "".to_string() {
            true => Err(FieldError::from("role not found")),
            false => Ok(role.to_norm()),
        }
    }

    async fn log_in(
        &self,
        ctx: &Context<'_>,
        email: String,
        password: String,
    ) -> FieldResult<String> {
        use dotenv::dotenv;
        use std::env;
        dotenv().ok();
        // for (key, value) in env::vars() {
        //     println!("{}: {}", key, value);
        // }
        match QueryRoot.user_by_email(ctx, email).await {
            Ok(data) => match verify(password, &data.password).unwrap() {
                true => {
                    #[allow(non_snake_case)]
                    let SECRET = env::var("SECRET").unwrap();
                    let option = Token {
                        user_id: data.email,
                    };
                    let token = encode(
                        &Header::default(),
                        &option,
                        &EncodingKey::from_secret(SECRET.as_ref()),
                    )
                    .unwrap();
                    Ok(token.to_string())
                }
                false => Err(FieldError::from("NO")),
            },
            Err(e) => Err(FieldError::from("NO")),
        }
    }
}
