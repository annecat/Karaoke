use serde::{Deserialize, Serialize};
use actix_web::web;
use sqlx::FromRow;

use crate::state::AppState; 


#[derive(Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct Suggestion {
    pub id: i32,
    pub content: String,

}

impl Suggestion {
    pub async fn insert_suggestion_into_db(&self, state: web::Data<AppState>) -> Result<Suggestion, sqlx::Error>{
        sqlx::query_as("INSERT INTO suggestions(content) VALUES ($1) RETURNING id, content")
            .bind(&self.content)
            .fetch_one(&state.pool).await
    }

    pub async fn _delete_song_from_playlist(&self, state: web::Data<AppState>) -> Result<bool, sqlx::Error>
    {
        let result = sqlx::query("DELETE FROM suggestions WHERE id = $1")
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


pub async fn _fetch_suggestions(state: web::Data<AppState>) -> Result<Vec<Suggestion>, sqlx::Error> {
    sqlx::query_as("select * FROM suggestions")
    .fetch_all(&state.pool)
    .await
}
