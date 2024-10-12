use graphql_client::GraphQLQuery;
use shared_types::EmailAddress;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphqls",
    query_path = "src/models/basketEvents.graphql",
    normalization = "rust",
    response_derives = "Debug,Clone"
)]
pub struct BasketEvents;

type EmailAddressScalar = EmailAddress;
