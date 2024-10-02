use get_manager_products::GetManagerProductsAllProducts;
use graphql_client::GraphQLQuery;
use shared_types::EmailAddress;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphqls",
    query_path = "src/models/getManagerProducts.graphql",
    normalization = "rust",
    response_derives = "Debug,Clone"
)]
pub struct GetManagerProducts;

type EmailAddressScalar = EmailAddress;

impl GetManagerProductsAllProducts {
    pub fn name(&self) -> String {
        match self {
            Self::Product(p) => p.name.clone(),
            Self::ProductInBackorder(p) => p.name.clone(), 
            Self::ProductInTransit(p) => p.name.clone()
        }
    }
    pub fn id(&self) -> String {
        match self {
            Self::Product(p) => p.id.clone(),
            Self::ProductInBackorder(p) => p.id.clone(), 
            Self::ProductInTransit(p) => p.id.clone()
        }
    }
    pub fn actions_allowed(&self) -> Vec<String> {
        match self {
            Self::Product(p) => p.actions_allowed.clone(),
            Self::ProductInBackorder(p) => p.actions_allowed.clone(), 
            Self::ProductInTransit(p) => p.actions_allowed.clone()
        }
    }
}