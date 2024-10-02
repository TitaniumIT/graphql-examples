use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphqls",
    query_path = "src/models/managerActions.graphql",
    normalization = "rust",
    response_derives = "Debug,Clone"
)]
pub struct Deliver;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphqls",
    query_path = "src/models/managerActions.graphql",
    normalization = "rust",
    response_derives = "Debug,Clone"
)]
pub struct Cancel;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphqls",
    query_path = "src/models/managerActions.graphql",
    normalization = "rust",
    response_derives = "Debug,Clone"
)]
pub struct Restock;
