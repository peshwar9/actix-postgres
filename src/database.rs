use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, PoolError};

use crate::envdata::ENVDATA;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn initialize_pool(database_url: &str) -> Result<Pool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn establish_connection() -> Pool {
    //   dotenv().ok();
    //  let database_url = env::var("DATABASE_URL").expect("DATABASE URL must be set");
    let database_url = &ENVDATA.database_url.clone();
    initialize_pool(&database_url).expect("Unable to create connection pool")
}

/* Old code
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE URL must be set");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
*/
