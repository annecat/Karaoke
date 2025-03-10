use actix_files::Files;
use actix_cors::Cors;
use shuttle_runtime::SecretStore;
use actix_web::web::ServiceConfig;
use actix_web::web;
use actix_web::http::header::HeaderValue;
use actix_web::dev::RequestHead;
use shuttle_actix_web::ShuttleActixWeb;
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

// The entry point for Shuttle deployment
#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres] pgpool: PgPool,
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {

    secrets.into_iter().for_each(|(key, val)| {
        std::env::set_var(key, val);
    });     

    sqlx::migrate!()
        .run(&pgpool)
        .await
        .expect("Failed to run migrations");


    let state: web::Data<AppState> = web::Data::new(AppState { 
        playlist_cache: Arc::new(RwLock::new(vec![])),
        pool: pgpool, 
    });


    let config = move |cfg: &mut ServiceConfig| {

        let cors = Cors::default()
        .allowed_origin_fn(move |origin: &HeaderValue, _req_head: &RequestHead| {
            if let Ok(origin_str) = origin.to_str() {
                //server_config.allowed_origins.contains(&origin_str.to_string())
                let auth_url = std::env::var("ALLOWED_ORIGINS").expect("Secret was not found");
                auth_url.contains(&origin_str.to_string())
            } else {
                false
            }
        })
        .allowed_methods(vec!["GET", "POST"]) // Restrict to needed methods
        .allowed_headers(vec!["Content-Type", "Authorization"]) // Only necessary headers
        .max_age(3600);

        //cfg.service(song_data);
        //cfg.service(song_update);
        cfg.service(
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
                .app_data(state)
        );
        // Add a static file service for the `public` folder
        cfg.service(Files::new("/maestro", "public").index_file("index.html"));
        cfg.service(Files::new("/", "public").index_file("index.html"));
    };


    Ok(config.into())
}

