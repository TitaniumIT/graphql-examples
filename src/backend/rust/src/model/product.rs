use juniper::graphql_object;
use std::{cmp::Ordering, sync::Arc, time::Duration};
use tokio::time::sleep;

use crate::{
    scalars::{DefaultScalarValue, EmailAddressScalar}, Context, StaticData
};

use super::{
    backorder::ProductInBackorder, categorie::Category, intransit::ProductInTransit,
    AvailableActionsInterfaceTypeValue, IProductValue,
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
    pub fn new(id: &str, n: &str, d: &str, c: Vec<i32>, s: i32) -> Self {
        Self {
            id: id.to_string(),
            name: n.to_string(),
            description: d.to_string(),
            categories: c.iter().map(|i| i.to_string()).collect(),
            stock: s,
        }
    }

    pub fn restock(&mut self) {
        self.stock += 1;
    }

    pub fn is_allowed_to_buy(&self) -> bool {
        self.stock > 0
    }

    pub async fn buy(
        &mut self,
        amount: i32,
        customer_id: EmailAddressScalar,
        data: Arc<StaticData>,
    ) -> bool {
        if self.stock - amount < 0 {
            return false;
        }
        self.stock -= amount;

        sleep(Duration::from_millis(5000)).await;

        if self.stock == 0 {
            let mut backorder = data.products_in_backorders.write().await;
            backorder.push(ProductInBackorder::new(self));
        }

        let mut in_transit = data.products_in_transit.write().await;
        in_transit.push(ProductInTransit::new(self, customer_id));

        return true;
    }
}

#[graphql_object(context = Context,scalar=DefaultScalarValue)]
#[graphql(impl = IProductValue)]
#[graphql(impl = AvailableActionsInterfaceTypeValue)]
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

    pub fn actions_allowed<'ctx>(&self, context: &'ctx Context) -> Vec<String> {
        if self.is_allowed_to_buy() && !context.ismanager {
            vec!["Buy".to_string()]
        } else {
            vec![]
        }
    }

    pub async fn categories_without_batch_async<'ctx>(
        &self,
        context: &'ctx Context,
    ) -> Vec<Category> {
        context
            .data
            .categories()
            .await
            .iter()
            .filter(|c| self.categories.contains(c.id()))
            .cloned()
            .collect()
    }

    pub async fn products_in_transit<'ctx>(
        &self,
        context: &'ctx Context,
        customer_id: EmailAddressScalar,
    ) -> Vec<ProductInTransit> {
        let in_transit = context.data.products_in_transit.read().await;

        in_transit
            .iter()
            .filter(|p| p.customer_id() == &customer_id && p.product_id() == self.id())
            .cloned()
            .collect()
    }
}
