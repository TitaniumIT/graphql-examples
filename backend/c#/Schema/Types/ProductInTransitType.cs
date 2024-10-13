using GraphQL.Types;
using GraphQLServer.Models;
using GraphQLServer.Schema.Interfaces;
using GraphQLServer.Schema.Scalars;

namespace GraphQLServer.Schema.Types;

internal class ProductInTransitType : AutoRegisteringObjectGraphType<ProductInTransit>
{
    public ProductInTransitType() : base(  )
    {
        Interface<ProductTypeInterface>();
        Interface<AvailableActionsInterfaceType>();

        Field<ListGraphType<StringGraphType>>("ActionsAllowed")
            .Resolve(ctx =>
            {
                var actions = new List<string>();
                if (ctx.Source.State == "InTransit")
                {
                    actions.Add("Deliver");
                    actions.Add("Cancel");
                }

                return actions;
            });
    }
}



