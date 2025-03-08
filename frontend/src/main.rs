use yew::prelude::*;
use gloo_net::http::Request;
    

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

use crate::config::Config;
use crate::components::songs_list::{SongsList, refresh_songs, force_refresh_songs};
use crate::components::chosen_songs_list::ChosenSongsList;
use crate::components::suggestions::Suggestions;
use crate::components::content::ContentComponent;

use crate::types::song::Song;

#[function_component(App)]
fn app() -> Html {

    let refresh_chosen_songs: UseStateHandle<bool> = use_state(|| false);

    let trigger_refresh = {
        let refresh_chosen_songs = refresh_chosen_songs.clone();
        Callback::from(move |_| refresh_chosen_songs.set(true))
    };

    html! {
        <div class="w3-main">
            <h1 style="text-align:center;">{ "Karaoke des viviers" }</h1>
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
                
                <ChosenSongsList refresh_trigger={refresh_chosen_songs.clone()}/>
               
            </div>
             <SongsList on_add={trigger_refresh.clone()}/>
                
             <Suggestions />
  
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
