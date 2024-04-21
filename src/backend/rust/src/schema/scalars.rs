use juniper::GraphQLScalar;

#[derive(GraphQLScalar, PartialEq, Clone)]
#[graphql(transparent)]
pub struct EmailAddressScalar(String);
