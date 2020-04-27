// use r2d2_mysql::mysql::{Opts, OptsBuilder};
// use r2d2_mysql::MysqlConnectionManager;

// pub type Pool = r2d2::Pool<MysqlConnectionManager>;

// pub fn get_db_pool() -> Pool {
//   let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//   let opts = Opts::from_url(&db_url).unwrap();
//   let builder = OptsBuilder::from_opts(opts);
//   let manager = MysqlConnectionManager::new(builder);
//   r2d2::Pool::new(manager).expect("Failed to create DB Pool")
// }
// use mongodb::db::ThreadedDatabase;
// use r2d2::Pool;
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
