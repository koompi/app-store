use mongodb::{error::Error, options::ClientOptions, Client};

pub async fn db_pool() -> Result<Client, Error> {
    // let client =
    //     Client::with_uri_str("mongodb://admin:admin2020@ds131296.mlab.com:31296/app_store").await;
    // let client = Client::
    // let mut client_options =
    //     ClientOptions::parse("mongodb://admin:admin2020@ds131296.mlab.com:31296/app_store").await?;
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
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
