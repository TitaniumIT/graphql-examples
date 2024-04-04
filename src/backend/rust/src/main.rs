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

    let static_product_list = Arc::new( vec![
        Product::new(1 ,"titanium","Strong metal" ,  vec![1,3],10),   
        Product::new (2 , "oak" , "strong wood" , vec![2, 3 ], 5),
        Product::new (3 , "iron" , "a metal" , vec![1, 3], 5),
        Product::new (4 , "silver" , "a precious metal" , vec![1, 3 ], 5),
        Product::new (5 , "gold" , "a rare precious metal" , vec![1, 3 ], 5),
        Product::new (6 , "porc" , "meat based on a Pig" , vec![4 ], 5),
        Product::new (7 , "beef" , "meat base on a cow" , vec![4 ], 5),
        Product::new (8 , "bread" , "made from wheat" , vec![4 ], 5) ]);

    let routes = (warp::post()
        .and(warp::path("graphql"))
        .and(juniper_warp::make_graphql_filter(
            schema.clone(),
            warp::any().map(move|| Context{ products: static_product_list.clone()} ),
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
    products : Arc<Vec<Product>>
}


impl juniper::Context for Context {}
type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

fn schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}

struct Query;

#[graphql_object(context = Context)]
impl Query {
    async fn products<'ctx>(context: &'ctx Context) -> Arc<Vec<Product>> {
        context.products.clone()
    }
}


struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    async fn buy<'ctx>(context: &'ctx Context,id: ProductId,customerId:String,amount:Option<i32>) -> &Product {
        let mut product = context.products.iter().find(|p| p.id == id).unwrap();
      //  product.stock = product.stock - amount.or_else(||Some(1)).unwrap();
        product
    }
}

#[derive(GraphQLScalar,PartialEq)]
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
    fn new(id:i32,n:&str,d:&str,c:Vec<i32>,s:i32) -> Self {
       Self{ id:ProductId(id),name:n.to_string(),description:d.to_string(),categories:c,stock:s}
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
  fn stock(&self) -> &i32{
    &self.stock
  }
}

