#![allow(non_snake_case)]
use std::fmt::Display;

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

#[derive(Clone)]
struct LoadedCategories(Option<Vec<get_product::categoryView>>);

#[derive(Clone, Debug)]
enum CustomerId {
    ValidEmail(EmailAddress),
    Invalid(String),
    Default(),
}

impl Display for CustomerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ValidEmail(address) => f.write_str(&address.to_string()),
            Self::Invalid(data) => f.write_str(data),
            Self::Default() => f.write_str("Type ypour email"),
        }
    }
}

fn App() -> Element {
    use_context_provider(|| Signal::new(LoadedCategories(None)));
    use_context_provider(|| Signal::new(CustomerId::Default()));
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
                .and_then(|p| {
                    Some(
                        p.products_relay
                            .edges
                            .into_iter()
                            .map(|edge| edge.node)
                            .collect(),
                    )
                })
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
        div {
            class:"card",
            h5 {
                class:" card-header",
                "Basket"
            }
            div {
                class:"card-body",
                Basket{}
            }
        }
    }
}

#[component]
fn Product(selected_id: Signal<String>) -> Element {
    let mut loaded = use_context::<Signal<LoadedCategories>>();

    let future: Resource<Result<get_product::ResponseData, String>> =
        use_resource(move || async move {
            let client = reqwest::Client::new();

            let variables = get_product::Variables {
                product_id: format!("{selected_id}"),
                load_categories: loaded.read().0.is_none(),
            };

            let result =
                post_graphql::<GetProduct, _>(&client, "http://localhost:7265/graphql", variables)
                    .await;

            if let Ok(result) = result {
                if !result.errors.is_some() {
                    let data = result.data.unwrap();
                    if let Some(categories) = data.categories.clone() {
                        *loaded.write() = LoadedCategories(Some(categories));
                    }
                    Ok(data)
                } else {
                    Err(result
                        .errors
                        .unwrap()
                        .iter()
                        .map(|e| e.message.clone())
                        .collect::<Vec<_>>()
                        .join(","))
                }
            } else {
                Err(format!("process error: {:?}", result.err()))
            }
        });

    if *selected_id.read() != "" {
        match &*future.read_unchecked() {
            Some(Ok(response)) => rsx! {
                {
                response.clone().product.and_then(|product| {
                    rsx!{
                    div {
                        class:"mb-3",
                        label {
                            r#for:"name",
                            class:"form-label",
                            "Product name"
                        },
                        input {
                            class:"form-control",
                            readonly:true,
                            id:"name",
                            value:"{product.name}"}
                      }
                    div {
                        class:"mb-3",
                        label {
                             r#for:"description",
                             class:"form-label",
                            " Product description"
                        },
                        input {
                             class:"form-control",
                             readonly:true,
                             id:"description",
                             value:"{product.description}"
                        }
                    }
                    div {
                        class:"card",
                        div {
                            class:"card-header",
                            "Product categories"
                        },
                        ul {
                            class:"list-group list-group-flush",
                            {
                              rsx!{
                                {
                                  loaded.read().0.clone().unwrap()
                                    .iter().map( |category |{
                                        let classes = if product.hasCategory(&category) {
                                            "list-group-item active"
                                        } else {
                                             "list-group-item"
                                        };
                                        rsx!{
                                            li {
                                                class:classes,
                                                "{category.name}"
                                            }
                                        }
                                    }
                                    )
                                 }
                              }
                            }
                        }
                    }
                  }
                }).or_else(|| rsx! {
                    tr {
                        td { colspan:"4","Loading"}
                    }
                } )
            }
            },
            Some(Err(err)) => rsx! {
                tr {
                    td {
                    colspan:"4",
                    "Error {err}"
                    }
                }
            },
            None => rsx! {
                tr {
                    td { colspan:"4","Loading"}
                }
            },
        }
    } else {
        rsx! {
            div {
                "No product selected"
            }
        }
    }
}

fn Basket() -> Element {
    let mut customer_id = use_context::<Signal<CustomerId>>();
    rsx! (
        div{
            input {
                class:"form-control",
                id:"customerid",
                value:"{customer_id}",
                required:true,
                r#type:"email",
                oninput: move |event|{
                    let result =  EmailAddress::new(&event.value());
                    if let Ok(email) = result {
                        customer_id.set(CustomerId::ValidEmail(email));
                    } else {
                        customer_id.set(CustomerId::Invalid(event.value()));
                    }
                }
            },
            if let CustomerId::Invalid(value) = customer_id.read() {
                {
                   rsx!{ span {
                    "Invalid email {value}"
                    }
                    }
                }
            }
        }
        table {
            class: "table",
            caption {
                "Basket for"
            }
            thead {
                    th {
                        scope:"col",
                        "Name"
                    }
                    th {scope:"col","ordered"}
                    th {scope:"col","intansit"}
                    th {scope:"col","deliverd"}
                    th {scope:"col","cancelled"}
            }
        }
    )
    //     <table class="table">
    //     <caption>customer:{{customerId}}</caption>
    //     <thead>
    //         <th scope="col">Name</th>
    //         <th scope="col">ordered</th>
    //         <th scope="col">intansit</th>
    //         <th scope="col">deliverd</th>
    //         <th scope="col">cancelled</th>
    //     </thead>
    //     <tbody>
    //             <tr *ngFor="let product of inBasket" scope="row" >
    //                 <td>{{product.name}}</td>
    //                 <td>{{product.nrOrderd}}</td>
    //                 <td>{{ product.nrInTransit }}</td>
    //                 <td>{{ product.nrDeliverd }}</td>
    //                 <td>{{ product.nrCancelled }}</td>
    //             </tr>
    //     </tbody>
    // </table>
}

use graphql_client::{reqwest::post_graphql, Error, GraphQLQuery};
use shared_types::EmailAddress;

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
    normalization = "rust",
    response_derives = "Debug,Clone"
)]
pub struct GetProduct;

impl get_product::productDetailView {
    pub fn hasCategory(&self, category: &get_product::categoryView) -> bool {
        self.selected_categories.iter().any(|c| c.id == category.id)
    }
}
