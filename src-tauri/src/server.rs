use actix_web::web::Bytes;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use log::info;
use serde_json::{Value, json};
use std::sync::Mutex;
use crate::db::Database;
use crate::game_data;


pub struct SharedState {
    pub game_state: Mutex<Option<Value>>,
    pub db: Database,
}

#[derive(serde::Deserialize)]
pub struct PlayerData {
    pub steamid: String,
    pub name: String,
    pub team: String,
    pub state: PlayerState,
    pub match_stats: MatchStats,
}

#[derive(serde::Deserialize)]
pub struct PlayerState {
    pub armor: i32,
    pub burning: i32,
    pub equip_value: i32,
    pub flashed: i32,
    pub health: i32,
    pub helmet: bool,
    pub money: i32,
    pub round_killhs: i32,
    pub round_kills: i32,
    pub smoked: i32,
}

#[derive(serde::Deserialize)]
pub struct MatchStats {
    pub assists: i32,
    pub deaths: i32,
    pub kills: i32,
    pub mvps: i32,
    pub score: i32,
}

#[derive(serde::Deserialize)]
pub struct GameData {
    pub mode: String,
    pub map: Option<String>,  // Changed from 'name' to 'map'
    pub round: i32,
    pub phase: String,
    pub timestamp: i64,
}

pub struct GamePlayerData {
    pub player_id: i32,
    pub game_id: i32,
    pub team: String,
}

pub async fn index() -> impl Responder {
    info!("Received a request at /");
    "Hello, world!"
}

async fn handle_update(
    state: web::Data<SharedState>,
    _req: HttpRequest,
    body: Bytes,
) -> impl Responder {
    let update: Value = serde_json::from_slice(&body).unwrap();

    info!("Received data: {}", update);

    if let Some(auth) = update.get("auth") {
        if auth.get("token") == Some(&Value::String("CCWJu64ZV3JHDT8hZc".to_string())) {
            let mut game_state = match state.game_state.lock() {
                Ok(game_state) => game_state,
                Err(poisoned) => {
                    eprintln!("Mutex was poisoned. Recovering...");
                    poisoned.into_inner()
                },
            };
            *game_state = Some(update.clone());
            
            if update["player"]["activity"] == "menu" || (update["map"]["phase"] != "live" && update["map"]["phase"] != "gameover") {
                // If the activity is "menu" or the phase is not "gameover", skip processing the rest of the JSON data
                return HttpResponse::Ok().finish();
            }
            

            // Extract the game data and player data from the update
            let game_data: GameData = serde_json::from_value(json!({
                "mode": update["map"]["mode"],
                "name": update["map"]["name"],
                "round": update["map"]["round"],
                "phase": update["map"]["phase"],
                "timestamp": update["provider"]["timestamp"],  // get timestamp from provider
            })).unwrap();
            let player_data: PlayerData = serde_json::from_value(update["player"].clone()).unwrap();

            // Handle the game data and player data
            let db = &state.db;
            let game_id = db.insert_game(&game_data).await.unwrap();
            let player_id = db.insert_player(&player_data).await.unwrap();

            // Create and insert the GamePlayerData
            let game_player_data = GamePlayerData {
                player_id: player_id,  
                game_id: game_id,  
                team: player_data.team.clone(),
            };
            db.insert_game_player(&game_player_data).await.unwrap();

            // Call the handle_game_data function
            match game_data::handle_game_data(&db, &game_data, &player_data).await {
                Ok(_) => (),
                Err(e) => eprintln!("Error handling game data: {}", e),
            }
        } else {
            return HttpResponse::Forbidden().finish();
        }
    } else {
        return HttpResponse::BadRequest().finish();
    }

    HttpResponse::Ok().finish()
}



pub async fn start_server() {
    let db = Database::new().unwrap();
    let state = web::Data::new(SharedState {
        game_state: Mutex::new(None),
        db,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/", web::get().to(index))
            .route("/update", web::post().to(handle_update))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .await
    .unwrap();
}
