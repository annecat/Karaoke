use serde::{Deserialize, Serialize};
use actix_web::web;
use sqlx::FromRow;

use crate::state::AppState; 


#[derive(Clone, PartialEq, Serialize, Deserialize, FromRow)]
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
        let result = sqlx::query("DELETE FROM current_playlist WHERE id = $1")
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
    sqlx::query_as("select * FROM current_playlist")
    .fetch_all(&state.pool)
    .await
}
