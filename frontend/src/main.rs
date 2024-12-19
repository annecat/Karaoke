use gloo::timers::callback::Interval;
use web_sys::window;
use yew::prelude::*;
use gloo_net::http::Request;
    

mod components {
    pub mod songs_list;
    pub mod chosen_songs_list;
    pub mod popup_add_song;
    pub mod popup_delete_song;
}

mod types {
    pub mod song;
}

mod config;

use crate::config::{Config};
use crate::components::songs_list::{SongsList, refresh_songs, force_refresh_songs};
use crate::components::popup_add_song::{PopupAddSong};
use crate::components::popup_delete_song::{PopupDeleteSong};
use crate::components::chosen_songs_list::{ChosenSongsList, refresh_chosen_songs};
use crate::types::song::{Song};

#[function_component(App)]
fn app() -> Html {

    let selected_song_to_add = use_state(|| None);
    let selected_song_to_delete = use_state(|| None);

    let location = window()
    .and_then(|win| win.location().pathname().ok()) // Get the path portion of the URL
    .unwrap_or_else(|| "/".to_string()); // Default to "/" if retrieval fails

    // Check if the current URL contains "/admin"
    let is_admin_page = location.contains("/maestro");
    

    let chosen_songs_list = use_state(|| vec![]);
    {
        let chosen_songs_list = chosen_songs_list.clone();  
        use_effect_with((), move |_| {
            refresh_chosen_songs(chosen_songs_list.clone());
        || ()
        });
    }
    let chosen_songs_list_callback = chosen_songs_list.clone();
    

    let songs_list = use_state(|| vec![]);
    {
        let songs_list = songs_list.clone();  
        use_effect_with((), move |_| {
            refresh_songs(songs_list.clone());
        || ()
        });
    }
    let songs_list_callback = songs_list.clone();


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


    let admin_refresh_song = {
        Callback::from(move |_event : MouseEvent| {
            web_sys::console::log_1(&format!("on refresh click").into());
            force_refresh_songs(songs_list.clone());
        })
    };

    let show_delete_popup = {  
        let selected_song_to_delete = selected_song_to_delete.clone();
        Callback::from(move |song: Song| selected_song_to_delete.set(Some(song)))
    };

    let hide_delete_popup = {
        let selected_song_to_delete = selected_song_to_delete.clone();
        Callback::from(move |_| selected_song_to_delete.set(None))
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
        let on_refresh_click = on_refresh_click.clone();

        Callback::from(move |input: String| {
            let on_refresh_click = on_refresh_click.clone(); // Re-clone if used inside async

            web_sys::console::log_1(&format!("Validated input: {}", input).into());
            if let Some(mut song) = (*selected_song_to_add).clone() {
                song.singer = Some(input);
                let json_song = serde_json::to_string(&song).expect("Failed to serialize song to JSON");
                web_sys::console::log_1(&format!("full song with singer : {}", json_song).into());
                selected_song_to_add.set(Some(song.clone()));

                wasm_bindgen_futures::spawn_local(async move {
                    let on_refresh_click = on_refresh_click.clone();
                    let config = Config::load();
                    let url = format!("{}/add-song", config.backoffice_url);

                    match Request::post(&url)
                        .header("Content-Type", "application/json")
                        .body(serde_json::to_string(&song).unwrap())
                    {
                        Ok(request) => match request.send().await {
                            Ok(resp) => {
                                if resp.ok() {
                                    web_sys::console::log_1(&"Song successfully sent!".into());
                                    //refresh_chosen_songs(chosen_songs_list);
                                    let event = MouseEvent::new("click").unwrap(); // Create a default MouseEvent
                                    on_refresh_click.emit(event); // Emit MouseEvent
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

 


    html! {
        <div class="container">
            <h1>{ "Karaoke des viviers" }</h1>
            <table>
                <thead>
                    <tr>
                        <th>{ "Artiste" }</th>
                        <th>{ "Chanson" }</th>
                        <th>{ "Chanteur"}</th>
                        <th></th>
                    </tr>
                </thead>
                <ChosenSongsList on_click={show_delete_popup.clone()} songs_list={(*chosen_songs_list_callback).clone()}/>
            </table>
            <p>
            <button onclick={on_refresh_click}>
                { "Refresh Songs" }
            </button>
            </p>
            <table>
                <thead>
                    <tr>
                        <th>{ "Artiste" }</th>
                        <th>{ "Chanson" }</th>
                        <th></th>
                    </tr>
                </thead>
                <SongsList on_click={show_add_popup.clone()} songs_list={(*songs_list_callback).clone()}/>
            </table>
//            { for chosen_song }
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
            if is_admin_page {
                <button onclick={admin_refresh_song}>
                    { "Force Refresh Google Song list" }
                </button>
            }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
