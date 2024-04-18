use juniper::{graphql_object, GraphQLScalar};
use std::{cmp::Ordering, sync::Arc};

use crate::{
    relaytypes::{Connection, Edge, PageInfo}, Category, Context, EmailAddressScalar
};


#[derive(Clone)]
pub struct Product {
    id: String,
    name: String,
    description: String,
    categories: Vec<String>,
    stock: i32,
}

impl Eq for Product {}

impl Ord for Product {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for Product {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub type ProductConnection = Connection<Box<Product>, String>;

#[graphql_object(context = Context)]
impl ProductConnection {
    pub fn edges(&self) -> &[ProductEdge] {
        self._edges()
    }

    pub fn items(&self) -> Vec<&Product> {
        self._edges().iter().map(|p| p.node()).collect()
    }

    pub fn total_count(&self) -> i32 {
        self.total_count
    }

    pub fn nodes(&self) -> Vec<&Box<Product>> {
        self._nodes()
    }

    pub fn page_info(&self) -> PageInfo<'_> {
        self._page_info()
    }
}

pub type ProductEdge = Edge<Box<Product>, String>;

#[graphql_object(context = Context)]
impl ProductEdge {
    pub fn node(&self) -> &Product {
        &self.node
    }
    pub fn cursor(&self) -> &String {
        &self.cursor
    }
}

impl PartialOrd for Product {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.id < other.id {
            Some(Ordering::Less)
        } else {
            if self.id == other.id {
                Some(Ordering::Equal)
            } else {
                Some(Ordering::Greater)
            }
        }
    }
}

impl Product {
    pub fn new(id: &str, n: &str, d: &str, c: Vec<i32>, s: i32) -> Box<Self> {
       Box::new( Self {
            id: id.to_string(),
            name: n.to_string(),
            description: d.to_string(),
            categories: c.iter().map(|i| i.to_string()).collect(),
            stock: s,
        })
    }

    pub fn change_stock(&mut self, amount: i32) {
        self.stock += amount
    }
}

#[graphql_object(context = Context)]
impl Product {
    pub fn id(&self) -> &String {
        &self.id
    }
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn in_stock(&self) -> &i32 {
        &self.stock
    }

    pub fn actions_allowed(&self) -> Vec<String> {
        vec!["Buy".to_string()]
    }

    pub async fn categories_without_batch_async<'ctx>(&self,
        context: &'ctx Context) -> Vec<Category> {
            let Context(context) = context;
            let context = context.read().await;

            context.categories().await.iter().filter(|c| self.categories.contains( c.id() ) ).cloned().collect()
    }

    pub async fn products_in_transit<'ctx>(&self,
          context: &'ctx Context,
         customer_id: EmailAddressScalar) -> Vec<ProductInTransit>  {
            
            let Context(context) = context;
            let context = context.read().await;

            context.products_in_transit.iter()
                .filter(|p| p.customer_id == customer_id && p.product_id() == self.id()).cloned().collect()
        }
}


#[derive(Clone)]
pub struct ProductInTransit {
    product: Box<Product>,
    state: String,
    id: String,
    customer_id: EmailAddressScalar
}

impl ProductInTransit {
    pub fn new(product: &Box<Product>,customer_id:EmailAddressScalar) -> Self {
        Self {
            product: product.clone(),
            customer_id: customer_id,
            state: "InTransit".to_string(),
            id: "1".to_string()
        }
    }
}

#[graphql_object(context = Context)]
impl ProductInTransit {

    pub fn state(&self) -> &String {
        &self.state
    }

    pub fn product_id(&self) -> &String{
        &self.product.id
    }

    pub fn id(&self) -> &String{
        &self.id
    }

    pub fn name(&self) -> &String{
        &self.product.name
    }

    pub fn description(&self) -> &String{
        &self.product.description
    }

    pub fn customer_id(&self) -> &EmailAddressScalar{
        &self.customer_id
    }

}