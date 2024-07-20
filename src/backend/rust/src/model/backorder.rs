use crate::{scalars::DefaultScalarValue, Context};
use super::{product::Product, AvailableActionsInterfaceTypeValue, IProductValue};
use juniper::graphql_object;

#[derive(Clone)]
pub struct ProductInBackorder {
    product_id: String,
    id: String,
}

impl ProductInBackorder {
    pub fn new(product: &Product) -> Self {
        Self {
            product_id: product.id().clone(),
            id: uuid::Uuid::new_v4().to_string(),
        }
    }
}

#[graphql_object(context = Context,scalar=DefaultScalarValue)]
#[graphql(impl = IProductValue)]
#[graphql(impl = AvailableActionsInterfaceTypeValue)]
impl ProductInBackorder {
    pub fn product_id(&self) -> &String {
        &self.product_id
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub async fn name<'ctx>(&self, context: &'ctx Context) -> String {
        let products = context.data.products.read().await;
        let product = products.iter().find(|p| p.id() == &self.product_id).unwrap();
        product.name().clone()
    }

    pub async fn description<'ctx>(&self, context: &'ctx Context) -> String {
        let products = context.data.products.read().await;
        let product = products.iter().find(|p| p.id() == &self.product_id).unwrap();
        product.description().clone()
    }

    pub fn actions_allowed<'ctx>(&self, context: &'ctx Context) -> Vec<String> {
        if context.ismanager {
            vec!["Restock".to_string()]
        } else {
            vec![]
        }
    }
}
