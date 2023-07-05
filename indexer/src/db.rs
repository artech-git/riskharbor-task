
use sqlx::Pool;
use sqlx::Postgres;

#[derive(Debug, Clone)]
pub struct Database(Pool<Postgres>); 

impl Database {
    fn init(url: &str) -> Result<Self, BackendError> {
        
        let pool = Pool::connect_lazy(url).unwrap();

        Database(pool)
    }

    fn insert(query: &str) -> Result<(), BackendError> {

        sqlx::query!(query).execute(self.0).

    }


}