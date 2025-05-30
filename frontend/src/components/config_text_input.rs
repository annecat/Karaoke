use crate::types::bo_config::BoConfig; 

use crate::config::Config; 

use yew::prelude::*;
use gloo_net::http::Request;

pub async fn get_text_config(name:String) -> String {
    let mut res = String::new();
    let content_to_retrieve = BoConfig {
        id: 1,
        name: name,
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
                    web_sys::console::error_1(&format!("data added to content").into());
                    res = data.value;
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


#[derive(Properties, PartialEq)]
pub struct ConfigTextInputProps {
    pub name: String,
}

#[function_component(ConfigTextInput)]

pub fn toggle_button(ConfigTextInputProps {name} : &ConfigTextInputProps) -> Html {
    let input_state = use_state(|| String::new());

    use_effect_with((), {
    let name = name.clone();
    let input_state = input_state.clone();
    move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            let config = get_text_config(name.to_string()).await;
            input_state.set(config);
        });
        || ()
    }
    });


    let on_click = {
        Callback::from( {
            let name= name.clone();
            let input_state = input_state.clone(); // Clone before moving
            move |_| {

                let bo_config = BoConfig {
                    id: 1,
                    name: name.to_string(),
                    value: input_state.to_string(),
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
    
    let on_input = {
        let input_state = input_state.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                input_state.set(input.value());
            }
        })
    };
    
    html! {
        <div id="text_config_input">
            
            <input type="text" size="50"
                value={(*input_state).clone()}
                oninput={on_input}
            />
            <button onclick={on_click}>{ "Valider" }</button>
            
           
        </div>
    }
}
