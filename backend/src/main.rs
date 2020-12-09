pub mod graphql;
pub mod handler;

use actix_web::{guard, web, App, HttpServer};
use async_graphql::{EmptySubscription, Schema};
use graphql::{mutation::RootMutation, query::RootQuery};
use handler::{gql_playgound, index};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename(".env").ok();

    let ip = dotenv::var("IP").unwrap();
    let port = dotenv::var("PORT").unwrap();
    let address = format!("{}:{}", ip, port);

    println!("AppStore is running at: http://{}", &address);

    let schema = Schema::build(RootQuery, RootMutation, EmptySubscription).finish();

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(web::resource("/api").guard(guard::Post()).to(index))
            .service(web::resource("/api").guard(guard::Get()).to(gql_playgound))
    })
    .bind(&address)?
    .run()
    .await
}
