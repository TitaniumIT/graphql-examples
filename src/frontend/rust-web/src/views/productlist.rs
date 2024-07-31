use dioxus::prelude::*;
use graphql_client::reqwest::post_graphql;
use log::info;

use crate::{controls::bootstrap::Table, models::{buy_product, get_products::{self, productView}, BuyProduct, GetProducts}, CustomerId};

#[derive(Default, Clone)]
pub struct ProductsCache {
    pub current_products: Vec<Signal<ProductRowState>>,
}

impl ProductsCache {
    pub fn SelectedProduct(&self) -> Option<Signal<ProductRowState>> {
        self.current_products
            .iter()
            .find(|p| p.read().selected)
            .map(|f| f.clone())
    }
}

#[derive(Default, Clone)]
pub struct ProductRowState {
    pub data: productView,
    pub selected: bool,
}


#[component]
pub fn Products(selected_id: Signal<String>) -> Element {
    let mut list = use_context::<Signal<ProductsCache>>();
    let _fetch: Resource<Result<(), String>> = use_resource(move || async move {
        info!("Products fetched");
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
                            .map(|edge| {
                                use_signal(|| ProductRowState {
                                    data: edge.node,
                                    selected: false,
                                })
                            })
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
         Table {
            columns: [ "Name", "Description" , "In Stock" , "Actions" ].map(String::from).to_vec(),
            body: rsx!{
                for p in list.read().current_products.iter() {
                  ProductRow{
                    product_signal: p.clone(),
                 }
             }
         }
       }
    }
}


#[component]
pub fn ProductRow(product_signal: Signal<ProductRowState>) -> Element {
    let customer_id = use_context::<Signal<CustomerId>>();
    let product = &product_signal.read().data;
    info!("Product row rendered");
    rsx! {
        tr {
            key: "{product.id}",
            class: if product_signal.read().selected { "table-active" } else {""},
            onclick: move |_| {
                            let list = use_context::<Signal<ProductsCache>>();
                            if let Some(mut old_product) = list.read().SelectedProduct() {
                                old_product.write().selected = false;
                            }
                            info!("Product row selected");
                            product_signal.write().selected = true;
                        },
            td { "{product.name}"}
            td { "{product.description}"}
            td { "{product.in_stock}"}
            { RowActions(product_signal,product,customer_id) }      
        }
    }
}

fn RowActions(product_signal:Signal::<ProductRowState>,product:&productView,customer_id:Signal::<CustomerId>) -> Element {
    rsx!(
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
    )
}


impl productView {
    pub async fn Buy(mut product: Signal<ProductRowState>, customer_id: &CustomerId) {
        info!("Product buy");
        let client = reqwest::Client::new();

        let CustomerId::ValidEmail(customer_id) = customer_id else {
            panic!("should not happen")
        };
        let variables = buy_product::Variables {
            product_id: product.read().data.id.clone(),
            customer_id: customer_id.clone(),
        };

        product.with_mut(|p| {
            p.data.actions_allowed = vec!["Processing".to_string()];
            p.data.in_stock -= 1;
        });

        let result =
            post_graphql::<BuyProduct, _>(&client, "http://localhost:7265/graphql", variables)
                .await
                .unwrap();

        let data = result.data.unwrap().buy.unwrap();

        product.with_mut(|p| {
            p.data.actions_allowed = data.actions_allowed;
            p.data.in_stock = data.in_stock;
        });
    }
}

