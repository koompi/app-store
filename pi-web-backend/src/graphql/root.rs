use async_graphql::{
    Context, EmptySubscription, FieldError, FieldResult, Schema, SimpleBroker, ID,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use bson;
use futures::{lock::Mutex, stream::StreamExt};
use mongodb::{
    bson::{doc, Bson},
    Client,
};
use slab::Slab;
use std::sync::Arc;
// MODELS
use crate::models::{
    asset::{Asset, AssetModel},
    org::{Org, OrgModel},
    product::{Product, ProductModel},
    profile::{Profile, ProfileModel},
    role::{Role, RoleModel},
    user::{User, UserModel},
};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};

pub type BooksSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[derive(Debug)]
pub struct DB {
    pub pool: Client,
}

#[derive(Debug, Serialize, Deserialize)]
struct Token {
    user_id: String,
}

pub type Storage = Arc<Mutex<Slab<User>>>;

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    // ============================ USER ============================
    async fn users(&self, ctx: &Context<'_>) -> FieldResult<Vec<User>> {
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
    async fn user_by_email(
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
    async fn assets(&self, ctx: &Context<'_>) -> FieldResult<Vec<Asset>> {
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

    async fn asset_by_id(
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

    async fn assets_by_owner_id(
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
    async fn orgs(&self, ctx: &Context<'_>) -> FieldResult<Vec<Org>> {
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

    async fn org_by_id(
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

    async fn orgs_by_owner_id(
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
    async fn products(&self, ctx: &Context<'_>) -> FieldResult<Vec<Product>> {
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

    async fn product_by_id(
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

    async fn products_by_owner_id(
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
    async fn profiles(&self, ctx: &Context<'_>) -> FieldResult<Vec<Profile>> {
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

    async fn profile_by_id(
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

    async fn profiles_by_owner_id(
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
    async fn roles(&self, ctx: &Context<'_>) -> FieldResult<Vec<Role>> {
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

    async fn role_by_id(
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
