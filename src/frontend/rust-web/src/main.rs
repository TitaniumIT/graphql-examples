#![allow(non_snake_case)]

use dioxus::prelude::*;

use log::LevelFilter;

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

#[derive(Clone, Copy)]
struct LoadedCategories(bool);


fn App() -> Element {
    use_context_provider(|| Signal::new(LoadedCategories(false)));
    rsx! {
        Router::<Route> {}
    }
}

use get_products::productView;

#[component]
fn Products(selected_id: Signal<String>) -> Element {
    let mut future: Resource<Result<Vec<productView>, String>> = use_resource(move || async move {
        let client = reqwest::Client::new();

        let variables = get_products::Variables {
            first: Some(5),
            after: None,
            before: None,
            last: None,
        };

        let result =
            post_graphql::<GetProducts, _>(&client, "http://localhost:7265/graphql", variables)
                .await
                .unwrap();

        if !result.errors.is_some() {
            Ok(result
                .data
                .and_then(|p| 
                    Some(p.products_relay
                        .edges
                        .into_iter()
                        .map(|edge| edge.node)
                        .collect())
                    )
                .unwrap())
        } else {
            Err("Failed".to_string())
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
    let mut loaded = use_context::<Signal<LoadedCategories>>();

    let mut future: Resource<Result<get_product::>, String>> = use_resource(move || async move {
        let client = reqwest::Client::new();

        let variables = get_product::Variables {
           
        };

        let result =
            post_graphql::<GetProducts, _>(&client, "http://localhost:7265/graphql", variables)
                .await
                .unwrap();

        if !result.errors.is_some() {
            Ok(result
                .data
                .and_then(|p| 
                    Some(p.products_relay
                        .edges
                        .into_iter()
                        .map(|edge| edge.node)
                        .collect())
                    )
                .unwrap())
        } else {
            Err("Failed".to_string())
        }
    });

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

use graphql_client::{reqwest::post_graphql, Error, GraphQLQuery};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphqls",
    query_path = "src/models/getproducts.graphql",
    normalization = "rust"
)]
pub struct GetProducts;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphqls",
    query_path = "src/models/getproduct.graphql",
    normalization = "rust"
)]
pub struct GetProduct;
