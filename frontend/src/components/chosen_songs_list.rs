use yew::prelude::*;
use web_sys::window;
use gloo_net::http::Request;
use log::error;
use crate::types::song::{Song};
use crate::config::{Config}; 


/// Refresh the chosen songs list by fetching from the server
pub fn refresh_chosen_songs(chosen_songs_list: UseStateHandle<Vec<Song>>) {
    wasm_bindgen_futures::spawn_local(async move {
        let config = Config::load();
        let url = format!("{}/song-playlist", config.backoffice_url);
        web_sys::console::log_1(&format!("refresh_chosen_songs").into());

        match Request::get(&url)
            .send()
            .await
        {
            Ok(response) => {
                if let Ok(fetched_songs) = response.json::<Vec<Song>>().await {
                    chosen_songs_list.set(fetched_songs);
                    web_sys::console::log_1(&format!("Fetch song ok").into());

                } else {
                    error!("Failed to parse JSON response");
                    web_sys::console::log_1(&format!("Failed to parse JSON response").into());

                }
            }
            Err(err) => {
                error!("Failed to fetch: {:?}", err);
                web_sys::console::log_1(&format!("Failed to fetch").into());

            }
        }
    });
}

#[derive(Properties, PartialEq)]
pub struct ChosenSongsListProps {
    pub on_click: Callback<Song>,
    pub songs_list: Vec<Song>
}

#[function_component(ChosenSongsList)]
pub fn chosen_songs_list(ChosenSongsListProps { on_click, songs_list }: &ChosenSongsListProps) -> Html {
    let location = window()
    .and_then(|win| win.location().pathname().ok()) // Get the path portion of the URL
    .unwrap_or_else(|| "/".to_string()); // Default to "/" if retrieval fails

    // Check if the current URL contains "/admin"
    let is_admin_page = location.contains("/maestro");


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
                <td>{if song.singer.is_some() { song.singer.clone().unwrap()} else {"None".to_string()} }</td>
                if is_admin_page {
                    <td onclick={on_song_select}>{ "supprimer"}</td>
                } else {
                    <td ></td>
                }
                
            </tr>
        }
    }).collect()
}