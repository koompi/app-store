pub mod database;
pub mod graphql;
pub mod handler;
pub mod models;

// Library imports
use actix_cors::Cors;
use actix_web::{
    guard,
    // http::{header, StatusCode},
    web,
    App,
    HttpServer,
};
use async_graphql::{EmptySubscription, Schema};
use listenfd::ListenFd;

// Local imports
use database::db_pool;
use graphql::{RootMutation, RootQuery};
use handler::{gql_playgound, index};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename(".env").ok();
    let mut listenfd = ListenFd::from_env();

    let ip = dotenv::var("IP").unwrap();
    let port = dotenv::var("PORT").unwrap();
    let address = format!("{}:{}", ip, port);

    let pool = db_pool().await.unwrap();

    println!("AppStore is running at: http://{}", &address);
    println!("GraphQL is running at: http://{}/api", &address);

    let schema = Schema::build(RootQuery, RootMutation, EmptySubscription).finish();

    let mut server = HttpServer::new(move || {
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
                    .allow_any_origin()
                    // .allowed_origin("http://127.0.0.1:4002")
                    .allowed_methods(vec!["POST", "GET"])
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
    });
    server = if let Some(listener) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(listener)?
    } else {
        server.bind(&address)?
    };
    server.run().await

    // .bind(&address)?
    // .run()
    // .await
}
