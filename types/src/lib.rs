use yew::prelude::Properties;

#[derive(Properties, PartialEq, Clone)]
pub struct Token {
    pub name: String,
    pub value: String,
}
