use crate::types::bo_config::BoConfig; 

use crate::config::Config; 

use yew::prelude::*;
use gloo_net::http::Request;

pub async fn get_config_open() -> bool {
    let mut res = false;
    let content_to_retrieve = BoConfig {
        id: 1,
        name: "open".to_string(),
        value: "".to_string(),
    };

    // Replace the URL with your backend endpoint
    let config: Config = Config::load();
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



#[function_component(OpenToggleButton)]
pub fn toggle_button() -> Html {
    let toggle_state = use_state(|| false);


    use_effect_with((), {
        let toggle_state = toggle_state.clone(); // Clone before moving
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if get_config_open().await {
                    toggle_state.set(true);
                }
            });
        }
    });


    let on_toggle = {
        Callback::from( {
            let toggle_state = toggle_state.clone(); // Clone before moving
            move |_| {
                let new_state = !*toggle_state;
                toggle_state.set(new_state);

                let bo_config = BoConfig {
                    id: 1,
                    name: "open".to_string(),
                    value: if new_state { "yes".to_string() } else { "no".to_string() },
                };
                
                let config: Config = Config::load();
                let url = format!("{}/change-config", config.backoffice_url);
            
                wasm_bindgen_futures::spawn_local(async move {
                    let response = Request::post(&url)
                        .header("Content-Type", "application/json")
                        .json(&bo_config)
                        .unwrap()
                        .send()
                        .await;

                    match response {
                        Ok(_) => gloo::console::log!("Config updated successfully!"),
                        Err(err) => gloo::console::error!(format!("Failed to update config: {err}")),
                    }
                });
            }
        })
    };

    html! {
        <button onclick={on_toggle} class={if *toggle_state { "bg-green-500" } else { "bg-red-500" }}>
            { if *toggle_state { "Yes" } else { "No" } }
        </button>
    }
}
