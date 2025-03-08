use yew::prelude::*;
use crate::types::song::Song;
use gloo_net::http::Request;
use log::error;
use crate::config::Config; 


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
    let search_query = use_state(|| "".to_string());
    // State for sorting
    let sort_column = use_state(|| "artist".to_string()); // Sort by artist initially
    let sort_order = use_state(|| true); // true = ascending, false = descending




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
                                let on_click = on_click.clone();
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
        </div>
    }

    
}