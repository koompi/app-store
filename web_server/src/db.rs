use r2d2_mongodb::{ConnectionOptions, MongodbConnectionManager};

pub type Pool = r2d2::Pool<MongodbConnectionManager>;

pub fn get_db_pool() -> Pool {
  let manager = MongodbConnectionManager::new(
    ConnectionOptions::builder()
      .with_host("ds219839.mlab.com", 19839)
      .with_db("actix-juniper")
      .with_auth("root", "root123")
      .build(),
  );
  let pool = Pool::builder().max_size(16).build(manager).unwrap();
  pool
}
