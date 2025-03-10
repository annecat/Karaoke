use serde::{Deserialize, Serialize};
use actix_web::{web, post, Responder, HttpResponse};
use sqlx::FromRow;
use crate::state::AppState; 
use serde_json::json;



#[post("/change-config")]
async fn change_config(content: web::Json<Config>, state: web::Data<AppState>) -> impl Responder {

    let res_content = content.into_inner().change_config_in_db(state).await   ;
    
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

#[post("/get-config")]
async fn get_config(content: web::Json<Config>, state: web::Data<AppState>) -> impl Responder {

    let res_content = content.into_inner().get_config_from_name(state).await   ;
    
    match res_content {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(error) => HttpResponse::InternalServerError().json(json!({
            "status": "ko",
            "content": error.to_string(), 
        })),
    }

}


#[derive(Clone, PartialEq, Serialize, Deserialize, FromRow, Debug)]
pub struct Config {
    pub id: i32,
    pub name : String,
    pub value: String,

}

impl Config {
    pub async fn change_config_in_db(&self, state: web::Data<AppState>) -> Result<Config, sqlx::Error>{
        sqlx::query_as("update config set value = $1 where name = $2 RETURNING id, name, value")
            .bind(&self.value)
            .bind(&self.name)
            .fetch_one(&state.pool).await
    }

    pub async fn get_config_from_name(&self, state: web::Data<AppState>) -> Result<Config, sqlx::Error> {
        sqlx::query_as("select id, name, value FROM config WHERE name = $1")
        .bind(&self.name)
        .fetch_one(&state.pool)
        .await
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};
    use sqlx::PgPool;
    use crate::state::AppState;
    use std::sync::{Arc, RwLock};
    use crate::song::Song;
    use std::path::Path;
    use std::fs;
    use toml;
    use serde::Deserialize;


    #[derive(Debug, Deserialize)]
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    struct Secrets {
        pub GOOGLE_API_KEY: String, // Replace with your actual secrets structure
        pub DATABASE_URL: String,
    }
    #[derive(Clone, PartialEq, Serialize, Deserialize, FromRow, Debug)]
    struct ResponseWrapper {
        status: String,
        content: Config,
    }
    
    fn load_secrets() -> Secrets {
        let path = Path::new("Secrets.toml"); // Adjust the path if needed
        let content = fs::read_to_string(path)
            .expect("Failed to read Secrets.toml");
        toml::from_str(&content)
            .expect("Failed to parse Secrets.toml")
    }

    async fn setup_test_db() -> PgPool {
        let secrets = load_secrets();
        let pool = PgPool::connect(&secrets.DATABASE_URL).await.expect("Fail to connect to Database");
        sqlx::query("CREATE TABLE IF NOT EXISTS config (id SERIAL PRIMARY KEY, name TEXT UNIQUE, value TEXT)")
            .execute(&pool)
            .await
            .unwrap();
        pool
    }

    #[actix_web::test]
    async fn test_change_config() {
        let pool = setup_test_db().await;
        let playlist_cache = Arc::new(RwLock::new(Vec::<Song>::new()));
        let state = web::Data::new(AppState { pool, playlist_cache });
        let app = test::init_service(App::new().app_data(state.clone()).service(change_config)).await;

        sqlx::query("update config set value='yes' where name='open'")
            .execute(&state.pool)
            .await
            .unwrap();
        
        let req = test::TestRequest::post()
            .uri("/change-config")
            .set_json(&Config { id: 1, name: "open".to_string(), value: "no".to_string() })
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        let raw_body: web::Bytes = test::read_body(resp).await;
        println!("Raw response body: {:?}", raw_body);

         
        let expected_result = Config {
            id : 1,
            name : "open".to_string(),
            value : "no".to_string()
        };

        let body: ResponseWrapper = serde_json::from_slice(&raw_body).expect("Failed to deserialize response body");
        println!("Deserialized response body: {:?}", body);

        assert_eq!(body.content, expected_result);
    }

    #[actix_web::test]
    async fn test_get_config() {
        let pool = setup_test_db().await;
        let playlist_cache = Arc::new(RwLock::new(Vec::<Song>::new()));
        let state = web::Data::new(AppState { pool, playlist_cache });
        let app = test::init_service(App::new().app_data(state.clone()).service(get_config)).await;

        sqlx::query("update config set value='yes' where name='open'")
            .execute(&state.pool)
            .await
            .unwrap();
        
        let req = test::TestRequest::post()
            .uri("/get-config")
            .set_json(&Config { id: 1, name: "open".to_string(), value: "yes".to_string() })
            .to_request();
         
            let expected_result = Config {
                id : 1,
                name : "open".to_string(),
                value : "yes".to_string()
             };

        let resp = test::call_service(&app, req).await;
            
        let raw_body: web::Bytes = test::read_body(resp).await;
        println!("Raw response body: {:?}", raw_body);

        let body: Config = serde_json::from_slice(&raw_body).expect("Failed to deserialize response body");
        println!("Deserialized response body: {:?}", body);

        assert_eq!(body, expected_result);


        
    }
}
