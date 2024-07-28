#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Input(value:String,label:String,readonly:bool) -> Element{
    rsx!{
        div {
            class:"mb-3",
            label {
                r#for:"name",
                class:"form-label",
                "{label}"
            },
            input {
                class:"form-control",
                readonly,
                id:"name",
                value:"{value}"
            }
          }
    }
}

#[component]
pub fn Card(title:String, children : Element) -> Element {
    rsx!{
        div {
            class:"card",
            h5 {
                class:" card-header",
                "{title}"
            }
            div {
                class:"card-body",
                {children}
            }
        }
}
}