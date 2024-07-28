#![allow(non_snake_case)]
use controls::bootstrap::Card;
use dioxus::prelude::*;
use graphql_client::reqwest::post_graphql;
use log::LevelFilter;
use std::fmt::Display;

use models::{
    buy_product::{self},
    get_product,
    get_products::{self, productView},
    BuyProduct, GetProduct, GetProducts,
};
use shared_types::EmailAddress;

mod controls;
mod models;

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

#[derive(Clone, Debug, PartialEq)]
enum CustomerId {
    ValidEmail(EmailAddress),
    Invalid(String),
    Default,
}

impl Display for CustomerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ValidEmail(address) => f.write_str(&address.to_string()),
            Self::Invalid(data) => f.write_str(data),
            Self::Default => f.write_str("Type ypour email"),
        }
    }
}

#[derive(Default, Clone)]
struct ProductsCache {
    pub current_products: Vec<Signal<productView>>,
}

fn App() -> Element {
    use_context_provider(|| Signal::new(LoadedCategories(None)));
    use_context_provider(|| Signal::new(CustomerId::Default));
    use_context_provider(|| Signal::new(ProductsCache::default()));

    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Products(selected_id: Signal<String>) -> Element {
    let mut list = use_context::<Signal<ProductsCache>>();
    let mut fetch: Resource<Result<(), String>> = use_resource(move || async move {
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
            list.write().current_products = result
                .data
                .and_then(|p| {
                    Some(
                        p.products_relay
                            .edges
                            .into_iter()
                            .map(|edge| use_signal(|| edge.node))
                            .collect(),
                    )
                })
                .unwrap();
            Ok(())
        } else {
            Err(result
                .errors
                .unwrap()
                .iter()
                .map(|e| e.message.clone())
                .collect::<Vec<_>>()
                .join(","))
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
                        for p in list.read().current_products.iter() {
                            ProductRow{
                                product_signal: p.clone(),
                                selected_id
                             }
                        }
            }
        }
    }
}

impl productView {
    pub async fn Buy(mut product: Signal<Self>, customer_id: &CustomerId) {
        let client = reqwest::Client::new();

        let CustomerId::ValidEmail(customer_id) = customer_id else {
            panic!("should not happen")
        };
        let variables = buy_product::Variables {
            product_id: product.read().id.clone(),
            customer_id: customer_id.clone(),
        };

        product.with_mut(|p| {
            p.actions_allowed = vec!["Processing".to_string()];
            p.in_stock -= 1;
        });

        let result =
            post_graphql::<BuyProduct, _>(&client, "http://localhost:7265/graphql", variables)
                .await
                .unwrap();

        let data = result.data.unwrap().buy.unwrap();

        product.with_mut(|p| {
            p.actions_allowed = data.actions_allowed;
            p.in_stock = data.in_stock;
        });
    }
}

#[component]
fn ProductRow(product_signal: Signal<productView>, selected_id: Signal<String>) -> Element {
    let customer_id = use_context::<Signal<CustomerId>>();
    let product = product_signal.read();
    let product_id = product.id.clone();
    rsx! {
        tr {
            key: "{product.id}",
            class: if product.id == *selected_id.read() { "table-active" } else {""},
                        onclick: move |_| {
                            *selected_id.write() = product_id.clone();
                            },
            td { "{product.name}"}
            td { "{product.description}"}
            td { "{product.in_stock}"}
            td {
                if let CustomerId::ValidEmail(_)=*customer_id.read()  {
                    if  product.canBuy() {
                        button {
                            r#type: "button",
                            class: "btn btn-primary",
                            onclick: move |event| {
                                event.stop_propagation();
                                spawn(async move {
                                    productView::Buy(product_signal,&customer_id.read()).await
                                });
                            } ,
                            "Buy"
                        }
                    } else {
                        if product.isProcessing() {
                            div {
                                class: "spinner-border spinner-border-sm",
                                role:"status",
                                span {
                                    class:"visually-hidden",
                                    "Processing.."
                                }
                            }
                        }
                    }
                }

            }
        }
    }
}

#[component]
fn Home() -> Element {
    let selected_id = use_signal(|| "".to_string());
    rsx! {
        Card {
            title: "Shop",
            Products {
                selected_id
            }
        }
       Card {
         title:"Product details",
         Product {
            selected_id
        }
       }
       Card {
        title:"Basket",
        Basket{}
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
        match &*future.read() {
            Some(Ok(response)) => rsx! {
                {
                    let list = use_context::<Signal<ProductsCache>>();

                response.clone().product.and_then(|product| {
                    rsx!{
                    controls::bootstrap::Input {
                        value: "{product.name}",
                        label: "Product name",
                        readonly: true
                    }
                    controls::bootstrap::Input {
                        value: "{product.description}",
                        label: "Product description",
                        readonly: true
                    }

                    if list.read().current_products.iter().find(|p| p.read().id == *selected_id.read() ).is_some_and( |p| p.read().isProcessing() ) {
                        div {
                            class: "alert alert-primary",
                            role: "alert",
                            "Processing Order"
                        }
                    }
                    Card {
                        title: "Product categories",
                        ul {
                            class:"list-group list-group-flush",
                            {
                              rsx!{
                                {
                                  loaded.read().0.clone().unwrap()
                                    .iter().map( |category |{
                                        rsx!{
                                            li {
                                                class: if product.hasCategory(&category) {    "list-group-item active"   } else{"list-group-item"   },
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
                class: format!("form-control {}" , if let CustomerId::Invalid(_) = *customer_id.read() { "is-invalid"} else { "is-valid"}),
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
            div {
                class: "invalid-feedback",
                "Invalid email {customer_id}"
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
