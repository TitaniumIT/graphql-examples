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


/*
    inBasket @client
    nrOrderd @client
    nrInTransit @client
    nrDeliverd @client
    nrCancelled @client  */