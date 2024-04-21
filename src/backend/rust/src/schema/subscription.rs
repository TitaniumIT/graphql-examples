use std::pin::Pin;

use futures::Stream;
use juniper::{graphql_subscription, FieldError};

use crate::{products::intransit::ProductInTransit, scalars::EmailAddressScalar, Context};
pub struct Subscriptions;

type ProductsIntransitStream =
    Pin<Box<dyn Stream<Item = Result<ProductInTransit, FieldError>> + Send>>;

#[graphql_subscription(context = Context)]
impl Subscriptions {
    pub async fn status_changed<'ctx>(
        context: &'ctx Context,
        customer_id: EmailAddressScalar,
    ) -> ProductsIntransitStream {
        let (_s,r) = &context.data.status_channel;
        let receiver= r.clone();
        let stream = async_stream::stream! {
            loop {
                if let Ok(product_changed) = receiver.recv().await {
                    if product_changed.customer_id() == &customer_id {
                        yield Ok(product_changed.clone())
                    }
                }
            }
        };
        Box::pin(stream)
    }
}
