use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;

const SONG_COLLECTION_FILEPATH: &str = "assets/song_collection.json";

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Song {
    pub id: usize,
    pub artist: String,
    pub name: String,
    pub lyrics_url: String,
}

// check if the collection is cache 
pub fn songs_list_cache_exists() -> bool {
    match fs::metadata(SONG_COLLECTION_FILEPATH) {
        Ok(metadata) => {
            if metadata.is_file() {
                //println!("File exists: {}", SONG_COLLECTION_FILEPATH);
                true
            } else {
                //println!("Path exists but is not a file: {}", SONG_COLLECTION_FILEPATH);
                false
            }
        }
        Err(_) => {
            //println!("File does not exist: {}", SONG_COLLECTION_FILEPATH);
            false
        }
    }
}

// cache the list of song in a file
pub fn cache_songs(data: &Vec<Song>) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(data)?; // Serialize to JSON
    let mut file = File::create(SONG_COLLECTION_FILEPATH)?; // Create or overwrite file
    file.write_all(json.as_bytes())?; // Write JSON to the file
    Ok(())
}


pub fn read_from_cache() -> std::io::Result<Vec<Song>> {
    let file = File::open(SONG_COLLECTION_FILEPATH)?; // Open the file
    let reader = BufReader::new(file); // Create a buffered reader
    let songs = serde_json::from_reader(reader)?; // Deserialize JSON to struct
    Ok(songs)
}

