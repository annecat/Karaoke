use yew::prelude::*;
use log::error;
use gloo_net::http::Request;
use serde::Deserialize;
use serde::Serialize;
use wasm_bindgen_futures::spawn_local;

mod components {
    pub mod songs_list;
    pub mod chosen_songs_list;
    pub mod popup_add_song;
    pub mod popup_delete_song;
    pub mod suggestions;
    pub mod content;
    pub mod popup_confirm;
}

mod types {
    pub mod song;
}

mod config;

use crate::components::songs_list::SongsList;
use crate::components::chosen_songs_list::ChosenSongsList;
use crate::components::suggestions::Suggestions;
use crate::components::content::ContentComponent;
use crate::config::Config; 


#[derive(Clone, PartialEq, Serialize, Deserialize, Properties)]
pub struct BoConfig {
    pub id: i32,
    pub name: String,
    pub value: String
}



async fn get_config_open() -> bool {
    let mut res = false;
    let content_to_retrieve = BoConfig {
        id: 1,
        name: "open".to_string(),
        value: "".to_string(),
    };

    // Replace the URL with your backend endpoint
    let config = Config::load();
    let url = format!("{}/get-config", config.backoffice_url);

    if let Ok(request) = Request::post(&url)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&content_to_retrieve).unwrap())
    {
        match request.send().await {
            Ok(response) => match response.json::<BoConfig>().await {
                Ok(data) => {
                    match data.value.as_str() {
                        "yes" => res = true,
                        _ => res = false,
                    }
                    web_sys::console::error_1(&format!("data added to content").into());
                }
                Err(err) => web_sys::console::error_1(&format!("get-content JSON parse error: {}", err).into()),
            },
            Err(err) => web_sys::console::error_1(&format!("Request send error: {}", err).into()),
        }
    } else {
        web_sys::console::error_1(&"Failed to create request.".into());
    }

    res
}


#[function_component(App)]
fn app() -> Html {

    let refresh_chosen_songs: UseStateHandle<bool> = use_state(|| false);
    let is_karaoke_open: UseStateHandle<bool> = use_state(|| false);

    let trigger_refresh = {
        let refresh_chosen_songs = refresh_chosen_songs.clone();
        Callback::from(move |_| refresh_chosen_songs.set(true))
    };
    let is_karaoke_open_clone = is_karaoke_open.clone();

    use_effect_with((), move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            if get_config_open().await {
                is_karaoke_open_clone.set(true);
            }
        });
    });

    html! {
        <div class="w3-main">
            <h1 style="text-align:center;">{ "Carpe Dièse" }</h1>
            <div class="w3-row-padding w3-margin-bottom">
                <div class="w3-half">
                    <a href="#chosen-song" style="text-decoration: none;">
                        <div class="w3-container w3-red w3-padding-16">
                            <div class="w3-left"><i class="fa fa-play-circle w3-xxxlarge"></i></div>
                            <div class="w3-right">
                            <h3></h3>
                            </div>
                            <div class="w3-clear"></div>
                            <h4>{"Prochains titres"}</h4>
                        </div>
                    </a>
                </div>
                <div class="w3-half">
                    <a href="#songs-list" style="text-decoration: none;">

                        <div class="w3-container w3-blue w3-padding-16">
                            <div class="w3-left"><i class="fa fa-music w3-xxxlarge"></i></div>
                            <div class="w3-right">
                            <h3></h3>
                            </div>
                            <div class="w3-clear"></div>
                            <h4>{"Titres disponibles"}</h4>
                        </div>
                    </a>
                </div>    
            </div>


            <div class="w3-container">
            <ContentComponent content_id="text_intro" />
              
            </div>
            <div class="w3-container">   
                { if *is_karaoke_open
                     {
                        html! {
                            <ChosenSongsList refresh_trigger={refresh_chosen_songs.clone()}/>
                        }
                    } else {
                        html! {
                            <p>{ "La selection de chanson est fermée" }</p>
                        }
                    }
                 }
               
            </div>
             <SongsList on_add={trigger_refresh.clone()} karaoke_open={*is_karaoke_open}/>
                
             <Suggestions />
  
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
