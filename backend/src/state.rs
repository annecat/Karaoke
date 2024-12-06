use std::sync::{Arc, RwLock};
use crate::song::Song; 
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub playlist_cache: Arc<RwLock<Vec<Song>>>,
    pub pool: PgPool,
}

impl AppState {
    pub fn update_playlist_cache(&self, new_songs: Vec<Song>) {
        // Obtain a mutable lock on the playlist_cache
        let mut playlist = self.playlist_cache.write().unwrap();

        // Replace the contents of the playlist_cache with the new vector
        *playlist = new_songs;
    }


    pub fn read_from_cache(&self) -> Option<Vec<Song>> {
        let cache = self.playlist_cache.read().ok()?;
        Some(cache.clone())
    }


    pub fn is_playlist_cache_empty(&self) -> bool {
        // Obtain a read lock on the playlist_cache
        let playlist = self.playlist_cache.read().unwrap();

        // Check if the Vec<Song> is empty
        playlist.is_empty()
    }

}



#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{web};

    use std::fs;
    use std::path::Path;
    use serde::Deserialize;
    use toml;

    #[derive(Debug, Deserialize)]
    #[allow(dead_code)]
    #[allow(non_snake_case)]
    struct Secrets {
        pub GOOGLE_API_KEY: String, // Replace with your actual secrets structure
        pub DATABASE_URL: String,
    }
    
    fn load_secrets() -> Secrets {
        let path = Path::new("Secrets.toml"); // Adjust the path if needed
        let content = fs::read_to_string(path)
            .expect("Failed to read Secrets.toml");
        toml::from_str(&content)
            .expect("Failed to parse Secrets.toml")
    }
    
    #[tokio::test]
    async fn test_cache_not_existing() {
        let secrets = load_secrets();

        let state = web::Data::new(AppState { 
            playlist_cache: Arc::new(RwLock::new(vec![])),
            pool: PgPool::connect_lazy(&secrets.DATABASE_URL).unwrap(), // Lazy connection, 
        });
        state.update_playlist_cache(vec![]);
        assert!(state.is_playlist_cache_empty() == true);
    }

    #[tokio::test]
    async fn test_cache_storing() {
        let secrets = load_secrets();

        let state = web::Data::new(AppState { 
            playlist_cache: Arc::new(RwLock::new(vec![])),
            pool: PgPool::connect_lazy(&secrets.DATABASE_URL).unwrap(), // Lazy connection, 
        });


        let test_cache = vec![
            Song{id:0,artist:"artiste 1".to_string(),title:"Chanson A".to_string(),lyrics_url:"test".to_string(),singer:None},
            Song{id:0,artist:"artiste 2".to_string(),title:"Chanson B".to_string(),lyrics_url:"test".to_string(),singer:None},
            Song{id:0,artist:"artiste 3".to_string(),title:"Chanson C".to_string(),lyrics_url:"test".to_string(),singer:None},
        ];
        state.update_playlist_cache(test_cache.clone());
        assert!(state.read_from_cache().unwrap() == test_cache);
    }

    #[tokio::test]
    async fn test_cache_existing() {
        let secrets = load_secrets();

        let state = web::Data::new(AppState { 
            playlist_cache: Arc::new(RwLock::new(vec![])),
            pool: PgPool::connect_lazy(&secrets.DATABASE_URL).unwrap(), // Lazy connection, 
        });


        let test_cache = vec![
            Song{id:0,artist:"artiste 1".to_string(),title:"Chanson A".to_string(),lyrics_url:"test".to_string(),singer:None},
            Song{id:0,artist:"artiste 2".to_string(),title:"Chanson B".to_string(),lyrics_url:"test".to_string(),singer:None},
            Song{id:0,artist:"artiste 3".to_string(),title:"Chanson C".to_string(),lyrics_url:"test".to_string(),singer:None},
        ];
        state.update_playlist_cache(test_cache.clone());
        assert!(state.is_playlist_cache_empty() == false);
    }
}
