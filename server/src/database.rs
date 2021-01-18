use mongodb::{error::Error, options::ClientOptions, Client};

pub async fn db_pool() -> Result<Client, Error> {
    dotenv::from_filename(".env").ok();

    let host = dotenv::var("DB_ADDR").unwrap();
    let port = dotenv::var("DB_PORT").unwrap();
    let pass = dotenv::var("DB_PASSWD").unwrap();
    let user = dotenv::var("DB_USER").unwrap();

    let db_address = format!(
        "mongodb://{user}:{pass}@{host}:{port}",
        user = user,
        pass = pass,
        host = host,
        port = port
    );

    let mut client_options = ClientOptions::parse(&db_address).await?;
    client_options.retry_writes = Some(false);
    let client = Client::with_options(client_options);

    match client {
        Ok(c) => {
            println!("Connected to database.");
            Ok(c)
        }
        Err(e) => Err(e),
    }
}
