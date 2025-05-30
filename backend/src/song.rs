use serde::{Deserialize, Serialize};
use actix_web::{web, post, get, Responder, HttpResponse};
use sqlx::FromRow;
use serde_json::json;
use log::debug;

use crate::config::Config;
use crate::state::AppState; 
use crate::google_sheet_response; 



#[derive(Clone, PartialEq, Serialize, Deserialize, FromRow, Debug)]
pub struct Song {
    pub id: i32,
    pub artist: String,
    pub title: String,
    pub lyrics_url: String,
    pub singer: Option<String>,
}

impl Song {
    pub async fn insert_song_into_playlist(&self, state: web::Data<AppState>) -> Result<Song, sqlx::Error>{
        sqlx::query_as("INSERT INTO current_playlist(artist, title, lyrics_url, singer) VALUES ($1, $2, $3, $4) RETURNING id, artist, title, lyrics_url, singer")
            .bind(&self.artist)
            .bind(&self.title)
            .bind(&self.lyrics_url)
            .bind(&self.singer)
            .fetch_one(&state.pool).await
    }

    pub async fn delete_song_from_playlist(&self, state: web::Data<AppState>) -> Result<bool, sqlx::Error>
    {
        let result = sqlx::query("UPDATE current_playlist SET is_deleted = TRUE WHERE id = $1")
            .bind(&self.id)
            .execute(&state.pool)
            .await; 

        match result {
            Ok(query_result) => {
                // Check if any rows were affected
                let rows_affected = query_result.rows_affected();
                Ok(rows_affected > 0) // Returns true if at least one row was deleted
            }
            Err(e) => Err(e), // Propagate the error
        }        
    }

}


pub async fn fetch_song_playlist(state: web::Data<AppState>) -> Result<Vec<Song>, sqlx::Error> {
    sqlx::query_as("select * FROM current_playlist WHERE is_deleted = FALSE")
    .fetch_all(&state.pool)
    .await
}


#[get("/song-update")]
async fn song_update(data: web::Data<AppState>) -> impl Responder {

    let google_sheet_id= Config {
            id:0, 
            name: "google_sheet_id".to_string(), 
            value : "".to_string()
        };
    let google_sheet_id = google_sheet_id.get_config_from_name(data.clone()).await.unwrap();

    match google_sheet_response::fetch_google_sheet(google_sheet_id.value).await {
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
    let google_sheet_id= Config {
            id:0, 
            name: "google_sheet_id".to_string(), 
            value : "".to_string()
        };
    let google_sheet_id = google_sheet_id.get_config_from_name(data.clone()).await.unwrap();

    // if the song collection does'nt exists we download it ortherwise we use the cache one
    if data.is_playlist_cache_empty() {
        debug!("Song list not cache creating it.");
        let content = google_sheet_response::fetch_google_sheet(google_sheet_id.value).await.expect("Error fetching document");
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

    let songs = fetch_song_playlist(state).await;

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