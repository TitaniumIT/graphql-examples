using GraphQLServer.Models;
using GraphQLServer.Models.ValueTypes;
using GraphQLServer.Schema.Scalars;
using GraphQLServer.Schema.Types;

namespace GraphQLServer.Schema
{
    public class Domain : GraphQL.Types.Schema
    {
        public Domain(IServiceProvider services) : base(services)
        {
            Query = services.GetRequiredService<QueryType>();
            Mutation = services.GetRequiredService<MutationType>();
            Subscription = services.GetRequiredService<SubscriptionType>();

            RegisterTypeMapping(typeof(Category), typeof(CategoryType));
            RegisterTypeMapping(typeof(EmailAddress), typeof(EmailAddressScalar));
        }
    }
}



