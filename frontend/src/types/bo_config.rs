use serde::Deserialize;
use serde::Serialize;
use yew::Properties;

#[derive(Clone, PartialEq, Serialize, Deserialize, Properties)]
pub struct BoConfig {
    pub id: i32,
    pub name: String,
    pub value: String
}

