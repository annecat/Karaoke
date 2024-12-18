use serde::Deserialize;
use serde::Serialize;
use yew::Properties;

#[derive(Clone, PartialEq, Serialize, Deserialize, Properties)]
pub struct Song {
    pub id: i32,
    pub artist: String,
    pub title: String,
    pub lyrics_url: String,
    pub singer: Option<String>,
}

