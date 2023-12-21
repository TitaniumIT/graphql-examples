using GraphQL.Types;
using GraphQLServer.Models;

namespace GraphQLServer.Schema.Types
{
    internal class AllProductTypes : UnionGraphType
    {
        public AllProductTypes()
        {
            Type<ProductType>();
            Type<ProductInTransitType>();
            Type<ProductInBackorderType>();
        }
    }
}



