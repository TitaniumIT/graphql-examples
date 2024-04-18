use std::env;
use std::sync::Arc;
use tokio::{self, sync::RwLock};
use warp::filters::cors::Cors;
use warp::Filter;

mod categorie;
mod product;
mod relaytypes;
mod schema;
mod mutation;

use crate::categorie::*;
use crate::product::*;
use crate::schema::*;

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

    let data = Arc::new(RwLock::new(StaticData {
        products: static_product_list,
        categories: static_categorie_list,
        products_in_transit: Vec::new()
    }));

    let cors = warp::cors()
        .allow_origins( vec!["http://localhost:4200","http://localhost:7265"] )
        .allow_methods( vec!["POST","OPTIONS"])
        .allow_headers(vec!["User-Agent", "Sec-Fetch-Mode", "Referer", "Origin", "Access-Control-Request-Method", 
            "Access-Control-Request-Headers", "Content-Type" ])
        .allow_credentials(true).build();

    let routes = (
        warp::post()
        .and(warp::path("graphql"))
        .and(juniper_warp::make_graphql_filter(
            schema.clone(),
            warp::any().map(move || Context(data.clone())),
        )).with(cors.clone()).with(log))
    .or(warp::get()
        .and(warp::path("playground"))
        .and(juniper_warp::playground_filter("/graphql", None)).with(log))
    .or(warp::any().map(warp::reply).with(cors.clone()).with(log));

    warp::serve(routes).run(([127, 0, 0, 1], 7265)).await
}
