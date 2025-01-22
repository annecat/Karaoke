use yew::prelude::*;
use crate::types::song::{Song};
use gloo_net::http::Request;
use log::{  error};
use crate::config::{Config}; 


/// Refresh the chosen songs list by fetching from the server
pub fn force_refresh_songs(chosen_songs_list: UseStateHandle<Vec<Song>>) {
    wasm_bindgen_futures::spawn_local(async move {
        let config = Config::load();
        let url = format!("{}/song-update", config.backoffice_url);
        web_sys::console::log_1(&format!("force_refresh_songs").into());


        match Request::get(&url)
            .send()
            .await
        {
            Ok(_response) => {
                refresh_songs(chosen_songs_list);
            }
            Err(err) => {
                error!("Failed to fetch: {:?}", err);
                web_sys::console::log_1(&format!("Failed to parse JSON response").into());

            }
        }
    });
}


/// Refresh the chosen songs list by fetching from the server
pub fn refresh_songs(chosen_songs_list: UseStateHandle<Vec<Song>>) {
    wasm_bindgen_futures::spawn_local(async move {
        let config = Config::load();
        let url = format!("{}/song-data", config.backoffice_url);
        match Request::get(&url)
            .send()
            .await
        {
            Ok(response) => {
                if let Ok(fetched_songs) = response.json::<Vec<Song>>().await {
                    chosen_songs_list.set(fetched_songs);
                } else {
                    error!("song-data : Failed to parse JSON response");
                }
            }
            Err(err) => {
                error!("Failed to fetch: {:?}", err);
            }
        }
    });
}

#[derive(Properties, PartialEq)]
pub struct SongsListProps {
    pub on_click: Callback<Song>,
    pub songs_list: Vec<Song>

}

#[function_component(SongsList)]
pub fn songs_list(SongsListProps { on_click, songs_list }: &SongsListProps) -> Html {

    songs_list.iter().map(|song| {
        let on_song_select = {
            let on_click = on_click.clone();
            let song = song.clone();
            Callback::from(move |_| {
                on_click.emit(song.clone())
            })
        };
            
        html! {
            <tr key={song.id}>
                <td>{song.artist.clone()}</td>
                <td>{song.title.clone()}</td>
                <td><a target="_blank" href={song.lyrics_url.clone()}>{ "Paroles"}</a></td>
                <td><a class="btn" onclick={on_song_select}>{ "choisir"}</a></td>
            </tr>
        }
    }).collect()
}