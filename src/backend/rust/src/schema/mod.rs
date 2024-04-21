use std::sync::Arc;

use juniper::RootNode;

use crate::staticdata::StaticData;

use self::{mutation::Mutation, query::Query, subscription::Subscriptions};

pub mod query;
pub mod mutation;
pub mod subscription;
pub mod scalars;
pub mod relaytypes;

#[derive(Clone)]
pub struct Context {
    pub data: Arc<StaticData>,
    pub ismanager: bool,
}

impl juniper::Context for Context {}

pub type Schema = RootNode<'static, Query, Mutation, Subscriptions>;

pub fn schema() -> Schema {
    Schema::new(Query, Mutation, Subscriptions)
}

