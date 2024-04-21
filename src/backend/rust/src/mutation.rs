use juniper::{graphql_object, graphql_value, FieldError, FieldResult};

use crate::{products::intransit::ProductInTransit, Context, EmailAddressScalar, Product};

pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    pub async fn buy<'ctx>(
        context: &'ctx Context,
        product_id: String,
        customer_id: EmailAddressScalar,
        amount: Option<i32>,
    ) -> FieldResult<Option<Product>> {
        let mut products = context.data.products.write().await;

        if let Some(index) = products.iter().position(|p| p.id() == &product_id) {
            let product = products.get_mut(index).unwrap();

            let amt = amount.or_else(|| Some(1)).unwrap();

            if amt < 0 {
                return Err(FieldError::new(
                    format!("The specified amount:{amt} less than 0"),
                    graphql_value!(None),
                ));
            }
            if !(product.buy(amt, customer_id, context.data.clone()).await) {
                return Err(FieldError::new(
                    format!(
                        "The specified amount:{amt} is not available current stock is {}",
                        product.in_stock()
                    ),
                    graphql_value!(None),
                ));
            }

            Ok(Some(product.clone()))
        } else {
            Err(FieldError::new(
                format!("Product with id {product_id} not found"),
                graphql_value!(None),
            ))
        }
    }

    pub async fn deliver<'ctx>(
        context: &'ctx Context,
        product_in_transit_id: String,
    ) -> FieldResult<ProductInTransit> {
        if !context.ismanager {
            Err(FieldError::new(
                "only mnanagers are allowed to deliver",
                graphql_value!(None),
            ))
        } else {
            let mut in_transit = context.data.products_in_transit.write().await;
            if let Some(index) = in_transit.iter().position(|p| p.id() == &product_in_transit_id)
            {
                let product_in_transit = in_transit.get_mut(index).unwrap();
                product_in_transit.deliver().await;
                Ok(product_in_transit.clone())
            } else {
                Err(FieldError::new(
                    format!("No product in transit found with id {product_in_transit_id}"),
                    graphql_value!(None),
                ))
            }
        }
    }

    pub async fn cancel<'ctx>(
        context: &'ctx Context,
        product_in_transit_id: String,
    ) -> FieldResult<ProductInTransit> {
        if !context.ismanager {
            Err(FieldError::new(
                "only mnanagers are allowed to cancel",
                graphql_value!(None),
            ))
        } else {
            let mut in_transit = context.data.products_in_transit.write().await;
            if let Some(index) = in_transit.iter().position(|p| p.id() == &product_in_transit_id)
            {
                let product_in_transit = in_transit.get_mut(index).unwrap();
                product_in_transit.cancel(context.data.clone()).await;
                Ok(product_in_transit.clone())
            } else {
                Err(FieldError::new(
                    format!("No product in transit found with id {product_in_transit_id}"),
                    graphql_value!(None),
                ))
            }
        }
    }

    // Field<ProductInBackorderType, ProductInBackorder>("Restock")
    // .AuthorizeWithPolicy("IsManager")
    // .Argument<NonNullGraphType<StringGraphType>>("ProductId")
    // .Resolve(ctx =>
    // {
    //     var productInBackorder = StaticDataSource.ProductInBackorders.FirstOrDefault(x => x.Id == ctx.GetArgument<string>("ProductId"));
    //     if (productInBackorder == null)
    //     {
    //         ctx.Errors.Add(new($"No product in backorder found with id {ctx.GetArgument<string>("ProductId")}"));
    //     }
    //     else
    //     {
    //         productInBackorder.Restock();
    //     }
    //     return productInBackorder;
    // });
}
