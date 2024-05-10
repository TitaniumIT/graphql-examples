#![allow(non_snake_case)]

use std::{sync::Arc, thread::Scope};

use cynic::{
    http::{CynicReqwestError, ReqwestExt},
    QueryBuilder,
};
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

#[derive(cynic::QueryVariables, Debug)]
pub struct GetProductsVariables<'a> {
    pub after: Option<&'a str>,
    pub before: Option<&'a str>,
    pub first: Option<i32>,
    pub last: Option<i32>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "QueryType", variables = "GetProductsVariables")]
pub struct GetProducts {
    #[arguments(first: $first, after: $after, last: $last, before: $before)]
    pub products_relay: Option<ProductConnection>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct ProductConnection {
    pub edges: Option<Vec<Option<ProductEdge>>>,
    pub total_count: Option<i32>,
    pub page_info: PageInfo,
    pub items: Option<Vec<Option<Product>>>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct ProductEdge {
    pub node: Option<Product>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: String,
    pub in_stock: i32,
    pub actions_allowed: Option<Vec<Option<String>>>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct PageInfo {
    pub has_next_page: bool,
    pub has_previous_page: bool,
    pub start_cursor: Option<String>,
    pub end_cursor: Option<String>,
}

#[component]
fn Products() -> Element {
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

    let mut selected_id = use_signal(|| "".to_string());
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
