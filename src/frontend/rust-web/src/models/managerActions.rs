use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphqls",
    query_path = "src/models/managerActionsDeliver.graphql",
    normalization = "rust",
    response_derives = "Debug,Clone"
)]
pub struct Deliver;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphqls",
    query_path = "src/models/managerActionsCancel.graphql",
    normalization = "rust",
    response_derives = "Debug,Clone"
)]
pub struct Cancel;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphqls",
    query_path = "src/models/managerActionsRestock.graphql",
    normalization = "rust",
    response_derives = "Debug,Clone"
)]
pub struct Restock;
