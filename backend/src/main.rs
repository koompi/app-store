// pub mod graphql;
// pub mod handler;

use actix_web::{guard, web, App, HttpRequest, HttpServer, Responder};
// use async_graphql::{EmptySubscription, Schema};
// use graphql::{mutation::RootMutation, query::RootQuery};
// use handler::{gql_playgound, index};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename(".env").ok();

    let addr = dotenv::var("PROCESS_ADDR").unwrap();
    let port = dotenv::var("PROCESS_PORT").unwrap();
    let address = format!("{}:{}", addr, port);

    // let schema = Schema::new(RootQuery, RootMutation, EmptySubscription);

    println!("{}", &address);

    HttpServer::new(move || {
        // App::new()
        // .data(schema.clone())
        // .service(web::resource("/").guard(guard::Post()).to(index))
        // .service(web::resource("/").guard(guard::Get()).to(gql_playgound))
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind(&address)?
    .run()
    .await
}
