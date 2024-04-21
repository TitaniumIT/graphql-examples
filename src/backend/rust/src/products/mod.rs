use juniper::{graphql_interface, GraphQLUnion};
use crate::Context;

use self::{backorder::ProductInBackorder, intransit::ProductInTransit, product::Product};

pub mod product;
pub mod intransit;
pub mod backorder;

#[derive(GraphQLUnion)]
#[graphql(Context = Context)]
pub enum AllProductTypes {
    Product(Product),
    ProductInBackorder(ProductInBackorder),
    ProductInTransit(ProductInTransit),
}

#[graphql_interface(for = [Product, ProductInTransit , ProductInBackorder])]
#[graphql(Context = Context)]
pub trait IProduct {
    fn id(&self) -> &String;
    fn name(&self) -> &String;
    fn description(&self) -> &String;
}
