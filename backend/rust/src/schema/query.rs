use juniper::graphql_object;

use crate::model::categorie::Category;
use crate::model::productrelay::{ProductConnection, ProductEdge};
use crate::scalars::DefaultScalarValue;
use crate::Context;
use crate::{
    product::*,
    model::AllProductTypes,
};

pub struct Query;

#[graphql_object(context = Context,scalar=DefaultScalarValue)]
impl Query {
    pub async fn products<'ctx>(context: &'ctx Context) -> Vec<Product> {
        let products = context.data.products.read().await;
        products.to_vec()
    }

    pub async fn products_skip_take<'ctx>(
        context: &'ctx Context,
        #[graphql(default = 0)] skip: i32,
        #[graphql(default = 5)] take: i32,
    ) -> Vec<Product> {
        let products = context.data.products.read().await;

        let mut collection = products.to_vec();
        collection.sort();
        collection
            .into_iter()
            .skip(skip as usize)
            .take(take as usize)
            .collect()
    }

    pub async fn products_relay<'ctx>(
        context: &'ctx Context,
        first: Option<i32>,
        after: Option<String>,
        last: Option<i32>,
        before: Option<String>,
    ) -> ProductConnection {
        let products = context.data.products.read().await;

        let get_source = || {
            if let Some(first) = first.or_else(|| {
                if last.is_none() {
                    Some(5)
                } else {
                    Option::None
                }
            }) {
                products
                    .iter()
                    .filter(|p| p.id() > &after.clone().or_else(|| Some("".to_string())).unwrap())
                    .take(first as usize)
                    .cloned()
                    .collect()
            } else if let Some(last) = last {
                products
                    .iter()
                    .filter(|p| p.id() < &before.clone().unwrap())
                    .take(last as usize)
                    .cloned()
                    .collect()
            } else {
                Vec::<Product>::new()
            }
        };

        let edges: Vec<ProductEdge> = get_source()
            .iter()
            .map(|p| ProductEdge {
                node: p.clone(),
                cursor: p.id().clone(),
            })
            .collect();

        ProductConnection {
            has_next_page: &edges.last().map(|edge| &edge.cursor)
                != &products.iter().map(|p| p.id()).max(),
            has_previous_page: &edges.first().map(|edge| &edge.cursor)
                != &products.iter().map(|p| p.id()).min(),
            total_count: products.len() as i32,
            edges: edges,
        }
    }

    pub async fn product<'ctx>(context: &'ctx Context, product_id: String) -> Option<Product> {
        let products = context.data.products.read().await;

        products.iter().find(|p| p.id() == &product_id).cloned()
    }

    pub async fn all_products<'ctx>(context: &'ctx Context) -> Vec<AllProductTypes> {
        let products = context.data.products.read().await;

        let intransit = context.data.products_in_transit.read().await;
        let backorder = context.data.products_in_backorders.read().await;

        products
            .iter()
            .map(|p| AllProductTypes::Product(p.clone()))
            .chain(
                intransit
                    .iter()
                    .map(|p| AllProductTypes::ProductInTransit(p.clone()))
                    .chain(
                        backorder
                            .iter()
                            .map(|p| AllProductTypes::ProductInBackorder(p.clone())),
                    ),
            )
            .collect()
    }

    pub async fn categories<'ctx>(context: &'ctx Context) -> Option<Vec<Category>> {
       Some(context.data.categories().await)
    }
}
