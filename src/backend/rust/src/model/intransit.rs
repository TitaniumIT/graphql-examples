use std::sync::Arc;

use juniper::graphql_object;

use crate::{ scalars::{DefaultScalarValue, EmailAddressScalar}, Context, StaticData};

use super::{product::Product, AvailableActionsInterfaceTypeValue, IProductValue};

#[derive(Clone,Debug)]
pub struct ProductInTransit {
    product_id: String,
    state: String,
    id: String,
    customer_id: EmailAddressScalar,
}

impl ProductInTransit {
    pub fn new(product: &Product, customer_id: EmailAddressScalar) -> Self {
        Self {
            product_id: product.id().clone(),
            customer_id: customer_id,
            state: "InTransit".to_string(),
            id: uuid::Uuid::new_v4().to_string(),
        }
    }

    pub async fn deliver(&mut self,data: Arc<StaticData>)  {
       self.state = "Deliverd".to_string();
       let (s,_r) = &data.status_channel;
       s.send(self.clone()).unwrap();
    }

    pub async fn cancel(&mut self,data: Arc<StaticData>)  {
       self.state = "Cancelled".to_string();
       let mut products = data.products.write().await;
       let product = products.iter_mut().find(|p| p.id() == &self.product_id).unwrap();
       product.restock();
       let (s,_r) = &data.status_channel;
       s.send(self.clone()).unwrap();
    }
}

#[graphql_object(context = Context,scalar=DefaultScalarValue)]
#[graphql(impl = IProductValue)]
#[graphql(impl = AvailableActionsInterfaceTypeValue)]
impl ProductInTransit {
    pub fn state(&self) -> &String {
        &self.state
    }

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

    pub fn customer_id(&self) -> &EmailAddressScalar {
        &self.customer_id
    }

    pub fn actions_allowed<'ctx>(&self, context: &'ctx Context) -> Vec<String> {
        if self.state == "InTransit" && context.ismanager {
            vec!["Deliver".to_string(), "Cancel".to_string()]
        } else {
            vec![]
        }
    }
}