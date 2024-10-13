using GraphQL.Types;

namespace GraphQLServer.Schema.Interfaces
{
    internal class AvailableActionsInterfaceType : InterfaceGraphType
    {
        public AvailableActionsInterfaceType()
        {
            Field<ListGraphType<StringGraphType>>("ActionsAllowed");
        }
    }
}



