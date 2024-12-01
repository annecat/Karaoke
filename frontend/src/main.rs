
use serde::Deserialize;
use gloo_net::http::Request;
use yew::prelude::*;


#[derive(Deserialize, Clone, PartialEq)]
struct Song {
    id: usize,
    artist: String,
    name: String,
    lyrics_url: String,
}

#[derive(Properties, PartialEq)]
struct SongsListProps {
    songs: Vec<Song>,
    on_click: Callback<Song>
}

#[derive(Properties, PartialEq)]
struct SongChosenProps {
    song: Song,
}

#[function_component(SongChosen)]
fn song_details(SongChosenProps { song }: &SongChosenProps) -> Html {
    html! {
        <div>
            <h3>{ song.name.clone() }</h3>
        </div>
    }
}

#[function_component(SongsList)]
fn songs_list(SongsListProps { songs, on_click }: &SongsListProps) -> Html {
    
    songs.iter().map(|song| {
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
                <td>{song.name.clone()}</td>
                <td onclick={on_song_select}>{ "choisir"}</td>
            </tr>
        }
    }).collect()
}



#[function_component(App)]
fn app() -> Html {

    let songs = use_state(|| vec![]);
    {
        let songs = songs.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_songs: Vec<Song> = Request::get("http://127.0.0.1:8080/song-data")
                    .send()
                    .await
                    .unwrap() // TODO : error to handle
                    .json()
                    .await
                    .unwrap(); // TODO : error to handle
                songs.set(fetched_songs);
            });
            || ()
        });
    }
    let selected_song = use_state(|| None);

    let on_song_select = {
        let selected_song = selected_song.clone();
        Callback::from(move |song: Song| {
            selected_song.set(Some(song))
        })
    };
    let chosen_song = selected_song.as_ref().map(|song| html! {
        <SongChosen song={song.clone()} />
    });

    html! {
        <div class="container">
            <h1>{ "Ouaiiiiii" }</h1>
            <table>
                <thead>
                    <tr>
                        <th>{ "Artiste" }</th>
                        <th>{ "Chanson" }</th>
                        <th></th>
                    </tr>
                </thead>
                <SongsList songs={(*songs).clone()} on_click={on_song_select.clone()}/>
            </table>
            { for chosen_song }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
