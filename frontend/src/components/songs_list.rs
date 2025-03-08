use web_sys::window;
use yew::prelude::*;
use crate::types::song::Song;
use gloo_net::http::Request;
use log::error;
use crate::config::Config; 
use crate::components::popup_add_song::PopupAddSong;


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
pub struct SongListProps {
    pub on_add: Callback<()>, // Callback to notify parent
}


#[function_component(SongsList)]
pub fn songs_list(SongListProps { on_add }: &SongListProps) -> Html {
    let search_query: UseStateHandle<String> = use_state(|| "".to_string());
    // State for sorting
    let sort_column = use_state(|| "artist".to_string()); // Sort by artist initially
    let sort_order = use_state(|| true); // true = ascending, false = descending
    let selected_song_to_add = use_state(|| None);

    let location = window()
    .and_then(|win| win.location().pathname().ok()) // Get the path portion of the URL
    .unwrap_or_else(|| "/".to_string()); // Default to "/" if retrieval fails

    // Check if the current URL contains "/admin"
    let is_admin_page = location.contains("/maestro");

    let songs_list = use_state(|| vec![]);
    {
        let songs_list = songs_list.clone();  
        use_effect_with((), move |_| {
            refresh_songs(songs_list.clone());
        || ()
        });
    }
    let songs_list = songs_list.clone();

    let admin_refresh_song = {
        let songs_list = songs_list.clone();

        Callback::from(move |_event : MouseEvent| {
            web_sys::console::log_1(&format!("on refresh click").into());
            force_refresh_songs(songs_list.clone());
        })
    };

    let show_add_popup = {  
        let selected_song_to_add = selected_song_to_add.clone();
        Callback::from(move |song: Song| selected_song_to_add.set(Some(song)))
    };

    let hide_add_popup = {
        let selected_song_to_add = selected_song_to_add.clone();
        Callback::from(move |_| selected_song_to_add.set(None))
    };

    let on_add_validate = {
        let hide_add_popup = hide_add_popup.clone();
        let selected_song_to_add = selected_song_to_add.clone();
        let on_add = on_add.clone();

        Callback::from(move |input: String| {
            let on_add = on_add.clone();

            web_sys::console::log_1(&format!("Validated input: {}", input).into());
            if let Some(mut song) = (*selected_song_to_add).clone() {
                song.singer = Some(input);
                let json_song = serde_json::to_string(&song).expect("Failed to serialize song to JSON");
                web_sys::console::log_1(&format!("full song with singer : {}", json_song).into());
                selected_song_to_add.set(Some(song.clone()));

                wasm_bindgen_futures::spawn_local(async move {
                    let config = Config::load();
                    let url = format!("{}/add-song", config.backoffice_url);
                    let on_add = on_add.clone();

                    match Request::post(&url)
                        .header("Content-Type", "application/json")
                        .body(serde_json::to_string(&song).unwrap())
                    {
                        Ok(request) => match request.send().await {
                            Ok(resp) => {
                                if resp.ok() {
                                    web_sys::console::log_1(&"Song successfully sent!".into());
                                    on_add.emit(()); // Notify parent
                                } else {
                                    web_sys::console::error_1(&format!("Failed to send song: {:?}", resp).into());
                                }
                            }
                            Err(err) => {
                                web_sys::console::error_1(&format!("Network error: {}", err).into());
                            }
                        },
                        Err(err) => {
                            web_sys::console::error_1(&format!("Failed to create request: {}", err).into());
                        }
                    }
                });
            
            }
            hide_add_popup.emit(());
            
        })
    };



    // Compute filtered songs dynamically
    let filtered_songs = {
        let search_query = search_query.clone();
        let sorted_songs = songs_list.clone();
        let sort_column = sort_column.clone();
        let sort_order = sort_order.clone();

        let mut songs = (*sorted_songs)
            .iter()
            .filter(|song| {
                let query = search_query.to_lowercase();
                song.artist.to_lowercase().contains(&query) || song.title.to_lowercase().contains(&query)
            })
            .cloned()
            .collect::<Vec<_>>();

        // Sorting logic
        songs.sort_by(|a, b| {
            let cmp = match sort_column.as_str() {
                "artist" => a.artist.to_lowercase().cmp(&b.artist.to_lowercase()),
                "title" => a.title.to_lowercase().cmp(&b.title.to_lowercase()),
                _ => a.artist.to_lowercase().cmp(&b.artist.to_lowercase()), // default to artist
            };
            if *sort_order {
                cmp
            } else {
                cmp.reverse() // reverse the order if descending
            }
        });

        songs
    };
    
    html! {
        <div class="w3-container" id="songs-list">   
        // Search bar
            <div class="search-bar">
                <input
                    type="text"
                    placeholder="Rechercher une chanson..."
                    value={(*search_query).clone()}
                    oninput={Callback::from(move |e: InputEvent| {
                        let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                        search_query.set(input.value());
                    })}
                />
            </div>

            // Table
            <table class="w3-table w3-striped w3-white">
                <thead class="w3-blue">
                    <tr>
                    <th onclick={
                        let sort_order = sort_order.clone();
                        let sort_column = sort_column.clone();

                        Callback::from(move |_| {
                            // Toggle sort by artist
                            let new_order = if *sort_column == "artist" && *sort_order { false } else { true };
                            let sort_order = sort_order.clone();
                            sort_column.set("artist".to_string());
                            sort_order.set(new_order);
                    })}>
                        { "Artiste" }
                        { if *sort_column == "artist" { if *sort_order { "↑" } else { "↓" } } else { "" } }
                    </th>
                    <th onclick={            
                        let sort_order = sort_order.clone();
                        let sort_column = sort_column.clone();
                        Callback::from(move |_| {
                            // Toggle sort by title
                            let new_order = if *sort_column == "title" && *sort_order { false } else { true };
                            sort_column.set("title".to_string());
                            let sort_order = sort_order.clone();
                            sort_order.set(new_order);
                    })}>
                        { "Titre" }
                        { if *sort_column == "title" { if *sort_order { "↑" } else { "↓" } } else { "" } }
                    </th>
                        <th>{ "Paroles" }</th>
                        <th>{ "Actions" }</th>
                    </tr>
                </thead>
                <tbody>
                    {
                        for filtered_songs.iter().map(|song| {
                            let on_song_select = {
                                let on_click = show_add_popup.clone();
                                let song = song.clone();
                                Callback::from(move |_| {
                                    on_click.emit(song.clone());
                                })
                            };

                            html! {
                                <tr key={song.id.to_string()}>
                                    <td>{ &song.artist }</td>
                                    <td>{ &song.title }</td>
                                    <td>
                                        <a target="_blank" href={song.lyrics_url.clone()}>
                                            { "Paroles" }
                                        </a>
                                    </td>
                                    <td>
                                        <button class="btn" onclick={on_song_select}>
                                            { "Choisir" }
                                        </button>
                                    </td>
                                </tr>
                            }
                        })
                    }
                </tbody>
            </table>
            {
                if let Some(_) = &*selected_song_to_add {
                    html! {
                        <PopupAddSong
                            on_validate={on_add_validate}
                            on_cancel={hide_add_popup}
                        />
                    }
                } else {
                    html! {}
                }
            }
            
            if is_admin_page {
                <button onclick={admin_refresh_song} class="admin-button">
                    { "Actualiser la liste de chanson depuis le Google Drive" }
                </button>
            }
        </div>
    }

    
}