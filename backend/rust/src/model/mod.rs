use juniper::{graphql_interface, GraphQLUnion};

use crate::Context;

use self::{backorder::ProductInBackorder, intransit::ProductInTransit, product::Product};
use crate::scalars::DefaultScalarValue;

pub mod product;
pub mod intransit;
pub mod backorder;
pub mod categorie;
pub mod productrelay;

#[derive(GraphQLUnion)]
#[graphql(context = Context,scalar=DefaultScalarValue)]
pub enum AllProductTypes {
    Product(Product),
    ProductInBackorder(ProductInBackorder),
    ProductInTransit(ProductInTransit),
}

#[graphql_interface(for = [Product, ProductInTransit , ProductInBackorder])]
#[graphql(context = Context,scalar=DefaultScalarValue)]
pub trait IProduct {
    fn id(&self) -> &String;
    fn name(&self) -> &String;
    fn description(&self) -> &String;
}

#[graphql_interface(for = [Product, ProductInTransit , ProductInBackorder])]
#[graphql(context = Context,scalar=DefaultScalarValue)]
pub trait AvailableActionsInterfaceType {
    fn actions_allowed() -> Vec<String>;
}
