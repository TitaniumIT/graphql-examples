using GraphQL;
using GraphQL.DataLoader;
using GraphQL.Reflection;
using GraphQL.Types;
using GraphQLServer.Models;
using GraphQLServer.Models.ValueTypes;
using GraphQLServer.Schema.Interfaces;
using GraphQLServer.Schema.Scalars;
using System.Threading.Tasks;

namespace GraphQLServer.Schema.Types
{
    internal class ProductType : AutoRegisteringObjectGraphType<Product>
    {
        private readonly ILogger<ProductType> _logger;

        enum Arguments {  CustomerId }
        public ProductType(IDataLoaderContextAccessor dataLoader, ILogger<ProductType> logger) : base(product => product.CategoryIds)
        {
            _logger = logger;
            Interface<ProductTypeInterface>();

            Interface<AvailableActionsInterfaceType>();
            Field<ListGraphType<StringGraphType>>("ActionsAllowed")
                .Resolve(ctx =>
                {
                    var actions = new List<string>();
                    if (ctx.Source.IsAllowedToBuy() && (!ctx.User?.IsInRole("Manager") ?? true))
                    {
                        actions.Add("Buy");
                    }
                    return actions;
                });

            Field<ListGraphType<CategoryType>, List<Category>>("Categories")
                .ResolveAsync(ctx =>
                {
                    var loader = dataLoader.Context!.GetOrAddBatchLoader<Product, List<Category>>("GetCategories"
                        , GetCategoriesByBatch) ?? throw new NullReferenceException();

                    return loader.LoadAsync(ctx.Source);
                });

            Field<ListGraphType<CategoryType>, List<Category>>("CategoriesWithoutBatchAsync")
             .Description("If only id,__typename field is requested , no backend call for categories made")
             .ResolveAsync(async ctx =>
             {
                 if (ctx.SubFields?.All(field => field.Value.Field.Name.Value == "id" || field.Value.Field.Name.Value == "__typename") ?? false)
                 {
                     return ctx.Source.CategoryIds.Select(id => new Category(id)).ToList();
                 }
                 else
                 {
                     return await GetCategoriesAsync(ctx.Source);
                 }

             });

            Field<ListGraphType<CategoryType>, List<Category>>("CategoriesWithoutBatchSync")
          .Description("If only id,__typename field is requested , no backend call for categories made")
          .Resolve(ctx =>
          {
              if (ctx.SubFields?.All(field => field.Value.Field.Name.Value == "id" || field.Value.Field.Name.Value == "__typename") ?? false)
              {
                  return ctx.Source.CategoryIds.Select(id => new Category(id)).ToList();
              }
              else
              {
                  return GetCategories(ctx.Source);
              }

          });

            Field<ListGraphType<ProductInTransitType>, List<ProductInTransit>>("ProductsInTransit")
                .Argument<NonNullGraphType<EmailAddressScalar>>(nameof(Arguments.CustomerId))
                .Resolve(ctx =>
                {
                    return StaticDataSource
                        .ProductInTransit
                            .Where(x => x.ProductId == ctx.Source.Id && x.CustomerId == ctx.GetArgument<EmailAddress>(nameof(Arguments.CustomerId))).ToList();
                });
        }

        List<Category> GetCategories(Product product)
        {
            _logger.LogInformation("Called GetCategories");
            return StaticDataSource.Categories.Where(cat => product.CategoryIds.Contains(cat.Id)).ToList();
        }

        Task<List<Category>> GetCategoriesAsync(Product product)
        {
            _logger.LogInformation("Called GetCategories");
            return Task.Run(() => StaticDataSource.Categories.Where(cat => product.CategoryIds.Contains(cat.Id)).ToList());
        }

        Task<IDictionary<Product, List<Category>>> GetCategoriesByBatch(IEnumerable<Product> products)
        {
            _logger.LogInformation("Called GetCategoriesByBatch");
            var uniqueIds = products.SelectMany(p => p.CategoryIds).Distinct();

            var categories = StaticDataSource.Categories.Where(cat => uniqueIds.Contains(cat.Id));

            return Task.FromResult<IDictionary<Product, List<Category>>>(
                products.
                    Select(p => new
                    {
                        key = p,
                        values = categories.Where(c => p.CategoryIds.Contains(c.Id)).ToList()
                    })
                    .ToDictionary(x => x.key, y => y.values));
        }
    }
}



