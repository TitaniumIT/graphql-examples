using GraphQL;
using GraphQL.Instrumentation;
using GraphQL.Types;
using GraphQL.Types.Relay.DataObjects;
using GraphQLServer.Models;
using GraphQLServer.Schema.Types;
using System.Linq;

namespace GraphQLServer.Schema
{
    public class QueryType : ObjectGraphType
    {
        public QueryType()
        {

            Description = 
@"# Query entrypoint
also **markdown** is supported,
## Links
* for markdown see [Markdown Guide](https://www.markdownguide.org)
* Created by [Titanium IT](https://www.titaniumit.nl)";

            Field<ListGraphType<ProductType>, List<Product>>("Products")
                .Description("All products without paging")
                .DeprecationReason("Use Skip and Take or the relay version")
                .Resolve(ctx =>
                {
                    return StaticDataSource.Products;
                });

            Field<ListGraphType<ProductType>, List<Product>>("ProductsSkipTake")
                .Argument<IntGraphType>("Skip", options => { options.DefaultValue = 0; })
                .Argument<IntGraphType>("Take", options => { options.DefaultValue = 5; })
                .Resolve(ctx =>
                {
                    return StaticDataSource.Products.OrderBy(x => x.Id).Skip(ctx.GetArgument<int>("Skip")).Take(ctx.GetArgument<int>("Take")).ToList();
                });

            Connection<ProductType>()
                .Name("ProductsRelay")
                .Bidirectional() 
                .PageSize(5)
                .ResolveAsync(async ctx =>
                {
                    Task<IEnumerable<Product>> GetSource() =>
                        Task.Run(() => {
                            if (ctx.First.HasValue)
                            {
                                return StaticDataSource.Products.Where(p => p.Id.CompareTo(ctx.After) > 0).Take(ctx.First.Value);
                            }
                            if (ctx.Last.HasValue)
                            {
                                return StaticDataSource.Products.OrderByDescending(c =>c.Id).Where(p => p.Id.CompareTo(ctx.Before) < 0).Take(ctx.Last.Value);
                            }
                            return Array.Empty<Product>();
                        });


                    var edges = (await GetSource()).OrderBy( p => p.Id).Select(p => new Edge<Product>() { Cursor = p.Id, Node = p }).ToList();

                    return new Connection<Product>()
                    {
                        Edges = edges,
                        TotalCount = StaticDataSource.Products.Count(),
                        PageInfo = new ()
                        {
                            EndCursor = edges.Last().Cursor,
                            StartCursor = edges.First().Cursor,
                            HasNextPage = edges.Last().Cursor != StaticDataSource.Products.Max( p => p.Id),
                            HasPreviousPage = edges.First().Cursor != StaticDataSource.Products.Min( p => p.Id)
                        }
                    };

                });

            Field<ProductType, Product>("Product")
                .Argument<NonNullGraphType<StringGraphType>>("ProductId")
                .Resolve(ctx =>
                {
                    return StaticDataSource.Products.FirstOrDefault(p => p.Id == ctx.GetArgument<string>("productId"));
                });

            Field<ListGraphType<CategoryType>, List<Category>>("Categories")
               .Resolve(ctx =>
               {
                   return StaticDataSource.Categories;
               });

            Field<ListGraphType<AllProductTypes>, List<IProduct>>("AllProducts")
              .AuthorizeWithPolicy("IsManager")
              .Resolve(ctx =>
              {
                  return StaticDataSource.ProductInTransit.Cast<IProduct>()
                            .Concat(StaticDataSource.Products.Cast<IProduct>())
                            .Concat(StaticDataSource.ProductInBackorders.Cast<IProduct>()).ToList();
              });

            Field<ProductContextType>("ProductContext");

            IsTypeOf = obj => obj is object;
        }
    }
}



