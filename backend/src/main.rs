use dotenvy::dotenv;
use std::env;
use actix_files::Files;
use actix_cors::Cors;
use actix_web::HttpServer;
use actix_web::web;
use actix_web::http::header::HeaderValue;
use actix_web::dev::RequestHead;
use actix_web::App;
use std::sync::{Arc, RwLock};
use crate::state::AppState;
use sqlx::PgPool;
use crate::song::{add_song, song_update, song_playlist, delete_song, song_data};
use crate::suggestion::add_suggestion;
use crate::content::{add_content, get_content};
use crate::config::{change_config, get_config};



mod google_sheet_response;
mod song;
mod content;
mod state;
mod suggestion;
mod config;


fn get_database_url() -> String {
    let host = env::var("POSTGRES_HOST").expect("POSTGRES_HOST must be set");
    let user = env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
    let password = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
    let port = env::var("POSTGRES_PORT").expect("POSTGRES_PORT must be set");
    let db = env::var("POSTGRES_DB").expect("POSTGRES_DB must be set");
    format!("postgres://{}:{}@{}:{}/{}", user, password, host, port, db)
}

// The entry point for Shuttle deployment
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Backend starting...");

    dotenv().ok();

    println!("PORT = {:?}", env::var("PORT"));
    println!("POSTGRES_HOST = {:?}", env::var("POSTGRES_HOST"));


    // Cloud Run fournit le port via $PORT
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");


    let pgpool = PgPool::connect(&get_database_url())
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!()
        .run(&pgpool)
        .await
        .expect("Failed to run migrations");


    let state: web::Data<AppState> = web::Data::new(AppState { 
        playlist_cache: Arc::new(RwLock::new(vec![])),
        pool: pgpool, 
    });



    HttpServer::new(move || {

        let cors = Cors::default()
        .allowed_origin_fn(move |origin: &HeaderValue, _req_head: &RequestHead| {
            if let Ok(origin_str) = origin.to_str() {
                let auth_url = std::env::var("ALLOWED_ORIGINS").expect("Env is not found");
                auth_url.contains(&origin_str.to_string())
            } else {
                false
            }
        })
        .allowed_methods(vec!["GET", "POST"]) // Restrict to needed methods
        .allowed_headers(vec!["Content-Type", "Authorization"]) // Only necessary headers
        .max_age(3600);

        App::new()
            .app_data(state.clone())
            .service(
                web::scope("/api")
                    .wrap(Arc::new(cors))
                    .service(song_data)
                    .service(song_update)
                    .service(add_song)
                    .service(song_playlist)
                    .service(delete_song)
                    .service(add_suggestion)
                    .service(add_content)
                    .service(get_content)
                    .service(change_config)
                    .service(get_config)
            )
        .service(Files::new("/maestro", "public").index_file("index.html"))
        .service(Files::new("/", "public").index_file("index.html"))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

