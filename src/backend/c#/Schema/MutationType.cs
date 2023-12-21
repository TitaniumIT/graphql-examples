using GraphQL;
using GraphQL.Types;
using GraphQLServer.Models;
using GraphQLServer.Models.ValueTypes;
using GraphQLServer.Schema.Scalars;
using GraphQLServer.Schema.Types;

namespace GraphQLServer.Schema;

public class MutationType : ObjectGraphType
{
    enum Arguments { CustomerId }
    public MutationType()
    {
        Field<ProductType>("Buy")
            .Argument<NonNullGraphType<StringGraphType>>("ProductId")
            .Argument<NonNullGraphType<EmailAddressScalar>>(nameof(Arguments.CustomerId))
            .Argument<IntGraphType>("Amount", "Defaults to 1")
            .Resolve(ctx =>
            {
                string productId = ctx.GetArgument<string>("ProductId");
                int amount = ctx.GetArgument<int?>("Amount") ?? 1;

                var product = StaticDataSource.Products.FirstOrDefault(x => x.Id == productId);
                if (product == null)
                {
                    ctx.Errors.Add(new ExecutionError($"Product with id {productId} not found"));
                }
                else
                {
                    if (amount < 0)
                    {
                        ctx.Errors.Add(new($"The specified amount:{amount} less than 0"));
                    }
                    else
                    {
                        if (!product.Buy(amount, ctx.GetArgument<EmailAddress>(nameof(Arguments.CustomerId))))
                        {
                            ctx.Errors.Add(new($"The specified amount:{amount} is not available current stock is {product.InStock}"));
                        }
                    }
                }
                return product;
            });

        Field<ProductInTransitType, ProductInTransit>("Deliver")
            .AuthorizeWithPolicy("IsManager")
            .Argument<NonNullGraphType<StringGraphType>>("ProductInTransitId")
            .Resolve(ctx =>
            {
                var productInTransit = StaticDataSource.ProductInTransit.FirstOrDefault(x => x.Id == ctx.GetArgument<string>("ProductInTransitId"));
                if (productInTransit == null)
                {
                    ctx.Errors.Add(new($"No product in transit found with id {ctx.GetArgument<string>("ProductInTransitId")}"));
                }
                else
                {
                    productInTransit.Deliver();
                }
                return productInTransit;
            });

        Field<ProductInTransitType, ProductInTransit>("Cancel")
          .AuthorizeWithPolicy("IsManager")
          .Argument<NonNullGraphType<StringGraphType>>("ProductInTransitId")
          .Resolve(ctx =>
          {
              var productInTransit = StaticDataSource.ProductInTransit.FirstOrDefault(x => x.Id == ctx.GetArgument<string>("ProductInTransitId"));
              if (productInTransit == null)
              {
                  ctx.Errors.Add(new($"No product in transit found with id {ctx.GetArgument<string>("ProductInTransitId")}"));
              }
              else
              {
                  productInTransit.Cancel();
              }
              return productInTransit;
          });

        Field<ProductInBackorderType, ProductInBackorder>("Restock")
        .AuthorizeWithPolicy("IsManager")
        .Argument<NonNullGraphType<StringGraphType>>("ProductId")
        .Resolve(ctx =>
        {
            var productInBackorder = StaticDataSource.ProductInBackorders.FirstOrDefault(x => x.Id == ctx.GetArgument<string>("ProductId"));
            if (productInBackorder == null)
            {
                ctx.Errors.Add(new($"No product in backorder found with id {ctx.GetArgument<string>("ProductId")}"));
            }
            else
            {
                productInBackorder.Restock();
            }
            return productInBackorder;
        });

    }
}



