//use actix_cors::Cors;
use shuttle_runtime::SecretStore;
use actix_web::web::ServiceConfig;
use actix_web::{get, Responder, HttpResponse};
use shuttle_actix_web::ShuttleActixWeb;

mod google_sheet_response;
mod song;


#[get("/")]
async fn hello() -> impl Responder {
        HttpResponse::Ok().body("Hello world!")
}


#[get("/song-update")]
async fn song_update() -> impl Responder {


    match google_sheet_response::fetch_google_sheet().await {
        Ok(content) => {
            let song_list = content.transform_google_format_to_song();
            song::cache_songs(&song_list).expect("Error saving the list of songs");    
            HttpResponse::Ok().body("Ok :p")
        },
        Err(err) => HttpResponse::InternalServerError().body(format!("Error fetching document: {}", err)),
    }
}

#[get("/song-data")]
async fn song_data() -> impl Responder {

    let song_list;
    // if the song collection does'nt exists we download it ortherwise we use the cache one
    if !song::songs_list_cache_exists() {
        let content = google_sheet_response::fetch_google_sheet().await.expect("Error fetching document");
            song_list = content.transform_google_format_to_song();
            song::cache_songs(&song_list).expect("Error saving the list of songs");
    } else {
        song_list = song::read_from_cache().expect("Error reading from file to load the songs list");
    }
    HttpResponse::Ok().json(song_list)
}


// The entry point for Shuttle deployment
#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        secrets.into_iter().for_each(|(key, val)| {
            std::env::set_var(key, val);
        });
        cfg.service(song_data);
        cfg.service(song_update);
    };

    Ok(config.into())
}

/* 
#[cfg(not(target_env = "shuttle"))]
#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8000") // TODO : Replace with a config depending on env
            .allowed_methods(vec!["GET", "POST"]) // Restrict to needed methods
            .allowed_headers(vec!["Content-Type", "Authorization"]) // Only necessary headers
            .max_age(3600);
        App::new()
            .wrap(cors)
            .service(hello)
            .service(song_data)
            .service(song_update)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
*/