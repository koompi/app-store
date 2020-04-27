#[macro_use]
extern crate juniper;
extern crate r2d2;
extern crate r2d2_mongodb;
extern crate serde_json;

use actix_web::{web, App, HttpServer};
use listenfd::ListenFd;

use crate::db::get_db_pool;
use crate::handlers::register;

mod db;
mod handlers;
mod schemas;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info,info");
    env_logger::init();

    let pool = get_db_pool();
    let mut server = HttpServer::new(move || {
        App::new()
            // .wrap(middleware::Logger::default())
            .service(
                web::scope("/")
                    .data(pool.clone())
                    .configure(register)
                    .default_service(web::to(|| async { "404" })),
            )
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server
            .bind("127.0.0.1:3000")
            .expect("Can not bind to port 8080")
    };

    server.run().await
}
