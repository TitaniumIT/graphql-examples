use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphqls",
    query_path = "src/models/getproduct.graphql",
    normalization = "rust",
    response_derives = "Debug,Clone"
)]
pub struct GetProduct;

impl get_product::productDetailView {
    pub fn hasCategory(&self, category: &get_product::categoryView) -> bool {
        self.selected_categories.iter().any(|c| c.id == category.id)
    }
}

