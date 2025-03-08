use yew::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use gloo_net::http::Request;
use gloo_utils;
use web_sys::window;
use crate::config::Config;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub html: String,
}

#[function_component(SafeHtml)]
pub fn safe_html(props: &Props) -> Html {
    let div = gloo_utils::document().create_element("div").unwrap();
    div.set_inner_html(&props.html.clone());

    Html::VRef(div.into())
}


#[derive(Clone, PartialEq, Serialize, Deserialize, Properties)]
pub struct Content {
    pub id: String,
    pub content_text: String,
}


#[derive(Properties, PartialEq)]
pub struct ContentComponentProps {
    pub content_id : String
}

#[function_component(ContentComponent)]
pub fn content_component(ContentComponentProps { content_id }: &ContentComponentProps) -> Html {
    // State to hold the content
    let content: UseStateHandle<Option<Content>> = use_state(|| None as Option<Content>);
    // For example, manage a "logged_in" state (in a real app, use a context or auth provider)

    let location = window()
    .and_then(|win| win.location().pathname().ok()) // Get the path portion of the URL
    .unwrap_or_else(|| "/".to_string()); // Default to "/" if retrieval fails

    // Check if the current URL contains "/admin"
    let logged_in = location.contains("/maestro");

    {
        // Fetch content on mount (using an effect)
        let content = content.clone();
        let content_id = content_id.clone();
        use_effect_with((), move |_| {
                spawn_local(async move {
                    let content_to_retrieve = Content {
                        id : content_id,
                        content_text : "".to_string()
                    };
                    // Replace the URL with your backend endpoint
                    let config = Config::load();
                    let url = format!("{}/get-content", config.backoffice_url);
                    if let Ok(request) = Request::post(&url)
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&content_to_retrieve).unwrap())
                {
                    match request.send().await {
                        Ok(response) => match response.json::<Content>().await {
                            Ok(data) => 
                                {
                                    content.set(Some(data));
                                    web_sys::console::error_1(&format!("data added to content").into());
                                },
                            Err(err) => web_sys::console::error_1(&format!("get-content JSON parse error: {}", err).into()),
                        },
                        Err(err) => web_sys::console::error_1(&format!("Request send error: {}", err).into()),
                    }
                } else {
                    web_sys::console::error_1(&"Failed to create request.".into());
                }
                });
                || ()
            },
        );
    }

    // Local state for editing text if needed
    let edit_text = {
        let content = content.clone();
        use_state(|| content.as_ref().map(|c| c.content_text.clone()).unwrap_or_default())
    };

    {
        let content = content.clone();
        let edit_text = edit_text.clone();
        use_effect_with(content, move |content| {
            if let Some(content_data) = content.as_ref() {
                edit_text.set(content_data.content_text.clone());
            }
            || ()
        });
    }

    let on_edit_change = {
        let edit_text = edit_text.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            edit_text.set(input.value());
        })
    };


    let on_save = {
        let edit_text = edit_text.clone();
        let content_id = content_id.clone();

        Callback::from(move |_| {
            web_sys::console::log_1(&format!("Saving: {}", *edit_text).into());
            let input_value = edit_text.clone();
            let content_id = content_id.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let config = Config::load();
                let url = format!("{}/add-content", config.backoffice_url);
                let content_to_send = Content {
                    id : content_id,
                    content_text : (*input_value).clone()
                };
                web_sys::console::error_1(&format!("Content text to send: {:?}", input_value).into());

                match Request::post(&url)
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&content_to_send).unwrap())
                {
                    Ok(request) => match request.send().await {
                        Ok(resp) => {
                            if resp.ok() {
                                web_sys::console::log_1(&"Content successfully sent!".into());
                            } else {
                                web_sys::console::error_1(&format!("Failed to send Content: {:?}", resp).into());
                            }
                        }
                        Err(err) => {
                            web_sys::console::error_1(&format!("Content Network error: {}", err).into());
                        }
                    },
                    Err(err) => {
                        web_sys::console::error_1(&format!("Content Failed to create request: {}", err).into());
                    }
                }
            });
        })
    };

    html! {
        <div>
            {
                if logged_in {
                    // Render editing UI if logged in.
                    html! {
                        <div>
                            <h3>{ "Edit Content" }</h3>
                            <textarea
                                value={(*edit_text).clone()}
                                oninput={on_edit_change.clone()}
                                rows="10"
                                cols="50"
                            />
                            <br/>
                            <button onclick={on_save}>{ "Save" }</button>
                        </div>
                    }
                } else {
                    // Render read-only view.
                    html! {
                        <div>
                        {
                                if let Some(ref c) = *content {
                                    html! {
                                        <div style="border: 1px solid #ccc; padding: 1em;">
                                        <SafeHtml html={c.content_text.clone()} />
                                        
                                        </div>

                                    }
                                } else {
                                    html! {
                                        <div></div>
                                    }
                                }
                            }
                        </div>
                    }
                }
            }
        </div>
    }
}