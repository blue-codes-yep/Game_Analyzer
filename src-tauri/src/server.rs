use actix_web::{web, App, HttpServer, Responder, HttpRequest, HttpResponse};
use actix_web::web::Bytes;
use std::sync::Mutex;
use serde_json::Value;
use log::info;
pub struct SharedState {
    pub game_state: Mutex<Option<Value>>,
}


pub async fn index() -> impl Responder {
    info!("Received a request at /");
    "Hello, world!"
}

async fn handle_update(state: web::Data<SharedState>, _req: HttpRequest, body: Bytes) -> impl Responder {
    let update: Value = serde_json::from_slice(&body).unwrap();


    info!("Received data: {}", update);
    
    if let Some(auth) = update.get("auth") {
        if auth.get("token") == Some(&Value::String("CCWJu64ZV3JHDT8hZc".to_string())) {
            let mut game_state = state.game_state.lock().unwrap();
            *game_state = Some(update);
        } else {
            return HttpResponse::Forbidden().finish();
        }
    } else {
        return HttpResponse::BadRequest().finish();
    }

    HttpResponse::Ok().finish()
}



pub async fn start_server() {
    let state = web::Data::new(SharedState {
        game_state: Mutex::new(None),
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
