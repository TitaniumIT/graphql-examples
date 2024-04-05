use juniper::{graphql_object, EmptyMutation, EmptySubscription, GraphQLScalar, RootNode};
use std::{borrow::Borrow, future::IntoFuture, sync::Arc};
use tokio::{self, sync::{Mutex, RwLock, RwLockReadGuard}};
use warp::Filter;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let schema = Arc::new(schema());

    let static_product_list = vec![
        Product::new(1, "titanium", "Strong metal", vec![1, 3], 10),
        Product::new(2, "oak", "strong wood", vec![2, 3], 5),
        Product::new(3, "iron", "a metal", vec![1, 3], 5),
        Product::new(4, "silver", "a precious metal", vec![1, 3], 5),
        Product::new(5, "gold", "a rare precious metal", vec![1, 3], 5),
        Product::new(6, "porc", "meat based on a Pig", vec![4], 5),
        Product::new(7, "beef", "meat base on a cow", vec![4], 5),
        Product::new(8, "bread", "made from wheat", vec![4], 5),
    ];

    let data = Arc::new(RwLock::new(StaticData{ products: static_product_list}));

    let routes = (warp::post()
        .and(warp::path("graphql"))
        .and(juniper_warp::make_graphql_filter(
            schema.clone(),
            warp::any().map(move|| Context(data.clone())),
        )))
    .or(warp::get()
        .and(warp::path("playground"))
        .and(juniper_warp::playground_filter("/graphql", None)));

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await
}


struct StaticData {
    products: Vec<Product>
}

struct Context(Arc<RwLock<StaticData>>);

impl juniper::Context for Context {}
type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

fn schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}

struct Query;

#[graphql_object(context = Context)]
impl Query {
    async fn products<'ctx>(context: &'ctx Context) -> Vec<Product>  {
        let Context(context) = context;
        let context = context.read().await;
        context.products.clone()
    }
}

struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    async fn buy<'ctx>(
        context: &'ctx Context,
        id: ProductId,
        customer_id: String,
        amount: Option<i32>,
    ) -> Option<Product> {
        let Context(context) = context;
        let mut context = context.write().await;

        if let Some(index) = context.products.iter().position(|p| p.id == id) {
            let product = context.products.get_mut(index).unwrap();
            product.change_stock( - amount.or_else(||Some(1)).unwrap() );
            Some( product.clone())
        } else {
            Option::None
        }

    }
}

#[derive(GraphQLScalar, PartialEq,Clone)]
#[graphql(transparent)]
struct ProductId(i32);

#[derive(Clone)]
struct Product {
    id: ProductId,
    name: String,
    description: String,
    categories: Vec<i32>,
    stock: i32,
}

impl Product {
    fn new(id: i32, n: &str, d: &str, c: Vec<i32>, s: i32) -> Self {
         Self {
            id: ProductId(id),
            name: n.to_string(),
            description: d.to_string(),
            categories: c,
            stock: s,
        }
    }

    fn change_stock(& mut self,amount:i32){
        self.stock += amount
    }
}

#[graphql_object]
impl Product {
    fn id(&self) -> &ProductId {
        &self.id
    }
    fn name(&self) -> &String {
        &self.name
    }
    fn stock(&self) -> &i32 {
        &self.stock
    }
}
