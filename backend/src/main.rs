use actix_files::Files;
use actix_cors::Cors;
use shuttle_runtime::SecretStore;
use actix_web::web::ServiceConfig;
use actix_web::{web, get, post, Responder, HttpResponse};
use actix_web::http::header::HeaderValue;
use actix_web::dev::RequestHead;
use shuttle_actix_web::ShuttleActixWeb;
use std::sync::{Arc, RwLock};
use crate::state::AppState;
use log::debug;
use sqlx::PgPool;
use crate::song::Song;
use crate::suggestion::Suggestion;
use serde_json::json;

mod google_sheet_response;
mod song;
mod state;
mod suggestion;

#[get("/song-update")]
async fn song_update(data: web::Data<AppState>) -> impl Responder {

    match google_sheet_response::fetch_google_sheet().await {
        Ok(content) => {
            let song_list = content.transform_google_format_to_song();
            data.update_playlist_cache(song_list);
            HttpResponse::Ok().body("Ok :p")
        },
        Err(err) => HttpResponse::InternalServerError().body(format!("Error fetching document: {}", err)),
    }
}

//get the songs from Google or cache is exists
#[get("/song-data")]
async fn song_data(data: web::Data<AppState>) -> impl Responder {

    let song_list;
    // if the song collection does'nt exists we download it ortherwise we use the cache one
    if data.is_playlist_cache_empty() {
        debug!("Song list not cache creating it.");
        let content = google_sheet_response::fetch_google_sheet().await.expect("Error fetching document");
            debug!("{:?}", content);
            //println!("{:?}", content);
            song_list = content.transform_google_format_to_song();
            data.update_playlist_cache(song_list.clone());
            HttpResponse::Ok().json(song_list)
    } else {
        debug!("Song list existing. Loading it");
        match data.read_from_cache() {
            Some(songs) => HttpResponse::Ok().json(songs),
            None => HttpResponse::InternalServerError().body("Error reading from cache to load the songs list"),
        }
    }
}

#[get("/song-playlist")]
async fn song_playlist(state: web::Data<AppState>) -> impl Responder {

    let songs = song::fetch_song_playlist(state).await;

    match songs {
        Ok(content) => HttpResponse::Ok().json(content),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error fetching document: {}", err)),
    }
}

#[post("/add-song")]
async fn add_song(song: web::Json<Song>, state: web::Data<AppState>) -> impl Responder {

    let song = song.into_inner().insert_song_into_playlist(state).await   ;
    
    match song {
        Ok(content) => HttpResponse::Ok().json(json!({
            "status": "ok",
            "content": content,
        })),
        Err(error) => HttpResponse::InternalServerError().json(json!({
            "status": "ko",
            "content": error.to_string(),
        })),
    }

}
 

#[post("/add-suggestion")]
async fn add_suggestion(suggestion: web::Json<Suggestion>, state: web::Data<AppState>) -> impl Responder {

    let suggestion = suggestion.into_inner().insert_suggestion_into_db(state).await   ;
    
    match suggestion {
        Ok(content) => HttpResponse::Ok().json(json!({
            "status": "ok",
            "content": content,
        })),
        Err(error) => HttpResponse::InternalServerError().json(json!({
            "status": "ko",
            "content": error.to_string(),
        })),
    }

}

#[post("/delete-song")]
async fn delete_song(song: web::Json<Song>, state: web::Data<AppState>) -> impl Responder {
    let deleted = song.into_inner().delete_song_from_playlist(state).await;

    match deleted {
        Ok(is_deleted_row) => match is_deleted_row {
            true => HttpResponse::Ok().json(json!({
                "status": "ok",
                "content": "one song deleted",
            })),
            false => HttpResponse::Ok().json(json!({
                "status": "ok",
                "content": "no song deleted",
            })),
        }
        Err(error) => HttpResponse::InternalServerError().json(json!({
            "status": "ko",
            "content": error.to_string(),
        })),
    }
}




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


    let state = web::Data::new(AppState { 
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
                .app_data(state)
        );
        // Add a static file service for the `public` folder
        cfg.service(Files::new("/maestro", "public").index_file("index.html"));
        cfg.service(Files::new("/", "public").index_file("index.html"));
    };


    Ok(config.into())
}

