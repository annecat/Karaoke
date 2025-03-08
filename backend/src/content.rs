use serde::{Deserialize, Serialize};
use actix_web::{web, post, Responder, HttpResponse};
use sqlx::FromRow;
use crate::state::AppState; 
use serde_json::json;



#[post("/add-content")]
async fn add_content(content: web::Json<Content>, state: web::Data<AppState>) -> impl Responder {

    let res_content = content.into_inner().insert_suggestion_into_db(state).await   ;
    
    match res_content {
        Ok(res) => HttpResponse::Ok().json(json!({
            "status": "ok",
            "content": res,
        })),
        Err(error) => HttpResponse::InternalServerError().json(json!({
            "status": "ko",
            "content": error.to_string(),
        })),
    }

}

#[post("/get-content")]
async fn get_content(content: web::Json<Content>, state: web::Data<AppState>) -> impl Responder {

    let res_content = content.into_inner().get_content_from_id(state).await   ;
    
    match res_content {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(error) => HttpResponse::InternalServerError().json(json!({
            "status": "ko",
            "content": error.to_string(), 
        })),
    }

}


#[derive(Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct Content {
    pub id: String,
    pub content_text: String,

}

impl Content {
    pub async fn insert_suggestion_into_db(&self, state: web::Data<AppState>) -> Result<Content, sqlx::Error>{
        sqlx::query_as("INSERT INTO content(id, content_text) VALUES ($1, $2) ON CONFLICT (id) DO UPDATE 
                            SET content_text = excluded.content_text RETURNING id, content_text")
            .bind(&self.id)
            .bind(&self.content_text)
            .fetch_one(&state.pool).await
    }

    pub async fn _delete_content_from_id(&self, state: web::Data<AppState>) -> Result<bool, sqlx::Error>
    {
        let result = sqlx::query("DELETE FROM content WHERE id = $1")
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

    pub async fn get_content_from_id(&self, state: web::Data<AppState>) -> Result<Content, sqlx::Error> {
        sqlx::query_as("select id, content_text FROM content WHERE id = $1")
        .bind(&self.id)
        .fetch_one(&state.pool)
        .await
    }

}


