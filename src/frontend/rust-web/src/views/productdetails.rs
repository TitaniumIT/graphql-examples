use dioxus::prelude::*;
use graphql_client::reqwest::post_graphql;
use log::info;

use crate::{controls::{ bootstrap::{Card, Input}}, models::{get_product, GetProduct}};

use super::productlist::ProductsCache;


#[derive(Clone)]
pub struct LoadedCategories(pub Option<Vec<get_product::categoryView>>);


#[component]
pub fn Product() -> Element {
    let mut loaded = use_context::<Signal<LoadedCategories>>();
    let list = use_context::<Signal<ProductsCache>>();

    let details = if let Some(selected_product) = list.read().SelectedProduct() {
        let selected_id = selected_product.read().data.id.clone();

        let future: Resource<Result<get_product::ResponseData, String>> =
            use_resource(use_reactive!(|(selected_id,)| async move {
                let client = reqwest::Client::new();
                info!("Product detail fetched");

                let variables = get_product::Variables {
                    product_id: selected_id.to_string(),
                    load_categories: loaded.read().0.is_none(),
                };

                let result = post_graphql::<GetProduct, _>(
                    &client,
                    "http://localhost:7265/graphql",
                    variables,
                )
                .await;

                if let Ok(result) = result {
                    if result.errors.is_none() {
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
            }));

        info!("Product detail Rendered");

        let rval = match &*future.read() {
            Some(Ok(response)) => rsx! {
                {
                    let list = use_context::<Signal<ProductsCache>>();

                response.clone().product.map(|product| 
                    rsx!{
                     Input {
                        value: "{product.name}",
                        label: "Product name",
                        readonly: true
                    }
                     Input {
                        value: "{product.description}",
                        label: "Product description",
                        readonly: true
                    }

                    if list.read().current_products.iter().find(|p| p.read().selected ).is_some_and( |p| p.read().data.isProcessing() ) {
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
                                                class: if product.hasCategory(category) {    "list-group-item active"   } else{"list-group-item"   },
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
                ).or_else(|| rsx! {
                    tr {
                        td { colspan:"4","Loading"}
                    }
                }.into() )
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
        };
        return rval;
    } else {
        rsx! {
            div {
                "No product selected"
            }
        }
    };
    return details;
}