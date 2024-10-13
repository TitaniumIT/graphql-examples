using GraphQL;
using GraphQL.Types;
using GraphQLServer.Models;
using GraphQLServer.Schema.Types;

namespace GraphQLServer.Schema;
public class ProductContextType : ObjectGraphType
{
    public ProductContextType()
    {
        Description = "Group possiblity, to group queries or mutations";

        Field<ProductType, Product>("ProductGetV2")
            .Argument<StringGraphType>("ProductId")
             .ResolveAsync(async ctx =>
             {
                 return await Task.Run(()=> StaticDataSource.Products.FirstOrDefault(p => p.Id == ctx.GetArgument<string>("ProductId")));
             });
    }
}

