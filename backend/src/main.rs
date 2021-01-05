pub mod database;
pub mod graphql;
pub mod handler;
pub mod models;

// Library imports
use actix_cors::Cors;
use actix_web::{
    guard,
    http::{header, StatusCode},
    web, App, HttpServer,
};
use async_graphql::{EmptySubscription, Schema};

// Local imports
use database::db_pool;
use graphql::{RootMutation, RootQuery};
use handler::{gql_playgound, index};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename(".env").ok();

    let ip = dotenv::var("IP").unwrap();
    let port = dotenv::var("PORT").unwrap();
    let address = format!("{}:{}", ip, port);

    let pool = db_pool().await.unwrap();

    println!("AppStore is running at: http://{}", &address);
    println!("GraphQL is running at: http://{}/api", &address);

    let schema = Schema::build(RootQuery, RootMutation, EmptySubscription).finish();

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .data(pool.clone())
            .wrap(
                // Cors::default()
                //     .allow_any_header()
                //     .allowed_methods(vec!["POST"])
                //     .allow_any_origin()
                //     .allowed_header(header::CONTENT_TYPE),
                Cors::default()
                    .allowed_origin("http://127.0.0.1:5500")
                    .allowed_methods(vec!["POST"])
                    // .allowed_headers(vec![
                    //     header::AUTHORIZATION,
                    //     header::ACCEPT,
                    //     header::ACCESS_CONTROL_ALLOW_ORIGIN,
                    //     header::CONTENT_TYPE,
                    //     header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
                    // ])
                    .allow_any_header()
                    .supports_credentials()
                    .max_age(3600),
            )
            .service(web::resource("/api").guard(guard::Post()).to(index))
            .service(web::resource("/api").guard(guard::Get()).to(gql_playgound))
    })
    .bind(&address)?
    .run()
    .await
}
