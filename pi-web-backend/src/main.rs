#![allow(non_camel_case_types)]
// #![allow(unused_imports)]
// #![allow(dead_code)]
// #![allow(unused_variables)]
pub mod database;
pub mod graphql;
pub mod models;

use actix_cors::Cors;
use actix_web::{guard, web, App, HttpResponse, HttpServer, Result};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_actix_web::{GQLRequest, GQLResponse};
use database::db_pool;
use graphql::root::DB;
use graphql::{
    mutation::MutationRoot,
    query::QueryRoot,
    root::{BooksSchema, Storage},
};
use listenfd::ListenFd;
use mongodb::Client;

async fn index(
    pool: web::Data<Client>,
    schema: web::Data<BooksSchema>,
    req: GQLRequest,
) -> GQLResponse {
    let ctx = DB {
        pool: pool.get_ref().to_owned(),
    };

    let mut query = req.into_inner();
    query = query.data(ctx);
    query.execute(&schema).await.into()
}

async fn index_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        )))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();
    let pool = db_pool().await.unwrap();

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(Storage::default())
        .finish();

    println!("Playground: http://localhost:3300");

    let mut server = HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .data(pool.clone())
            .wrap(Cors::new().supports_credentials().finish())
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server
            .bind("0.0.0.0:3300")
            .expect("Can not bind to port 3300")
    };
    server.run().await
}
