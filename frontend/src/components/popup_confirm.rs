use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PopupProps {
    pub message: String, 
    pub on_close: Callback<()>,

}


#[function_component(PopupConfirm)]
pub fn popup_confirm(props: &PopupProps) -> Html {

    let on_close = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| on_close.emit(()))
    };

    html! {
        <div class="popup">
            <div class="popup-content">
                <h3>{ &props.message }</h3>                
                <div class="popup-buttons">
                    <button onclick={on_close}>{ "Ok" }</button>
                </div>
            </div>
        </div>
    }
}