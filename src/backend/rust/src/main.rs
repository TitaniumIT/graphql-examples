use std::env;
use std::sync::Arc;
use async_channel::unbounded;
use juniper_graphql_ws::ConnectionConfig;
use tokio::{self, sync::RwLock};
use warp::Filter;

mod categorie;
mod products;
mod relaytypes;
mod schema;
mod staticdata;

use crate::categorie::*;
use crate::products::product::{self, Product};
use crate::schema::*;
use crate::staticdata::StaticData;

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "warp,graphql-rust-backend");
    env_logger::init();
    println!("Hello, world!");

    let log = warp::log("graphql-rust-backend");
    let schema = Arc::new(schema());

    let static_product_list = vec![
        Product::new("1", "titanium", "Strong metal", vec![1, 3], 10),
        Product::new("2", "oak", "strong wood", vec![2, 3], 5),
        Product::new("3", "iron", "a metal", vec![1, 3], 5),
        Product::new("4", "silver", "a precious metal", vec![1, 3], 5),
        Product::new("5", "gold", "a rare precious metal", vec![1, 3], 5),
        Product::new("6", "porc", "meat based on a Pig", vec![4], 5),
        Product::new("7", "beef", "meat base on a cow", vec![4], 5),
        Product::new("8", "bread", "made from wheat", vec![4], 5),
    ];

    let static_categorie_list = vec![
        Category::new("1", "metals"),
        Category::new("2", "wood"),
        Category::new("3", "non-food"),
        Category::new("4", "food"),
    ];

    let data = Arc::new(StaticData {
        products: Arc::new(RwLock::new(static_product_list)),
        categories: Arc::new(RwLock::new(static_categorie_list)),
        products_in_transit: Arc::new(RwLock::new(Vec::new())),
        products_in_backorders: Arc::new(RwLock::new(Vec::new())),
        status_channel: unbounded()
    });

    let cors = warp::cors()
        .allow_origins(vec!["http://localhost:4200", "http://localhost:7265"])
        .allow_methods(vec!["POST", "OPTIONS"])
        .allow_headers(vec![
            "User-Agent",
            "Sec-Fetch-Mode",
            "Referer",
            "Origin",
            "Access-Control-Request-Method",
            "managersecret",
            "Access-Control-Request-Headers",
            "Content-Type",
        ])
        .allow_credentials(true)
        .build();

    let http_data = data.clone();
    let ws_data = data.clone();

    let routes = (warp::post()
        .and(warp::path("graphql"))
        .and(juniper_warp::make_graphql_filter(
            schema.clone(),
            warp::any()
                .and(warp::header::optional("managersecret"))
                .map(move |managersecret: Option<String>| Context {
                    data: http_data.clone(),
                    ismanager: managersecret == Some("I`m Manager".to_string()),
                }),
        ))
        .with(cors.clone())
        .with(log))
        .or(
            warp::path("graphql").and(juniper_warp::subscriptions::make_ws_filter(
                schema.clone(),
                ConnectionConfig::new( Context {
                    data: ws_data,
                    ismanager: false,
                }),
            ))
            .with(log),
        )
    .or(warp::get()
        .and(warp::path("playground"))
        .and(juniper_warp::playground_filter("/graphql", Some("/graphql") ))
        .with(log))
    .or(warp::any().map(warp::reply).with(cors.clone()).with(log));

    warp::serve(routes).run(([127, 0, 0, 1], 7265)).await
}
