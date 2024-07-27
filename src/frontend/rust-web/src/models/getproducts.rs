use graphql_client:: GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphqls",
    query_path = "src/models/getproducts.graphql",
    normalization = "rust",
    response_derives = "Debug,Clone"
)]
pub struct GetProducts;

impl get_products::productView {
    pub fn canBuy(&self) -> bool {
        self.actions_allowed.contains(&"Buy".to_string())
    }
}