use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};

pub type DatabasePool = Pool<ConnectionManager<PgConnection>>;

pub fn get_connection_pool() -> DatabasePool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    Pool::builder().test_on_check_out(true).build(manager).expect("Pool fail to initialized")
}