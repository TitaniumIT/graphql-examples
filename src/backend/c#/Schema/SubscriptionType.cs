using GraphQL;
using GraphQL.Types;
using GraphQLServer.Models;
using GraphQLServer.Models.ValueTypes;
using GraphQLServer.Schema.Scalars;
using GraphQLServer.Schema.Types;
using System.Reactive.Linq;

namespace GraphQLServer.Schema
{
    public class SubscriptionType : ObjectGraphType
    {
        enum Arguments { CustomerId }
        public SubscriptionType()
        {
          
            Field<ProductInTransitType, ProductInTransit>("StatusChanged")
                .Argument<EmailAddressScalar>(nameof(Arguments.CustomerId))
                .ResolveStream((ctx) =>
                {
                    return StaticDataSource.ProductInTransitEvents.Where(x => x.CustomerId.Address == ctx.GetArgument<EmailAddress>(nameof(Arguments.CustomerId)).Address);
                });

        }
    }
}