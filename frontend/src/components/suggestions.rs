use yew::prelude::*;
use crate::config::Config;
use gloo_net::http::Request;

use serde::Deserialize;
use serde::Serialize;
use yew::Properties;

#[derive(Clone, PartialEq, Serialize, Deserialize, Properties)]

pub struct Suggestion {
    pub id: i32,
    pub content: String,

}


#[function_component(Suggestions)]
pub fn suggestions() -> Html {
    let input_value: UseStateHandle<String> = use_state(|| "".to_string()); // State to hold the input text
    let answer: UseStateHandle<String> = use_state(|| "".to_string()); // State to hold the answer

    let on_validate: Callback<MouseEvent> = {
        let input_value = input_value.clone();
        let answer = answer.clone();
        Callback::from(move |_| {
            let input_value = input_value.clone();
            let answer = answer.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let config = Config::load();
                let url = format!("{}/add-suggestion", config.backoffice_url);
                let suggestion = Suggestion {
                    id :0,
                    content : (*input_value).clone()
                };
                match Request::post(&url)
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&suggestion).unwrap())
                {
                    Ok(request) => match request.send().await {
                        Ok(resp) => {
                            if resp.ok() {
                                web_sys::console::log_1(&"Suggestions successfully sent!".into());
                                answer.set("Votre suggestion a été enregistrée !".to_string());
                            } else {
                                web_sys::console::error_1(&format!("Failed to send Suggestions: {:?}", resp).into());
                                answer.set("Echec de l'envoi de suggestion :'(".to_string());
                            }
                        }
                        Err(err) => {
                            web_sys::console::error_1(&format!("Network error: {}", err).into());
                            answer.set("Echec de l'envoi de suggestion :'(".to_string());
                        }
                    },
                    Err(err) => {
                        web_sys::console::error_1(&format!("Failed to create request: {}", err).into());
                        answer.set("Echec de l'envoi de suggestion :'(".to_string());
                    }
                }
            });
        })
    };

    let on_input = {
        let input_value = input_value.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlTextAreaElement>() {
                input_value.set(input.value());
            }
        })
    };

    html! {
        <div id="suggestion">
            <h1>{ "Des suggestions ?" }</h1>
            <textarea
                value={(*input_value).clone()}
                oninput={on_input}
                maxwidth="2000"
            />
            <button onclick={on_validate}>{ "Valider" }</button>
            <p>{ (*answer).clone() }</p>
        </div>
    }
}