use graphql_client::GraphQLQuery;
use shared_types::EmailAddress;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphqls",
    query_path = "src/models/getbasket.graphql",
    normalization = "rust",
    response_derives = "Debug,Clone"
)]
pub struct GetBasketProducts;

type EmailAddressScalar = EmailAddress;

pub trait BasketClientProperies: Sized {
    fn nrOrderd(&self) -> i32;
    fn nrInTransit(&self) -> i32;
    fn nrDeliverd(&self) -> i32;
    fn nrCancelled(&self) -> i32;
    fn name(&self) -> &String;
}

impl BasketClientProperies for get_basket_products::BasketView {
    fn nrOrderd(&self) -> i32 {
       1
    }

    fn nrInTransit(&self) -> i32 {
        todo!()
    }

    fn nrDeliverd(&self) -> i32 {
        todo!()
    }

    fn nrCancelled(&self) -> i32 {
        todo!()
    }

    fn name(&self) -> &String {
       &self.name
    }
}