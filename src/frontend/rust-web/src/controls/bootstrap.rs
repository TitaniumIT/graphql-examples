#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Input(value:String,label:String,readonly:bool,placeholder:Option<String>) -> Element{
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
                value:"{value}",
                placeholder: if let Some(placeholder) = placeholder { placeholder} 
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

#[component]
pub fn Table(caption:Option<String>,columns:Vec<String>,body:Element) -> Element {
    rsx! {
             table {
                 class:"table table-sm",
                 if let Some(caption) = caption {
                    caption {
                        "{caption}"
                    }
                 }
                 thead {
                   class:"table-light",
                   for header in columns {
                        th { scope:"col",  "{header}" }
                   }
                   }
                tbody {
                    { body }
                }
            }
        }
    }
