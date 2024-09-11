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

impl get_basket_products::BasketView {
    pub fn nrOrderd(&self) -> i32 {
       1
    }

    pub fn nrInTransit(&self) -> i32 {
       0
    }

    pub fn nrDeliverd(&self) -> i32 {
      0
    }

    pub fn nrCancelled(&self) -> i32 {
       0
    }
}