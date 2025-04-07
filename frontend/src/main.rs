use yew::prelude::*;
use web_sys::window;

mod components {
    pub mod songs_list;
    pub mod chosen_songs_list;
    pub mod popup_add_song;
    pub mod popup_delete_song;
    pub mod suggestions;
    pub mod content;
    pub mod popup_confirm;
    pub mod open_toggle_button;
}

mod types {
    pub mod song;
    pub mod bo_config;
}

mod config;

use crate::components::songs_list::SongsList;
use crate::components::chosen_songs_list::ChosenSongsList;
use crate::components::suggestions::Suggestions;
use crate::components::content::ContentComponent;
use crate::components::open_toggle_button::get_config_open;
use crate::components::open_toggle_button::OpenToggleButton;




#[function_component(App)]
fn app() -> Html {

    let refresh_chosen_songs: UseStateHandle<bool> = use_state(|| false);
    let is_karaoke_open: UseStateHandle<bool> = use_state(|| false);

    let trigger_refresh = {
        let refresh_chosen_songs = refresh_chosen_songs.clone();
        Callback::from(move |_| refresh_chosen_songs.set(true))
    };
    let is_karaoke_open_clone = is_karaoke_open.clone();
    let location = window()
    .and_then(|win: web_sys::Window| win.location().pathname().ok()) // Get the path portion of the URL
    .unwrap_or_else(|| "/".to_string()); // Default to "/" if retrieval fails

    // Check if the current URL contains "/admin"
    let is_admin_page = location.contains("/maestro");



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
                { if *is_karaoke_open || is_admin_page
                     {
                        html! {
                            <ChosenSongsList refresh_trigger={refresh_chosen_songs.clone()}/>
                        }
                    } else {
                        html! {
                            <p>
                                <table class="w3-table w3-striped w3-white" id="chosen-song">
                                    <thead class="w3-red">
                                    <tr>
                                        <th>{"#"}</th>
                                        <th>{"La selection de chanson est fermée"}</th>    
                                    </tr>
                                    </thead>
                                    <tbody>
                                    </tbody>
                                </table>
                            </p>
                        }
                    }
                 }
               
            </div>
             <SongsList on_add={trigger_refresh.clone()} karaoke_open={*is_karaoke_open}/>
                
             <Suggestions />


             { if is_admin_page
                {
                    html! {
                        <p style="center">
                            {"Ouvrir le karaoké :"} <OpenToggleButton />
                        </p>
                    }
                } else {
                   html! {
                   }
               }
            }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
