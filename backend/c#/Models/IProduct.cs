namespace GraphQLServer.Models
{
    internal interface IProduct
    {
        string Id { get; }
        string Name { get; }
        string Description { get; }

    }

    internal interface ICursorId
    {
        string Cursor { get; }
    }
}



