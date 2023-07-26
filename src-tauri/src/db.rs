use deadpool_postgres::{Config, Pool};
use crate::server::{PlayerData, GameData, GamePlayerData};
use tokio_postgres::{NoTls, types::Timestamp};
use tokio_postgres::types::ToSql;

use dotenv::dotenv;
use std::env;
use chrono::{DateTime, Utc, NaiveDateTime};

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
            let steamid: String = row.get("steamid");
            println!("ID: {}, Name: {}, steamid: {}", id, name, steamid);
        }
        

        Ok(())
    }

    pub async fn insert_player(&self, player_data: &PlayerData) -> Result<i32, Box<dyn std::error::Error>> {
        // Get a connection from the pool
        let conn = self.pool.get().await?;
    
        // Execute the query
        let rows = conn.query("INSERT INTO players (name, steamid) VALUES ($1, $2) RETURNING id", &[&player_data.name, &player_data.steamid]).await?;
    
        // Get the generated ID
        let id: i32 = rows[0].get(0);
    
        Ok(id)
    }
    
    pub async fn insert_game(&self, game_data: &GameData) -> Result<i32, Box<dyn std::error::Error>> {
        // Get a connection from the pool
        let conn = self.pool.get().await?;
    
        // Convert the timestamp from i64 to NaiveDateTime
        let naive_timestamp = NaiveDateTime::from_timestamp(game_data.timestamp, 0);
    
        // Convert NaiveDateTime to a string
        let timestamp_str = naive_timestamp.to_string();
    
        // Check if name is null and handle accordingly
        let name = match &game_data.map {
            Some(name) => name,
            None => "default", // Replace with a suitable default value
        };
    
        // Execute the query
        let rows = conn.query("INSERT INTO games (mode, map, round, phase, timestamp) VALUES ($1, $2, $3, $4, TO_TIMESTAMP($5, 'YYYY-MM-DD HH24:MI:SS')) RETURNING game_id", &[&game_data.mode, &name, &game_data.round, &game_data.phase, &timestamp_str]).await?;
    
        // Get the generated ID
        let game_id: i32 = rows[0].get(0);

    
        Ok(game_id)
    }
    
    
    

    pub async fn insert_game_player(&self, game_player_data: &GamePlayerData) -> Result<(), Box<dyn std::error::Error>> {
        // Get a connection from the pool
        let conn = self.pool.get().await?;

        // Execute the query
        conn.execute("INSERT INTO game_players (player_id, game_id, team) VALUES ($1, $2, $3)", &[&game_player_data.player_id, &game_player_data.game_id, &game_player_data.team]).await?;

        Ok(())
    }
}
