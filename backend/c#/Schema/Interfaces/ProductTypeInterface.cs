using GraphQL.Types;
using GraphQLServer.Models;

namespace GraphQLServer.Schema.Interfaces
{
    internal class ProductTypeInterface : AutoRegisteringInterfaceGraphType<IProduct>
    {
        public ProductTypeInterface()
        {
        }
    }
}



