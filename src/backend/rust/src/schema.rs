use juniper::{graphql_object, EmptySubscription, GraphQLScalar, RootNode};
use std::{sync::Arc, time::Duration};
use tokio::{self, sync::RwLock, time::sleep};

use crate::{ mutation::Mutation, product::*, Category};

pub struct StaticData {
    pub products: Vec<Box<Product>>,
    pub categories: Vec<Category>,
    pub products_in_transit: Vec<ProductInTransit>
}

impl StaticData {
    pub async fn categories(&self) -> &Vec<Category> {
        sleep(Duration::from_millis(1000)).await;
        &self.categories
    }
}

pub struct Context(pub Arc<RwLock<StaticData>>);
impl juniper::Context for Context {}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    pub async fn products<'ctx>(context: &'ctx Context) -> Vec<Box<Product>> {
        let Context(context) = context;
        let context = context.read().await;
        context.products.clone()
    }

    pub async fn products_skip_take<'ctx>(
        context: &'ctx Context,
        #[graphql(default = 0)] skip: i32,
        #[graphql(default = 5)] take: i32,
    ) -> Vec<Box<Product>> {
        let Context(context) = context;
        let context = context.read().await;

        let mut collection = context.products.clone();
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
        let Context(context) = context;
        let context = context.read().await;

        let get_source = || {
            if let Some(first) = first.or_else(|| if last.is_none() { Some(5)} else { Option::None}) {
                context
                    .products
                    .iter()
                    .filter(|p| p.id() > &after.clone().or_else(||Some("".to_string())).unwrap())
                    .take(first as usize)
                    .cloned()
                    .collect()
            } else if let Some(last) = last {
                context
                    .products
                    .iter()
                    .filter(|p| p.id() < &before.clone().unwrap())
                    .take(last as usize)
                    .cloned()
                    .collect()
            } else {
                Vec::<Box<Product>>::new()
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
            has_next_page: &edges.last().map(|edge| &edge.cursor ) != &context.products.iter().map(|p|p.id()).max(),
            has_previous_page: &edges.first().map(|edge| &edge.cursor) != &context.products.iter().map(|p|p.id()).min(),
            total_count: context.products.len() as i32,
            edges: edges
        }
    }

    pub async fn product<'ctx>(context: &'ctx Context, product_id: String) -> Option<Box<Product>> {
        let Context(context) = context;
        let context = context.read().await;

        context
            .products
            .iter()
            .find(|p| p.id() == &product_id)
            .cloned()
    }

    pub async fn categories<'ctx>(context: &'ctx Context) -> Vec<Category> {
        let Context(context) = context;
        let context = context.read().await;

        context.categories().await.to_vec()
    }
}

#[derive(GraphQLScalar)]
#[derive(PartialEq)]
#[derive(Clone)]
#[graphql(transparent)]
pub struct EmailAddressScalar(String);