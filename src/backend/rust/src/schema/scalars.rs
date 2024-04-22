use juniper::GraphQLScalar;

#[derive(GraphQLScalar, PartialEq, Clone,Debug)]
#[graphql(transparent)]
pub struct EmailAddressScalar(String);
