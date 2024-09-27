use std::ops::IndexMut;

use dioxus::prelude::*;
use graphql_client::reqwest::post_graphql;
use log::info;

use crate::{
    controls::bootstrap::Table,
    models::{
        buy_product,
        get_basket_products::{BasketView, BasketViewInTransit},
        get_products::{self, pageInfoView, productView},
        BuyProduct, GetProducts,
    },
    views::basket::ProductsInTransiteCache,
    CustomerId, APIURL,
};

#[derive(Default, Clone)]
pub struct ProductsCache {
    pub current_products: Vec<Signal<ProductRowState>>,
    pub page_info: pageInfoView,
}

impl ProductsCache {
    pub fn SelectedProduct(&self) -> Option<Signal<ProductRowState>> {
        self.current_products
            .iter()
            .find(|p| p.read().selected)
            .copied()
    }
}

#[derive(Default, Clone)]
pub struct ProductRowState {
    pub data: productView,
    pub selected: bool,
}

#[component]
pub fn Products() -> Element {
    let mut list = use_context::<Signal<ProductsCache>>();
    let mut input_variables = use_signal(|| get_products::Variables {
        first: Some(5),
        after: None,
        before: None,
        last: None,
    });
    let _fetch: Resource<Result<(), String>> = use_resource(move || async move {
        info!("Products fetched");
        let client = reqwest::Client::new();

        let variables = input_variables.read().clone();

        let result = post_graphql::<GetProducts, _>(&client, APIURL, variables)
            .await
            .unwrap();

        if result.errors.is_none() {
            list.with_mut(|l| {
                let data = result.data;
                data.map(|r| {
                    l.current_products = r
                        .products_relay
                        .edges
                        .into_iter()
                        .map(|edge| {
                            Signal::new(ProductRowState {
                                data: edge.node,
                                selected: false,
                            })
                        })
                        .collect();
                    l.page_info = r.products_relay.page_info;
                });
            });
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
            for (i,p) in list.read().current_products.iter().enumerate() {
                  ProductRow{
                    key: "{i}",
                    product_signal: *p
                 }
         }
       }
       ul {
          class: "pagination",
          li {
             class: if !list.read().page_info.has_previous_page { "page-item disabled"} else { "page-item" },
             a {
                class: "page-link",
                role: "button",
                aria_disabled: !list.read().page_info.has_previous_page,
                onclick : move |_| {
                    input_variables.with_mut( |i| {
                        let pageinfo = list.read().page_info.clone();
                        i.first = None;
                        i.after = None;
                        i.last= Some(5);
                        i.before= pageinfo.start_cursor;
                 } )},
                span {
                   aria_hidden: true,
                   dangerous_inner_html: "&laquo;"
                 }
             }
          }
      li {
             class: if !list.read().page_info.has_next_page { "page-item disabled"} else { "page-item" },
             a {
                class: "page-link",
                role: "button",
                aria_disabled: !list.read().page_info.has_next_page,
                onclick : move |_| {
                    input_variables.with_mut( |i| {
                        let pageinfo = list.read().page_info.clone();
                        i.first = Some(5);
                        i.after = pageinfo.end_cursor;
                        i.last=Option::None;
                        i.before=Option::None;
                 } )},
                 span {
                   aria_hidden: true,
                   dangerous_inner_html: "&raquo;"
                 }
             }
          } }
    }
}

#[component]
pub fn ProductRow(product_signal: Signal<ProductRowState>) -> Element {
    info!("Product row rendered");
    let customer_id = use_context::<Signal<CustomerId>>();
    let product = &product_signal.read().data;
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

fn RowActions(
    product_signal: Signal<ProductRowState>,
    product: &productView,
    customer_id: Signal<CustomerId>,
) -> Element {
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

        let result = post_graphql::<BuyProduct, _>(&client, APIURL, variables)
            .await
            .unwrap();

        let data = result.data.unwrap().buy.unwrap();

        product.with_mut(|p| {
            p.data.actions_allowed = data.actions_allowed;
            p.data.in_stock = data.in_stock;
        });

        let mut list = use_context::<Signal<ProductsInTransiteCache>>();
        list.with_mut(|c| {
            if let Some(index) = c.basket.iter().position(|p| p.read().id == data.id) {
                c.basket[index].with_mut( |e|
                    e.in_transit = data
                        .in_transit
                        .iter()
                        .map(|t| BasketViewInTransit {
                            id: t.id.clone(),
                            product_id: t.product_id.clone(),
                            state: t.state.clone(),
                        })
                        .collect());
            } else {
                c.basket.push(Signal::new(BasketView {
                    id: data.id,
                    name: data.name,
                    in_transit: data
                        .in_transit
                        .iter()
                        .map(|t| BasketViewInTransit {
                            id: t.id.clone(),
                            product_id: t.product_id.clone(),
                            state: t.state.clone(),
                        })
                        .collect(),
                }));
            }
        });
    }
}
