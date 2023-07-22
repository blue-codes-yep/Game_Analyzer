use deadpool_postgres::{Config, Pool};
use tokio_postgres::NoTls;
use dotenv::dotenv;
use std::env;

pub struct Database {
    pool: Pool,
}

impl Database {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok(); // load environment variables

        let mut cfg = Config::new();
        cfg.host = env::var("DATABASE_HOST").ok();
        cfg.user = env::var("DATABASE_USER").ok();
        cfg.password = env::var("DATABASE_PASSWORD").ok();
        cfg.dbname = env::var("DATABASE_NAME").ok();

        // Create a connection pool
        let pool: Pool = cfg.create_pool(None, NoTls)?;

        Ok(Database { pool })
    }

    pub async fn execute_query(&self, query: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Get a connection from the pool
        let conn = self.pool.get().await?;

        // Execute the query
        let rows = conn.query(query, &[]).await?;

        for row in &rows {
            let id: i32 = row.get("id");
            let name: String = row.get("name");
            let score: i32 = row.get("score");
            println!("ID: {}, Name: {}, Score: {}", id, name, score);
        }
        

        Ok(())
    }
}
