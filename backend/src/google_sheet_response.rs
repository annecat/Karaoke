use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use reqwest::Client;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use serde::{Deserialize, Serialize};
use crate::song::Song;


#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct GoogleSheetResponse {
    range: String,
    majorDimension: String,
    values: Vec<Vec<String>>, // Nested vectors for rows and columns
}

// JWT claims for Google OAuth2
#[derive(Serialize, Deserialize)]
struct Claims {
    iss: String,       // Issuer (Service Account email)
    scope: String,     // API scope
    aud: String,       // Audience
    exp: usize,        // Expiration time
    iat: usize,        // Issued at
}


// Function to generate an access token
pub async fn get_access_token(service_account_key: &str) -> Result<String, reqwest::Error> {
    let key: serde_json::Value = serde_json::from_str(service_account_key).expect("google key not in right format");
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
pub async fn fetch_google_sheet() -> Result<GoogleSheetResponse, reqwest::Error> {

    //let service_account_key = std::fs::read_to_string(ACCOUNT_KEY_FILE).expect("Service account file missing");
    let service_account_key = std::env::var("GOOGLE_API_KEY").expect("Secret was not found");

    
    let access_token = get_access_token(&service_account_key).await.expect("Failed to authenticate");
    
    let sheet_id = "1OReTpbzBUhBRmgryjINbRhbxbYKsnTxJVKvBUPL2Wm0"; // TODO : put in a config file
    let range = "Chanson!A1:C10"; // TODO : put in a config file
   


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

impl GoogleSheetResponse {
    pub fn transform_google_format_to_song(&self) -> Vec<Song>{
        self
            .values.clone()
            .into_iter()
            .filter_map(|row| {
                // Attempt to map each row to a Song
                if let (Some(artist), Some(name)) = (row.get(0), row.get(1)) {
                    Some(Song {
                        id: 0,
                        artist: artist.clone(),
                        name: name.clone(),
                        lyrics_url: "test".to_string(),
                    })
                } else {
                    None // Skip rows with invalid data
                }
            })
            .collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_google_format_to_song() {
        let mock_sheet_data = GoogleSheetResponse {
            range: "A1:C1".to_string(),
            majorDimension: "ROWS".to_string(),
            values: vec![
                vec!["artiste 1".to_string(), "Chanson A".to_string(), "Artist A".to_string()],
                vec!["artiste 2".to_string(), "Chanson B".to_string(), "Artist A".to_string()],
                vec!["artiste 3".to_string(), "Chanson C".to_string(), "Artist A".to_string()],
            ],
        };
        let expected_result = vec![
            Song{id:0,artist:"artiste 1".to_string(),name:"Chanson A".to_string(),lyrics_url:"test".to_string()},
            Song{id:0,artist:"artiste 2".to_string(),name:"Chanson B".to_string(),lyrics_url:"test".to_string()},
            Song{id:0,artist:"artiste 3".to_string(),name:"Chanson C".to_string(),lyrics_url:"test".to_string()},
        ];

        let songs = mock_sheet_data.transform_google_format_to_song();
    
        assert!(songs == expected_result);
    }
}
