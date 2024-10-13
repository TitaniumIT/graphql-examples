#![allow(non_snake_case)]
use controls::bootstrap::Card;
use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
use graphql_client::{GraphQLQuery, Response};
use log::{info, LevelFilter};
use views::{
    basket::{Basket, CustomerId, ProductsInTransiteCache},
    manager::Manager,
    productdetails::{LoadedCategories, Product},
    productlist::{Products, ProductsCache},
};

mod controls;
mod models;
mod views;

static APIURL: &str = "localhost:8080/graphql";

fn http_endpoint() -> String {
    format!("http://{}", APIURL)
}

fn ws_endpoint() -> String {
    format!("ws://{}", APIURL)
}

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/manager")]
    Manager {},
}

fn main() {
    // Init debug
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}

fn App() -> Element {
    use_context_provider(|| Signal::new(LoadedCategories(None)));
    use_context_provider(|| Signal::new(CustomerId::Default));
    use_context_provider(|| Signal::new(ProductsCache::default()));
    use_context_provider(|| Signal::new(ProductsInTransiteCache::default()));

    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    info!("Home rendered");
    rsx! {
        Card {
            title: "Shop",
            Products {
            }
        }
       Card {
         title:"Product details",
         Product {
         }
       }
       Card {
        title:"Basket",
        Basket{

        }
      }
    }
}

// Taken from graphql-client feature reqwest featue because of dependency issues
pub async fn post_graphql<Q: GraphQLQuery, U: reqwest::IntoUrl>(
    client: &reqwest::Client,
    url: U,
    variables: Q::Variables,
) -> Result<Response<Q::ResponseData>, reqwest::Error> {
    let body = Q::build_query(variables);
    let reqwest_response = client.post(url).json(&body).send().await?;
    reqwest_response.json().await
}
