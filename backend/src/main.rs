use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use actix_cors::Cors;
use actix_web::{get, App, HttpServer, Responder, HttpResponse};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use std::time::UNIX_EPOCH;




// JWT claims for Google OAuth2
#[derive(Serialize, Deserialize)]
struct Claims {
    iss: String,       // Issuer (Service Account email)
    scope: String,     // API scope
    aud: String,       // Audience
    exp: usize,        // Expiration time
    iat: usize,        // Issued at
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GoogleSheetResponse {
    range: String,
    majorDimension: String,
    values: Vec<Vec<String>>, // Nested vectors for rows and columns
}

// Function to generate an access token
async fn get_access_token(service_account_key: &str) -> Result<String, reqwest::Error> {
    let key: serde_json::Value = serde_json::from_str(service_account_key).unwrap();
    let private_key = key["private_key"].as_str().unwrap();
    let client_email = key["client_email"].as_str().unwrap();

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize;
    let claims = Claims {
        iss: client_email.to_string(),
        scope: "https://www.googleapis.com/auth/spreadsheets.readonly".to_string(),
        aud: "https://oauth2.googleapis.com/token".to_string(),
        exp: now + 3600,
        iat: now,
    };

    let jwt = encode(
        &Header::new(Algorithm::RS256),
        &claims,
        &EncodingKey::from_rsa_pem(private_key.as_bytes()).unwrap(),
    )
    .unwrap();

    let client = Client::new();
    let response = client
        .post("https://oauth2.googleapis.com/token")
        .form(&[
            ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
            ("assertion", &jwt),
        ])
        .send()
        .await?;

    let response_json: serde_json::Value = response.json().await?;
    Ok(response_json["access_token"].as_str().unwrap().to_string())
}


// Fetch data from Google Sheets
async fn fetch_google_sheet(sheet_id: &str, range: &str, access_token: &str) -> Result<GoogleSheetResponse, reqwest::Error> {
    let url = format!(
        "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}",
        sheet_id, range
    );

    let client = Client::new();
    let response = client
        .get(&url)
        .bearer_auth(access_token)
        .send()
        .await?;

    response.json::<GoogleSheetResponse>().await
}


#[get("/")]
async fn hello() -> impl Responder {
    /*let service_account_key = std::fs::read_to_string("/mnt/c/perso/geek/karaoke/backend/assets/wise-scene-402116-cb412ba46835.json").unwrap();
    let access_token = get_access_token(&service_account_key).await.unwrap();

    let sheet_id = "1OReTpbzBUhBRmgryjINbRhbxbYKsnTxJVKvBUPL2Wm0";
    let range = "Chanson!A1:C10"; // Adjust the range as needed


    match fetch_google_sheet(sheet_id, range, &access_token).await {
        Ok(content) => HttpResponse::Ok().body(format!("<pre>{}</pre>", content)),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error fetching document: {}", err)),
    }*/
    HttpResponse::Ok().body("Hello world!")
}

#[get("/sheet-data")]
async fn sheet_data() -> impl Responder {
    let service_account_key = std::fs::read_to_string("/mnt/c/perso/geek/karaoke/backend/assets/wise-scene-402116-cb412ba46835.json").expect("Service account file missing");
    let access_token = get_access_token(&service_account_key).await.expect("Failed to authenticate");
    
    let sheet_id = "1OReTpbzBUhBRmgryjINbRhbxbYKsnTxJVKvBUPL2Wm0";
    let range = "Chanson!A1:C10"; // Adjust the range as needed
   
    match fetch_google_sheet(sheet_id, range, &access_token).await {
        Ok(content) => HttpResponse::Ok().json(content),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error fetching document: {}", err)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        /*let cors = Cors::default()
            .allowed_origin("http://172.31.2.41") // Replace with your frontend URL
            .allowed_methods(vec!["GET", "POST"]) // Restrict to needed methods
            .allowed_headers(vec!["Content-Type", "Authorization"]) // Only necessary headers
            .max_age(3600);*/
        let cors = Cors::default()
            .allow_any_origin() // Allows requests from any origin (for dev only)
            .allow_any_method() // Allows any HTTP method
            .allow_any_header(); // Allows any header
        App::new()
            .wrap(cors)
            .service(hello)
            .service(sheet_data)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
