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
    pub fn nrOrderd(&self) -> usize {
       let mut ids : Vec<String> =self.in_transit.iter().map(|e| e.id.clone() ).collect();
       ids.dedup();
       ids.iter().count() 
    }

    pub fn nrInTransit(&self) -> usize {
       self.in_transit.iter().filter(|f|f.state == "InTransit").count()
    }

    pub fn nrDeliverd(&self) -> i32 {
      0
    }

    pub fn nrCancelled(&self) -> i32 {
       0
    }
}