
use serde::Deserialize;
use gloo_net::http::Request;
use yew::prelude::*;

#[derive(Deserialize, Debug, Clone)]
struct GoogleSheetResponse {
    range: String,
    majorDimension: String,
    values: Vec<Vec<String>>, // Nested vectors for rows and columns
}

#[function_component(App)]
fn app() -> Html {
    let sheet_data = use_state(|| None::<GoogleSheetResponse>);
    {
        let sheet_data = sheet_data.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_sheet: GoogleSheetResponse = Request::get("http://172.31.2.41:8080/sheet-data")
                    .send()
                    .await
                    .unwrap()
                    .json::<GoogleSheetResponse>()
                    .await
                    .unwrap();
                sheet_data.set(Some(fetched_sheet));
            });
            || ()
        });
    }


    html! {
        <div>
            <h1>{ "Google Sheet Data" }</h1>
            {
                if let Some(data) = (*sheet_data).clone() {
                    html! {
                        <table>
                            <thead>
                                <tr>
                                    { for data.values.get(0).unwrap_or(&vec![]).iter().map(|header| html! { <th>{ header }</th> }) }
                                </tr>
                            </thead>
                            <tbody>
                                { for data.values.iter().skip(1).map(|row| html! {
                                    <tr>
                                        { for row.iter().map(|cell| html! { <td>{ cell }</td> }) }
                                    </tr>
                                }) }
                            </tbody>
                        </table>
                    }
                } else {
                    html! { <p>{ "Loading..." }</p> }
                }
            }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
    