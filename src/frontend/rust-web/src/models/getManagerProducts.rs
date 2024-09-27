use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphqls",
    query_path = "src/models/getManagerProducts.graphql",
    normalization = "rust",
    response_derives = "Debug,Clone"
)]
pub struct GetManagerProducts;