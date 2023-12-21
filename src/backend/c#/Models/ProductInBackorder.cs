namespace GraphQLServer.Models
{
    internal class ProductInBackorder : IProduct
    {
        static long _id = 20000;
        readonly Product _product;
        public ProductInBackorder(Product productWithStock)
        {
            _product = productWithStock;
        }

        public string Id =>  $"{_id++}";
        public string ProductId => _product.Id;
        public string Name => ((IProduct)_product).Name;
        public string Description => ((IProduct)_product).Description;

        public void Restock()
        {
            StaticDataSource.ProductInBackorders.Remove(this);
            _product.Restock();
        }
    }
}




