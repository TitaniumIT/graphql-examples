using GraphQL.Types;
using GraphQLServer.Models;
using GraphQLServer.Schema.Interfaces;

namespace GraphQLServer.Schema.Types;

internal class ProductInBackorderType : AutoRegisteringObjectGraphType<ProductInBackorder>
{
    public ProductInBackorderType()
    {
        Interface<ProductTypeInterface>();
        Interface<AvailableActionsInterfaceType>();
        Field<ListGraphType<StringGraphType>>("ActionsAllowed")
            .Resolve(ctx =>
            {
                var actions = new List<string>();
                if (StaticDataSource.ProductInBackorders.Any(x => x.Id == ctx.Source.Id))
                {
                    actions.Add("Restock");
                }
                return actions;
            });
    }
}



