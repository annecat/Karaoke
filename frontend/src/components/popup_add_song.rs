use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PopupProps {
    pub on_validate: Callback<String>, // Callback for validate button
    pub on_cancel: Callback<()>,       // Callback for cancel button
}


#[function_component(PopupAddSong)]
pub fn popup_add_song(props: &PopupProps) -> Html {
    let input_value = use_state(|| "".to_string()); // State to hold the input text

    
    let on_input = {
        let input_value = input_value.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            input_value.set(input.value());
        })
    };

    let on_validate: Callback<MouseEvent> = {
        let input_value = input_value.clone();
        let on_validate: Callback<String> = props.on_validate.clone();
        Callback::from(move |_| on_validate.emit((*input_value).clone()))
    };

    let on_cancel = {
        let on_cancel = props.on_cancel.clone();
        Callback::from(move |_event: MouseEvent| {
            on_cancel.emit(());
        })  
    };

    html! {
        <div class="popup">
            <div class="popup-content">
                <h3>{ "Entrez votre nom :" }</h3>
                <input
                    type="text"
                    placeholder="Enter text"
                    value={(*input_value).clone()}
                    oninput={on_input}
                />
                <div class="popup-buttons">
                    <button onclick={on_validate}>{ "Valider" }</button>
                    <button onclick={on_cancel}>{ "Annuler" }</button>
                </div>
            </div>
        </div>
    }
}