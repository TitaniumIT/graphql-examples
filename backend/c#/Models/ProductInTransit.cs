using GraphQLServer.Models.ValueTypes;

namespace GraphQLServer.Models
{
    internal class ProductInTransit : IProduct
    {
        readonly Product _product;

        static long _id = 100000;

        public ProductInTransit(Product productWithStock, EmailAddress customerId)
        {
            _product = productWithStock;
            CustomerId = customerId;
            State = "InTransit";
            Id = $"{_id++}";
        }

        public string Id { get; }
        public string ProductId => ((IProduct) _product).Id;
        public string Name => ((IProduct)_product).Name;
        public string Description => ((IProduct)_product).Description;
        public EmailAddress CustomerId { get; }
        public string State { get; internal set; }

        public void Deliver()
        {
            State = "Deliverd";
            StaticDataSource.ProductInTransitEvents.OnNext(this);
        }

        public void Cancel()
        {
            State = "Cancelled";
            _product.Restock();
            StaticDataSource.ProductInTransitEvents.OnNext(this);
        }
    }
}



