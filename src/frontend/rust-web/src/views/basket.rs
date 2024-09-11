use std::fmt::Display;

use dioxus::prelude::*;
use shared_types::EmailAddress;

use crate::{controls::bootstrap::Table, models::{buy_product::productsInTransit, get_basket_products::BasketView, BasketClientProperies}};

#[derive(Clone, Debug, PartialEq)]
pub enum CustomerId {
    ValidEmail(EmailAddress),
    Invalid(String),
    Default,
}

impl CustomerId {
    fn is_not_default(&self) -> bool {
       if let Self::Default = self { false } else { true }
    }
    fn is_default(&self) -> bool {
       if let Self::Default = self { true } else { false }
    }
}

impl Display for CustomerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ValidEmail(address) => f.write_str(&address.to_string()),
            Self::Invalid(data) => f.write_str(data),
            Self::Default => f.write_str("Type your email"),
        }
    }
}

#[derive(Default,Clone)]
pub struct ProductsInTransiteCache {
    pub basket: Vec<Signal<BasketView>>
}

pub fn Basket() -> Element {
    let mut customer_id = use_context::<Signal<CustomerId>>();
    let mut list = use_context::<Signal<ProductsInTransiteCache>>();
    rsx! {
        div{
            input {
                class: format!("form-control {}" , if let CustomerId::Invalid(_) = *customer_id.read() { "is-invalid"} else { "is-valid"}),
                id:"customerid",
                value: if customer_id.read().is_not_default() { "{customer_id}" },
                required:true,
                placeholder: if customer_id.read().is_default() { "{customer_id}" } ,
                r#type:"email",
                oninput: move |event|{
                    let result =  EmailAddress::new(&event.value());
                    if let Ok(email) = result {
                        customer_id.set(CustomerId::ValidEmail(email));
                    } else {
                        if event.value().is_empty() {
                            customer_id.set(CustomerId::Default)
                        } else {
                            customer_id.set(CustomerId::Invalid(event.value()));
                        }
                    }
                }
            },
            div {
                class: "invalid-feedback",
                "Invalid email {customer_id}"
            }
        }
        Table {
            caption: "Basket for {customer_id}",
            columns: [ "Name" ,"ordered" ,"intansit" ,"deliverd" ,"cancelled" ].map(String::from).to_vec(),
            for row in list.read().basket.iter() {
                BasketRowView {
                    basket_row: *row
                }
            }
        }
    }
   }
 
    //             <tr *ngFor="let product of inBasket" scope="row" >
    //                 <td>{{product.name}}</td>
    //                 <td>{{product.nrOrderd}}</td>
    //                 <td>{{ product.nrInTransit }}</td>
    //                 <td>{{ product.nrDeliverd }}</td>
    //                 <td>{{ product.nrCancelled }}</td>
    //             </tr>
    //     </tbody>
    // </table>

#[component]
fn BasketRowView(basket_row: Signal<BasketView>) -> Element {
    let row = basket_row.read();
    rsx! {
       tr { 
         td { "{row.name()}" }
         td { "{row.nrOrderd()}" }
         }
    }
}
