#![allow(non_snake_case)]
use controls::bootstrap::Card;
use dioxus::prelude::*;
use log::{info, LevelFilter};
use views::{basket::{Basket, CustomerId}, productdetails::{LoadedCategories, Product}, productlist::{Products, ProductsCache}};

mod controls;
mod models;
mod views;

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

// #[derive(Default, Clone)]
// struct ProductsInTransitCache {
//     pub intransit: Vec<Signal<>>,
// }

fn App() -> Element {
    use_context_provider(|| Signal::new(LoadedCategories(None)));
    use_context_provider(|| Signal::new(CustomerId::Default));
    use_context_provider(|| Signal::new(ProductsCache::default()));

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

