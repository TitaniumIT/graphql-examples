#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;

#[cynic::schema("example")]
mod schema {}

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

struct Product {
    name: String,
    description: String,
    in_stock: i32,
}

#[component]
fn Products() -> Element {
    let products = vec![
        Product {
            name: "P1".to_string(),
            description: "Desc".to_string(),
            in_stock: 4,
        },
        Product {
            name: "P2".to_string(),
            description: "Desc 1".to_string(),
            in_stock: 4,
        },
        Product {
            name: "P3".to_string(),
            description: "Desc 2".to_string(),
            in_stock: 4,
        },
        Product {
            name: "P4".to_string(),
            description: "Desc 4".to_string(),
            in_stock: 4,
        },
    ];

    let loading = false;
    let mut selected_id = use_signal(||"P3".to_string());

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
          if ! loading {
                tbody {
                    for product in products {
                        tr {
                            class: if product.name == *selected_id.read() { "table-active" } else {""},
                            onclick: move |_| {
                                *selected_id.write() = product.name.clone();
                            },
                            td { "{product.name}"}
                            td { "{product.description}"}
                            td { "{product.in_stock}"}
                            td { 
                                
                            }
                        }
                    }
                }
            } else {
                tbody {
                    tr {
                       td {
                        colspan:"4",
                        "Loading"
                       }
                    }
                }
            }
      }
    }
}

#[component]
fn Home() -> Element {
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
                }
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



