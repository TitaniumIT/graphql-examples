use juniper::graphql_object;

use crate::{relaytypes::{Connection, Edge, PageInfo}, scalars::DefaultScalarValue, Context};

use super::product::Product;


pub type ProductConnection = Connection<Product, String>;

#[graphql_object(context = Context,scalar=DefaultScalarValue)]
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

    pub fn nodes(&self) -> Vec<&Product> {
        self._nodes()
    }

    pub fn page_info(&self) -> PageInfo<'_> {
        self._page_info()
    }
}

pub type ProductEdge = Edge<Product, String>;

#[graphql_object(context = Context,scalar=DefaultScalarValue)]
impl ProductEdge {
    pub fn node(&self) -> &Product {
        &self.node
    }
    pub fn cursor(&self) -> &String {
        &self.cursor
    }
}
