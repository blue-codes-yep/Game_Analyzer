use crate::db::Database;
use crate::server::{PlayerData, GameData, GamePlayerData};
use std::error::Error;

pub async fn handle_game_data(db: &Database, game_data: &GameData, player_data: &PlayerData) -> Result<(), Box<dyn Error>> {
    // Insert the player data into the players table and get the generated ID
    let player_id = db.insert_player(&player_data).await?;

    // Insert the game data into the games table and get the generated ID
    let game_id = db.insert_game(&game_data).await?;

    // Create a GamePlayerData and insert it into the game_players table
    let game_player_data = GamePlayerData {
        player_id,
        game_id,
        team: player_data.team.clone(),
    };
    db.insert_game_player(&game_player_data).await?;

    Ok(())
}

