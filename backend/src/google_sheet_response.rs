use std::fs;
use serde_json::Value;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use reqwest::Client;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use serde::{Deserialize, Serialize};
use crate::song::Song;
use log::debug;


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
pub async fn get_access_token(key: &Value) -> Result<String, reqwest::Error> {
   // let key: serde_json::Value = serde_json::from_str(service_account_key).expect("google key not in right format");
    let private_key = key["private_key"].as_str().expect("private_key missing");
    let client_email = key["client_email"].as_str().expect("client_email missing");


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
pub async fn fetch_google_sheet(sheet_id: String) -> Result<GoogleSheetResponse, reqwest::Error> {

    //let service_account_key = std::fs::read_to_string(ACCOUNT_KEY_FILE).expect("Service account file missing");
    //let service_account_key = std::env::var("GOOGLE_API_KEY").expect("Secret was not found");
    let key_path = std::env::var("GOOGLE_API_KEY_PATH").expect("GOOGLE_API_KEY_PATH not set");
    let key_file = fs::read_to_string(key_path).expect("Failed to read Google API key file");
    let service_account_key: Value = serde_json::from_str(&key_file).expect("Invalid JSON in Google API key");

    
    let access_token = get_access_token(&service_account_key).await.expect("Failed to authenticate");
    
    //let sheet_id: &'static str = "1KWhp9nuuA4WrbEk2IssQUBVCPjVT6WX9gjuV9qFo7AI"; 
    
    let range = "A:D"; // TODO : put in a config file
   


    let url = format!(
       "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}?valueRenderOption=FORMATTED_VALUE",
        sheet_id, range
    );

    let client = Client::new();
    let response = client
        .get(&url)
        .bearer_auth(access_token)
        .send()
        .await?;
    debug!("{:?}", response);

    response.json::<GoogleSheetResponse>().await
}

impl GoogleSheetResponse {
    pub fn transform_google_format_to_song(&self) -> Vec<Song>{
        self
            .values.clone()
            .into_iter()
            .skip(1)//skipping the fist element (column names)
            .enumerate()
            .filter_map(|(i, row)| {
                // Attempt to map each row to a Song
                if let (Some(artist), Some(title), Some(lyrics)) = (row.get(1), row.get(0), row.get(2)) {
                    Some(Song {
                        id: (i + 1) as i32,
                        artist: artist.clone(),
                        title: title.clone(),
                        lyrics_url: lyrics.clone(),
                        singer:None
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
                vec!["header".to_string(), "header".to_string(), "header".to_string(),"header".to_string(),"header".to_string()],
                vec!["Chanson A".to_string(), "artiste 1".to_string(), "Artist A".to_string(),"test".to_string(),"test".to_string()],
                vec!["Chanson B".to_string(), "artiste 2".to_string(), "Artist A".to_string(),"test 2".to_string(),"test 2".to_string()],
                vec!["Chanson C".to_string(), "artiste 3".to_string(), "Artist A".to_string(),"test 3".to_string(),"test 3".to_string()],
            ],
        };
        let expected_result = vec![
            Song{id:0,artist:"artiste 1".to_string(),title:"Chanson A".to_string(),lyrics_url:"test".to_string(),singer:None},
            Song{id:0,artist:"artiste 2".to_string(),title:"Chanson B".to_string(),lyrics_url:"test 2".to_string(),singer:None},
            Song{id:0,artist:"artiste 3".to_string(),title:"Chanson C".to_string(),lyrics_url:"test 3".to_string(),singer:None},
        ];

        let songs = mock_sheet_data.transform_google_format_to_song();
        println!("{:?}", songs);
    
        assert!(songs == expected_result);
    }
}
