#![allow(non_snake_case)]

mod schema;

use cynic::{
    http::{CynicReqwestError, ReqwestExt},
    QueryBuilder,
};
use dioxus::prelude::*;
use log::LevelFilter;

use crate::schema::{GetProducts, GetProductsVariables};

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Products(selected_id: Signal<String>) -> Element {
    let mut future = use_resource(move || async move {
        let client = reqwest::Client::new();
        let query = GetProducts::build(GetProductsVariables {
            first: Some(5),
            after: None,
            before: None,
            last: None,
        });
        let result = client
            .post("http://localhost:7265/graphql")
            .run_graphql(query)
            .await
            .unwrap();
        if !result.errors.is_some() {
            Ok(result
                .data
                .and_then(|p| p.products_relay)
                .and_then(|c| c.edges)
                .and_then(|e| {
                    Some(
                        e.into_iter()
                            .map(|edge| edge.and_then(|e| e.node))
                            .map(|p| p.unwrap())
                            .collect::<Vec<_>>(),
                    )
                })
                .unwrap())
        } else {
            Err(CynicReqwestError::ErrorResponse(
                reqwest::StatusCode::OK,
                "Failed".to_string(),
            ))
        }
    });

    rsx! {
     table {
         class:"table table-sm",
         thead {
           class:"table-light",
           th { scope:"col",  "Name" }
           th { scope:"col",  "Description" }
           th { scope:"col",  "In Stock" }
           th { scope:"col",  "Actions" }
           }
           tbody {
               match &*future.read_unchecked() {
                   Some(Ok(products)) => rsx! {
                        { products.iter().map(|product| {
                             let id = product.id.clone();
                             rsx!{
                                tr {
                                        key: "{product.id}",
                                        class: if product.id == *selected_id.read() { "table-active" } else {""},
                                            onclick: move |_| {
                                                *selected_id.write() = id.clone();
                                                },
                                        td { "{product.name}"}
                                        td { "{product.description}"}
                                        td { "{product.in_stock}"}
                                        td {      }
                                }
                            }
                       }) }
                    },
                    Some(Err(_)) => rsx! {
                            tr {
                                td {
                                colspan:"4",
                                "Error"
                                }
                            }
                        },
                    None => rsx! {
                            tr {
                                td { colspan:"4","Loading"}
                            }
                        }
                }
            }
        }
    }
}

#[component]
fn Home() -> Element {
    let mut selected_id = use_signal(|| "".to_string());

    rsx! {
        div {
            class:"card",
            h5 {
                class:" card-header",
                "Shop"
            }
            div {
                class:"card-body",
                Products {
                    selected_id: selected_id
                }
            }
        }
        div {
            class:"card",
            h5 {
                class:" card-header",
                "Product details"
            }
            div {
                class:"card-body",
                Product {
                    selected_id: selected_id
                }
            }
        }
    }
}

#[component]
fn Product(selected_id: Signal<String>) -> Element {
    if *selected_id.read() != "" {
        rsx! {
            div {
              class: "mb-3",
              label {
                r#for:"name",
                class: "form-label",
                "Product name"
              },
              input {
                class:"form-control",
                readonly:true,
                id:"name",
                value: "{selected_id}"
              }
            }
        }
    } else {
        rsx! {
            div {
                "No product selected"
            }
        }
    }
}

// <div class="card">
//     <h5 class="card-header">Shop</h5>
//     <div class="card-body">
//         <app-products (selectedProductId)="show($event)">
//         </app-products>
//     </div>
// </div>
// <div class="card">
//     <h5 class="card-header">Product details</h5>
//     <div class="card-body">
//         <app-product [productId]="selectedProduct"></app-product>
//     </div>
// </div>
// <div class="card">
//     <h5 class="card-header">My products</h5>
//     <div class="card-body">
//         <app-basket></app-basket>
//     </div>
// </div>
