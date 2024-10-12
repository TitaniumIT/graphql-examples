use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphqls",
    query_path = "src/models/getproducts.graphql",
    normalization = "rust",
    response_derives = "Debug,Clone,PartialEq,Eq,Default",
    variables_derives = "Clone"
)]
pub struct GetProducts;

impl get_products::productView {
    pub fn canBuy(&self) -> bool {
        self.actions_allowed.contains(&"Buy".to_string())
    }

    pub fn isProcessing(&self) -> bool {
        self.actions_allowed.contains(&"Processing".to_string())
    }
}
