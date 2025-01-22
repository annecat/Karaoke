use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PopupProps {
    pub on_validate: Callback<()>, // Callback for validate button
    pub on_cancel: Callback<()>,       // Callback for cancel button
}

#[function_component(PopupDeleteSong)]
pub fn popup_delete_song(props: &PopupProps) -> Html {
    
    let on_validate = {
        let on_validate = props.on_validate.clone();
        Callback::from(move |_event: MouseEvent| on_validate.emit(()))
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
                <h3>{ "Supprimer" }</h3>
                <p>{"Voulez-vouz vraiment supprimer ?"}</p>
                <div class="popup-buttons">
                    <button onclick={on_validate}>{ "Valider" }</button>
                    <button onclick={on_cancel}>{ "Annuler" }</button>
                </div>
            </div>
        </div>
    }
}