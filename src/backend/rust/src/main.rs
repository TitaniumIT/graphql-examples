use juniper::{
    graphql_object, EmptyMutation, EmptySubscription, GraphQLScalar, RootNode
};
use warp:: Filter;
use std::sync::Arc;
use tokio;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let schema = Arc::new(schema());

    let routes = (warp::post()
        .and(warp::path("graphql"))
        .and(juniper_warp::make_graphql_filter(
            schema.clone(),
            warp::any().map(|| Context),
        )))
        .or(warp::get()
        .and(warp::path("playground"))
        .and(juniper_warp::playground_filter(
            "/graphql",
            None,
        )));

     warp::serve(routes).run(([127, 0, 0, 1], 8080)).await
}

struct Context {
    products: Vec<Product>
}

impl Context {
    fn new() -> Self {
        Self { 
            products:  vec![
                Product { id:ProductId(1) , name:"titanium".to_string() , description:"Strong metal" , categories: vec![1,3],stock:10},
                Product {id:ProductId(2), name:"oak".to_string() , description:"strong wood" , categories: vec![2, 3 ], stock: 5},
                Product {id:ProductId(3) , name:"iron" , description:"a metal" , categories: vec![1, 3 ], stock:5},
                Product {id:ProductId(4) , name:"silver" , description:"a precious metal" ,categories: vec![1, 3 ], stock:5},
                Product {id:ProductId(5) , name:"gold" , description:"a rare precious metal" , categories: vec![1,3 ], stock:5},
                Product {id:ProductId(6) , name:"porc" ,description: "meat based on a Pig" , categories: vec![4 ], stock:5},
                Product {id:ProductId(7) , name:"beef" , description:"meat base on a cow" , categories: vec![4 ], stock:5},
                Product {id:ProductId(8) , name:"bread" ,description: "made from wheat" , categories: vec![4 ], stock:5}
            ]
        }
    }
}

impl juniper::Context for Context {}
type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

fn schema() -> Schema {
    Schema::new(Query, EmptyMutation::new(), EmptySubscription::new())
}

struct Query;

#[graphql_object(context = Context)]
impl Query {
    async fn products<'ctx>(context: &'ctx Context) -> &Vec<Product> {
        &context.products
    }
}

#[derive(GraphQLScalar)]
#[graphql(transparent)]
struct ProductId(i32);

struct Product{
    id: ProductId,
    name: String,
    description:String,
    categories: Vec<i32>,
    stock: i32
}

impl Product{
}

#[graphql_object]
 impl Product {

  fn id(&self) -> &ProductId {
    &self.id
  }
  fn name(&self) -> &String {
    &self.name
  }
}

