use std::{fmt::Display, future::IntoFuture};
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use futures_util::StreamExt;
use graphql_client::GraphQLQuery;
use shared_types::EmailAddress;

use crate::{
    controls::bootstrap::Table,
    models::{basket_events, get_basket_products::BasketView, BasketEvents},
    ws_endpoint,
};

#[derive(Clone, Debug, PartialEq)]
pub enum CustomerId {
    ValidEmail(EmailAddress),
    Invalid(String),
    Default,
}

impl CustomerId {
    fn is_not_default(&self) -> bool {
        if let Self::Default = self {
            false
        } else {
            true
        }
    }
    fn is_default(&self) -> bool {
        if let Self::Default = self {
            true
        } else {
            false
        }
    }
}

impl Display for CustomerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ValidEmail(address) => f.write_str(&address.to_string()),
            Self::Invalid(data) => f.write_str(data),
            Self::Default => f.write_str("Type your email"),
        }
    }
}

#[derive(Default, Clone)]
pub struct ProductsInTransiteCache {
    pub basket: Vec<Signal<BasketView>>,
}

fn set_customer(event: Event<FormData>, mut customer_id: Signal<CustomerId>,mut events:Coroutine::<()> ) {
    let result = EmailAddress::new(&event.value());
    if let Ok(email) = result {
        customer_id.set(CustomerId::ValidEmail(email));
    } else {
        if event.value().is_empty() {
            customer_id.set(CustomerId::Default)
        } else {
            customer_id.set(CustomerId::Invalid(event.value()));
        }
    }
    events.restart();
}

use graphql_ws_client::{graphql::StreamingOperation, ws_stream_wasm::Connection, Client};

pub fn Basket() -> Element {
    let mut customer_id = use_context::<Signal<CustomerId>>();
    let mut list = use_context::<Signal<ProductsInTransiteCache>>();
    let basket_events: Coroutine<()> = use_coroutine(|rx| async move {

        if let CustomerId::ValidEmail(email) = customer_id.read().clone() {
        
        let ws_conn = ws_stream_wasm::WsMeta::connect( ws_endpoint(), Some(vec!["graphql-transport-ws"]),)
            .await
            .expect("assume the connection succeeds");

           let variables = basket_events::Variables { customer_id: email };
           let operation= StreamingOperation::<BasketEvents>::new(variables);

           let connection = Connection::new(ws_conn).await;

           let (client, actor) = Client::build(connection).await.unwrap();
           spawn(actor.into_future());
       //    wasm_bindgen_futures::spawn_local(actor.into_future());

           let mut stream = client.subscribe(operation).await.unwrap();

            while let Some(message) = stream.next().await {
                let event = message.unwrap().data.unwrap().status_changed;

                list.with_mut(|basket| {
                    if let Some(index) = basket
                        .basket
                        .iter()
                        .position(|p| p.read().id == event.product_id)
                    {
                        info!("Got event {:?}",event);
                        basket.basket[index].with_mut(|item| {
                            let state = item
                                .in_transit
                                .iter_mut()
                                .find(|p| p.id == event.id)
                                .unwrap();
                            state.state = event.state;
                            info!("Updated state {:?}",state);
                        })
                    }
                });

            }
        }
    });

    rsx! {
        div{
            input {
                class: format!("form-control {}" , if let CustomerId::Invalid(_) = *customer_id.read() { "is-invalid"} else { "is-valid"}),
                id:"customerid",
                value: if customer_id.read().is_not_default() { "{customer_id}" },
                required:true,
                placeholder: if customer_id.read().is_default() { "{customer_id}" } ,
                r#type:"email",
                oninput: move |event| set_customer(event, customer_id,basket_events)
                }
            },
            div {
                class: "invalid-feedback",
                "Invalid email {customer_id}"
            }
        Table {
            caption: "Basket for {customer_id}",
            columns: [ "Name" ,"ordered" ,"intansit" ,"deliverd" ,"cancelled" ].map(String::from).to_vec(),
            for row in list.read().basket.iter() {
                BasketRowView {
                    basket_row: *row
                }
            }
        }
    }
}

#[component]
fn BasketRowView(basket_row: Signal<BasketView>) -> Element {
    let row = basket_row.read();
    rsx! {
       tr {
         td { "{row.name}" }
         td { "{row.nrOrderd()}" }
         td { "{row.nrInTransit()}" }
         td { "{row.nrDeliverd()}" }
         td { "{row.nrCancelled()}" }
         }
    }
}
