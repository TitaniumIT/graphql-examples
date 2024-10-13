using GraphQLServer.Models.ValueTypes;

namespace GraphQLServer.Models
{
    internal class Product: IProduct 
    {
        public string Id { get; }
        public string Name { get; }
        public string Description { get; }
        public int InStock { get; internal set; }
        public IReadOnlyCollection<string> CategoryIds { get; }

        public Product(string id, string name, string description, string[] categoryIds, int inStock)
        {
            Id = id;
            Name = name;
            Description = description;
            CategoryIds = categoryIds;
            InStock = inStock;
        }

        public bool IsAllowedToBuy() => InStock > 0;

        internal bool Buy(int amount,EmailAddress customerId)
        {
            if (InStock - amount < 0) return false;
            InStock -= amount;
            Thread.Sleep(5000);

            if ( InStock == 0)
            {
                StaticDataSource.ProductInBackorders.Add(new(this));
            }

            StaticDataSource.ProductInTransit.Add(new(this,customerId));

            return true;
        }

        internal void Restock()
        {
            InStock += 1;
        }
    }
}



