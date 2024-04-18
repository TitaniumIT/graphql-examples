use juniper::graphql_object;

use crate::{Context, EmailAddressScalar, Product, ProductInTransit};

pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    pub async fn buy<'ctx>(
        context: &'ctx Context,
        product_id: String,
        customer_id: EmailAddressScalar,
        amount: Option<i32>,
    ) -> Option<Box<Product>> {
        let Context(context) = context;
        let mut context = context.write().await;

        if let Some(index) = context.products.iter().position(|p| p.id() == &product_id) {
            let mut products = context.products.clone();
            
            let product= products.get_mut(index).unwrap();
            
            product.change_stock(-amount.or_else(|| Some(1)).unwrap());

            context.products_in_transit.push( ProductInTransit::new(product, customer_id) );

            Some(product.clone())
        } else {
            Option::None
        }
    }
}
