use gloo::timers::callback::Interval;
use yew::prelude::*;
use web_sys::window;
use gloo_net::http::Request;
use log::error;
use crate::types::song::Song;
use crate::config::Config; 
use crate::components::popup_delete_song::PopupDeleteSong;


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
                match response.json::<Vec<Song>>().await {
                    Ok(fetched_songs) => {
                        chosen_songs_list.set(fetched_songs);
                        web_sys::console::log_1(&format!("Fetch song ok").into());
                    } 
                    Err(err) => {
                        error!("Failed to parse JSON response");
                        web_sys::console::log_1(&format!("Failed to parse JSON response {}", err).into());
                    }
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
    pub refresh_trigger: UseStateHandle<bool>,
}


#[function_component(ChosenSongsList)]
pub fn chosen_songs_list(ChosenSongsListProps { refresh_trigger }: &ChosenSongsListProps) -> Html {
    let location = window()
    .and_then(|win: web_sys::Window| win.location().pathname().ok()) // Get the path portion of the URL
    .unwrap_or_else(|| "/".to_string()); // Default to "/" if retrieval fails
    
    let selected_song_to_delete = use_state(|| None);

    // Check if the current URL contains "/admin"
    let is_admin_page = location.contains("/maestro");


    let chosen_songs_list = use_state(|| vec![]);
    let chosen_songs_list_clone = chosen_songs_list.clone(); // Clone for first effect
    use_effect_with((), move |_| {
        refresh_chosen_songs(chosen_songs_list_clone.clone());
        || ()
    });

    let refresh_trigger = refresh_trigger.clone(); // Clone refresh trigger
    let chosen_songs_list_clone = chosen_songs_list.clone(); // Clone again for second effect
    use_effect_with(refresh_trigger, move |refresh_trigger| {
        if **refresh_trigger {
            refresh_chosen_songs(chosen_songs_list_clone.clone());
            refresh_trigger.set(false); // Reset the trigger
        }
        || ()
    });

    let chosen_songs_list_callback = chosen_songs_list.clone();
    
    
    let show_delete_popup = {  
        let selected_song_to_delete = selected_song_to_delete.clone();
        Callback::from(move |song: Song| selected_song_to_delete.set(Some(song)))
    };

    let hide_delete_popup = {
        let selected_song_to_delete = selected_song_to_delete.clone();
        Callback::from(move |_| selected_song_to_delete.set(None))
    };

    let chosen_songs_list: UseStateHandle<Vec<Song>> = chosen_songs_list.clone();  
    let on_refresh_click = {
        Callback::from(move |_event : MouseEvent| {
            web_sys::console::log_1(&format!("on refresh click").into());
            refresh_chosen_songs(chosen_songs_list.clone())
        })
       
    };
    {
        let on_refresh_click = on_refresh_click.clone();
        use_effect_with((),move |_| {
            let interval = Interval::new(60000,
                move || {
                    web_sys::console::log_1(&"Interval!".into());
                    //refresh_chosen_songs(chosen_songs_list);
                    let event = MouseEvent::new("click").unwrap(); // Create a default MouseEvent
                    on_refresh_click.emit(event); // Emit MouseEvent
                }
            );

            // Return a cleanup function to stop the interval when the component unmounts
            move || { drop(interval);}
        })
    }

    let on_delete_validate = {
        let hide_delete_popup = hide_delete_popup.clone();
        let selected_song_to_delete = selected_song_to_delete.clone();
        let on_refresh_click = on_refresh_click.clone();

        Callback::from(move |_| {
            let on_refresh_click = on_refresh_click.clone(); // Re-clone if used inside async

            web_sys::console::log_1(&"Delete a song".into());
            

            if let Some(song) = (*selected_song_to_delete).clone() {
                selected_song_to_delete.set(Some(song.clone()));

                wasm_bindgen_futures::spawn_local(async move {
                    let on_refresh_click = on_refresh_click.clone();

                    let config = Config::load();
                    let url = format!("{}/delete-song", config.backoffice_url);

                    match Request::post(&url)
                        .header("Content-Type", "application/json")
                        .body(serde_json::to_string(&song).unwrap())
                    {
                        Ok(request) => match request.send().await {
                            Ok(resp) => {
                                if resp.ok() {
                                    web_sys::console::log_1(&"Song successfully deleted!".into());
                                    //refresh_chosen_songs(chosen_songs_list);
                                    let event = MouseEvent::new("click").unwrap(); // Create a default MouseEvent
                                    on_refresh_click.emit(event); // Emit MouseEvent
                        
                                } else {
                                    web_sys::console::error_1(&format!("Failed to delete song: {:?}", resp).into());
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
            hide_delete_popup.emit(());

        })
    };


    let mut cpt = 0;

    let rows: Html = chosen_songs_list_callback.iter().map(|song| {
        let on_song_select = {
            let on_click = show_delete_popup.clone();
            let song = song.clone();
            Callback::from(move |_| {
                on_click.emit(song.clone())
            })
        };
        cpt += 1;
        html! {
            <tr key={song.id.to_string()}>
                <td>{cpt}</td>
                <td>{song.artist.clone()}</td>
                <td>{song.title.clone()}</td>
                <td>{song.singer.clone().unwrap_or_else(|| "None".to_string())}</td>
                if is_admin_page {
                    <td>
                        <button class="btn" onclick={on_song_select}>
                            { "Supprimer" }
                        </button>
                    </td>
                } 
            </tr>
        }
    }).collect();

    if cpt == 0 {
        html! {
            <p>{ "Aucune chanson sélectionnée" }</p>
        }
    } else {
        html! {
            <div class="w3-container">

                <table class="w3-table w3-striped w3-white" id="chosen-song">
                        <thead class="w3-red">
                        <tr>
                            <th>{"#"}</th>
                            <th>{"Artiste"}</th>
                            <th>{"Titre"}</th>
                            <th>{"Chanteur"}</th>
                            if is_admin_page {
                                <th>{"Action"}</th>
                            } 
                        </tr>
                    </thead>
                    <tbody>
                        { rows }
                    </tbody>
                </table>
                <p>
                <button onclick={on_refresh_click} class="w3-red">
                    { "Actualiser la liste de chansons ci-dessus." }
                </button>
                </p>
                {
                    if let Some(_) = &*selected_song_to_delete {
                        html! {
                            <PopupDeleteSong
                                on_validate={on_delete_validate}
                                on_cancel={hide_delete_popup}
                            />
                        }
                    } else {
                        html! {}
                    }
                }
        
            </div>

        }
    }

}