use async_graphql::{Context, FieldError, FieldResult};
use bson;
use futures::{lock::Mutex, stream::StreamExt};
use mongodb::bson::{doc, Bson};
use slab::Slab;
use std::sync::Arc;
// MODELS
use super::root::DB;
use crate::models::{
    asset::{Asset, AssetModel},
    org::{Org, OrgModel},
    product::{Product, ProductModel},
    profile::{Profile, ProfileModel},
    role::{Role, RoleModel},
    user::{User, UserModel},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Token {
    user_id: String,
}

pub type Storage = Arc<Mutex<Slab<User>>>;

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    // ============================ USER ============================
    pub async fn users(&self, ctx: &Context<'_>) -> FieldResult<Vec<User>> {
        let db = ctx.data_unchecked::<DB>().pool.clone();
        let collection = db.database("actix-juniper").collection("users");
        let mut data: Vec<User> = Vec::new();
        let mut cursor = collection.find(None, None).await?;

        // Iterate over the results of the cursor.
        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    let u: UserModel = bson::from_bson(Bson::Document(document))?;

                    data.push(u.to_norm());
                }
                Err(e) => return Err(e.into()),
            }
        }

        match data.is_empty() {
            true => Ok(data),
            false => Ok(data),
        }
    }

    pub async fn user_by_id(
        &self,
        ctx: &Context<'_>,
        #[arg(desc = "Query by user ID")] id: String,
    ) -> FieldResult<User> {
        let db = ctx.data_unchecked::<DB>().pool.clone();
        let collection = db.database("actix-juniper").collection("users");

        let converted_id = match bson::oid::ObjectId::with_string(&id) {
            Ok(data) => data,
            Err(_) => return Err(FieldError::from("Not a valid id")),
        };

        // create query
        let cursor = collection
            .find_one(doc! { "_id": converted_id }, None)
            .await
            .unwrap_or(None);

        let mut user: UserModel = UserModel::new();
        // iterate over query to get result
        for doc in cursor {
            user = bson::from_bson(Bson::Document(doc))?;
        }

        // return data
        match user._id.to_string() == "".to_string() {
            true => Err(FieldError::from("User not found")),
            false => Ok(user.to_norm()),
        }
    }
    pub async fn users_by_ids(
        &self,
        ctx: &Context<'_>,
        #[arg(desc = "Query by user ID")] ids: Vec<String>,
    ) -> FieldResult<Vec<User>> {
        let db = ctx.data_unchecked::<DB>().pool.clone();
        let collection = db.database("actix-juniper").collection("users");
        let mut id_list: Vec<bson::oid::ObjectId> = Vec::new();
        let mut data: Vec<User> = Vec::new();
        for i in ids.iter() {
            let converted_id = match bson::oid::ObjectId::with_string(i) {
                Ok(data) => data,
                Err(_) => return Err(FieldError::from("Not a valid id")),
            };
            id_list.push(converted_id)
        }

        // create query
        let mut cursor = collection
            .find(doc! { "_id": { "$all": id_list } }, None)
            .await?;

        // let mut user: UserModel = UserModel::new();
        // iterate over query to get result
        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    let u: UserModel = bson::from_bson(Bson::Document(document))?;

                    data.push(u.to_norm());
                }
                Err(e) => return Err(e.into()),
            }
        }

        match data.is_empty() {
            true => Ok(data),
            false => Ok(data),
        }
    }
    pub async fn user_by_email(
        &self,
        ctx: &Context<'_>,
        #[arg(desc = "Query by user ID")] email: String,
    ) -> FieldResult<User> {
        let db = ctx.data_unchecked::<DB>().pool.clone();
        let collection = db.database("actix-juniper").collection("users");

        // create query
        let cursor = collection
            .find_one(doc! { "email": email }, None)
            .await
            .unwrap_or(None);

        let mut user: UserModel = UserModel::new();
        // iterate over query to get result
        for doc in cursor {
            user = bson::from_bson(Bson::Document(doc))?;
        }

        // return data
        match user._id.to_string() == "".to_string() {
            true => Err(FieldError::from("User not found")),
            false => Ok(user.to_norm()),
        }
    }
    // ============================ ASSET ============================
    pub async fn assets(&self, ctx: &Context<'_>) -> FieldResult<Vec<Asset>> {
        let db = ctx.data_unchecked::<DB>().pool.clone();
        let collection = db.database("actix-juniper").collection("assets");
        let mut data: Vec<Asset> = Vec::new();
        let mut cursor = collection.find(None, None).await?;

        // Iterate over the results of the cursor.
        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    let u: AssetModel = bson::from_bson(Bson::Document(document))?;

                    data.push(u.to_norm());
                }
                Err(e) => return Err(e.into()),
            }
        }

        match data.is_empty() {
            true => Ok(data),
            false => Ok(data),
        }
    }

    pub async fn asset_by_id(
        &self,
        ctx: &Context<'_>,
        #[arg(desc = "Query by asset ID")] id: String,
    ) -> FieldResult<Asset> {
        let db = ctx.data_unchecked::<DB>().pool.clone();
        let collection = db.database("actix-juniper").collection("assets");
        let converted_id = match bson::oid::ObjectId::with_string(&id) {
            Ok(data) => data,
            Err(_) => return Err(FieldError::from("Not a valid id")),
        };

        // create query
        let cursor = collection
            .find_one(doc! { "_id": converted_id }, None)
            .await
            .unwrap_or(None);

        let mut asset: AssetModel = AssetModel::new();
        // iterate over query to get result
        for doc in cursor {
            asset = bson::from_bson(Bson::Document(doc))?;
        }

        // return data
        match asset._id.to_string() == "".to_string() {
            true => Err(FieldError::from("asset not found")),
            false => Ok(asset.to_norm()),
        }
    }

    pub async fn assets_by_owner_id(
        &self,
        ctx: &Context<'_>,
        #[arg(desc = "Query by user ID")] owner_id: String,
    ) -> FieldResult<Vec<Asset>> {
        let db = ctx.data_unchecked::<DB>().pool.clone();
        let collection = db.database("actix-juniper").collection("assets");
        let mut data: Vec<Asset> = Vec::new();
        let mut cursor = collection.find(doc! { "owner_id": owner_id }, None).await?;

        // Iterate over the results of the cursor.
        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    let u: AssetModel = bson::from_bson(Bson::Document(document))?;

                    data.push(u.to_norm());
                }
                Err(e) => return Err(e.into()),
            }
        }

        match data.is_empty() {
            true => Ok(data),
            false => Ok(data),
        }
    }
    // ============================ ORG ============================
    pub async fn orgs(&self, ctx: &Context<'_>) -> FieldResult<Vec<Org>> {
        let db = ctx.data_unchecked::<DB>().pool.clone();
        let collection = db.database("actix-juniper").collection("orgs");
        let mut data: Vec<Org> = Vec::new();
        let mut cursor = collection.find(None, None).await?;

        // Iterate over the results of the cursor.
        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    let u: OrgModel = bson::from_bson(Bson::Document(document))?;

                    data.push(u.to_norm());
                }
                Err(e) => return Err(e.into()),
            }
        }

        match data.is_empty() {
            true => Ok(data),
            false => Ok(data),
        }
    }

    pub async fn org_by_id(
        &self,
        ctx: &Context<'_>,
        #[arg(desc = "Query by org ID")] id: String,
    ) -> FieldResult<Org> {
        let db = ctx.data_unchecked::<DB>().pool.clone();
        let collection = db.database("actix-juniper").collection("orgs");
        let converted_id = match bson::oid::ObjectId::with_string(&id) {
            Ok(data) => data,
            Err(_) => return Err(FieldError::from("Not a valid id")),
        };

        // create query
        let cursor = collection
            .find_one(doc! { "_id": converted_id }, None)
            .await
            .unwrap_or(None);

        let mut org: OrgModel = OrgModel::new();
        // iterate over query to get result
        for doc in cursor {
            org = bson::from_bson(Bson::Document(doc))?;
        }

        // return data
        match org._id.to_string() == "".to_string() {
            true => Err(FieldError::from("org not found")),
            false => Ok(org.to_norm()),
        }
    }

    pub async fn orgs_by_owner_id(
        &self,
        ctx: &Context<'_>,
        #[arg(desc = "Query by user ID")] owner_id: String,
    ) -> FieldResult<Vec<Org>> {
        let db = ctx.data_unchecked::<DB>().pool.clone();
        let collection = db.database("actix-juniper").collection("orgs");
        let mut data: Vec<Org> = Vec::new();
        let mut cursor = collection.find(doc! { "owner_id": owner_id }, None).await?;

        // Iterate over the results of the cursor.
        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    let u: OrgModel = bson::from_bson(Bson::Document(document))?;

                    data.push(u.to_norm());
                }
                Err(e) => return Err(e.into()),
            }
        }

        match data.is_empty() {
            true => Ok(data),
            false => Ok(data),
        }
    }
    // ============================ PRODUCT ============================
    pub async fn products(&self, ctx: &Context<'_>) -> FieldResult<Vec<Product>> {
        let db = ctx.data_unchecked::<DB>().pool.clone();
        let collection = db.database("actix-juniper").collection("products");
        let mut data: Vec<Product> = Vec::new();
        let mut cursor = collection.find(None, None).await?;

        // Iterate over the results of the cursor.
        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    let u: ProductModel = bson::from_bson(Bson::Document(document))?;

                    data.push(u.to_norm());
                }
                Err(e) => return Err(e.into()),
            }
        }

        match data.is_empty() {
            true => Ok(data),
            false => Ok(data),
        }
    }

    pub async fn product_by_id(
        &self,
        ctx: &Context<'_>,
        #[arg(desc = "Query by product ID")] id: String,
    ) -> FieldResult<Product> {
        let db = ctx.data_unchecked::<DB>().pool.clone();
        let collection = db.database("actix-juniper").collection("products");
        let converted_id = match bson::oid::ObjectId::with_string(&id) {
            Ok(data) => data,
            Err(_) => return Err(FieldError::from("Not a valid id")),
        };

        // create query
        let cursor = collection
            .find_one(doc! { "_id": converted_id }, None)
            .await
            .unwrap_or(None);

        let mut product: ProductModel = ProductModel::new();
        // iterate over query to get result
        for doc in cursor {
            product = bson::from_bson(Bson::Document(doc))?;
        }

        // return data
        match product._id.to_string() == "".to_string() {
            true => Err(FieldError::from("product not found")),
            false => Ok(product.to_norm()),
        }
    }

    pub async fn products_by_owner_id(
        &self,
        ctx: &Context<'_>,
        #[arg(desc = "Query by user ID")] owner_id: String,
    ) -> FieldResult<Vec<Product>> {
        let db = ctx.data_unchecked::<DB>().pool.clone();
        let collection = db.database("actix-juniper").collection("products");
        let mut data: Vec<Product> = Vec::new();
        let mut cursor = collection.find(doc! { "owner_id": owner_id }, None).await?;

        // Iterate over the results of the cursor.
        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    let u: ProductModel = bson::from_bson(Bson::Document(document))?;

                    data.push(u.to_norm());
                }
                Err(e) => return Err(e.into()),
            }
        }

        match data.is_empty() {
            true => Ok(data),
            false => Ok(data),
        }
    }
    // ============================ PROFILE ============================
    pub async fn profiles(&self, ctx: &Context<'_>) -> FieldResult<Vec<Profile>> {
        let db = ctx.data_unchecked::<DB>().pool.clone();
        let collection = db.database("actix-juniper").collection("profiles");
        let mut data: Vec<Profile> = Vec::new();
        let mut cursor = collection.find(None, None).await?;

        // Iterate over the results of the cursor.
        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    let u: ProfileModel = bson::from_bson(Bson::Document(document))?;

                    data.push(u.to_norm());
                }
                Err(e) => return Err(e.into()),
            }
        }

        match data.is_empty() {
            true => Ok(data),
            false => Ok(data),
        }
    }

    pub async fn profile_by_id(
        &self,
        ctx: &Context<'_>,
        #[arg(desc = "Query by profile ID")] id: String,
    ) -> FieldResult<Profile> {
        let db = ctx.data_unchecked::<DB>().pool.clone();
        let collection = db.database("actix-juniper").collection("profiles");
        let converted_id = match bson::oid::ObjectId::with_string(&id) {
            Ok(data) => data,
            Err(_) => return Err(FieldError::from("Not a valid id")),
        };

        // create query
        let cursor = collection
            .find_one(doc! { "_id": converted_id }, None)
            .await
            .unwrap_or(None);

        let mut profile: ProfileModel = ProfileModel::new();
        // iterate over query to get result
        for doc in cursor {
            profile = bson::from_bson(Bson::Document(doc))?;
        }

        // return data
        match profile._id.to_string() == "".to_string() {
            true => Err(FieldError::from("profile not found")),
            false => Ok(profile.to_norm()),
        }
    }

    pub async fn profiles_by_owner_id(
        &self,
        ctx: &Context<'_>,
        #[arg(desc = "Query by user ID")] owner_id: String,
    ) -> FieldResult<Vec<Profile>> {
        let db = ctx.data_unchecked::<DB>().pool.clone();
        let collection = db.database("actix-juniper").collection("profiles");
        let mut data: Vec<Profile> = Vec::new();
        let mut cursor = collection.find(doc! { "owner_id": owner_id }, None).await?;

        // Iterate over the results of the cursor.
        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    let u: ProfileModel = bson::from_bson(Bson::Document(document))?;

                    data.push(u.to_norm());
                }
                Err(e) => return Err(e.into()),
            }
        }

        match data.is_empty() {
            true => Ok(data),
            false => Ok(data),
        }
    }
    // ============================ ROLE ============================
    pub async fn roles(&self, ctx: &Context<'_>) -> FieldResult<Vec<Role>> {
        let db = ctx.data_unchecked::<DB>().pool.clone();
        let collection = db.database("actix-juniper").collection("roles");
        let mut data: Vec<Role> = Vec::new();
        let mut cursor = collection.find(None, None).await?;

        // Iterate over the results of the cursor.
        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    let u: RoleModel = bson::from_bson(Bson::Document(document))?;

                    data.push(u.to_norm());
                }
                Err(e) => return Err(e.into()),
            }
        }

        match data.is_empty() {
            true => Ok(data),
            false => Ok(data),
        }
    }

    pub async fn role_by_id(
        &self,
        ctx: &Context<'_>,
        #[arg(desc = "Query by role ID")] id: String,
    ) -> FieldResult<Role> {
        let db = ctx.data_unchecked::<DB>().pool.clone();
        let collection = db.database("actix-juniper").collection("roles");
        let converted_id = match bson::oid::ObjectId::with_string(&id) {
            Ok(data) => data,
            Err(_) => return Err(FieldError::from("Not a valid id")),
        };

        // create query
        let cursor = collection
            .find_one(doc! { "_id": converted_id }, None)
            .await
            .unwrap_or(None);

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
}
