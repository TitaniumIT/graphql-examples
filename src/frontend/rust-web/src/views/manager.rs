use std::str::FromStr;

use crate::{
    controls::bootstrap::{Card, Table}, http_endpoint, models::{
       cancel, deliver, get_manager_products::{self, GetManagerProductsAllProducts}, Cancel, Deliver, GetManagerProducts
    }, post_graphql
};
use dioxus::prelude::*;
use log::info;
use reqwest::Client;

#[component]
pub fn Manager() -> Element {
    let fetch = use_resource(move || async move {
        let client = get_client();

        let vars = get_manager_products::Variables {};

        let result = post_graphql::<GetManagerProducts, _>(&client, http_endpoint(), vars)
            .await
            .unwrap();

        if result.errors.is_none() {
            Ok(result.data.map(|f| f.all_products).unwrap())
        } else {
            Err("Fetch failed")
        }
    });

    rsx! {
       Card {
        title: "Manager",
        Table {
          columns:[ "Product", "Name", "BasketId", "Actions" ].map(String::from).to_vec(),
             match &*fetch.read_unchecked() {
                Some(Ok(response))=> rsx!{ { response.iter().map(|row| ManagerRow(row)) } },
                Some(Err(e)) => rsx!{},
                None => rsx!{}
          }
        }
      }
    }
}

fn get_client() -> Client {
    Client::builder()
        .default_headers(
            std::iter::once((
                reqwest::header::HeaderName::from_str("managersecret").unwrap(),
                reqwest::header::HeaderValue::from_str("I`m Manager").unwrap(),
            ))
            .collect(),
        )
        .build()
    .unwrap()
}


fn managerAction(action:String,product_id:String) -> Element {
   let action_handler =  match action.as_str() {
       "Deliver" => EventHandler::new(move |e:MouseEvent| { 
               let id = product_id.clone();
               spawn( async move {
                      let client = get_client();
                      info!("Deliver {}",id); 
                      let vars = deliver::Variables {
                          product_id: id
                      };
                    post_graphql::<Deliver,_>(&client, http_endpoint(), vars).await.unwrap();
                });
            }), 
       "Cancel" => EventHandler::new(move|e:MouseEvent| { 
                  let id = product_id.clone();
                  spawn( async move {
                      let client = get_client();
                      info!("Cancel  {}",id); 
                      let vars = cancel::Variables {
                            product_id: id
                      };
                      post_graphql::<Cancel,_>(&client, http_endpoint(), vars).await.unwrap();
                  });
                }), 
       "Restock" =>  EventHandler::new( move|e:MouseEvent| { info!("Restock {}",product_id); }), 
       _ => todo!()
   };

   rsx!{
      button {
        r#type: "button",
        class: "btn btn-primary me-1",
        onclick: move |e| action_handler.call(e),
        "{action}"
      }
   } 

}


fn ManagerRow(product: &GetManagerProductsAllProducts) -> Element {

   let binding = product.actions_allowed().clone();
   let product_id = product.id(); 

   let actions = binding.iter().map(|action| 
           {  
              let product_id = product_id.clone(); 
              managerAction(action.clone(), product_id)
           });

   rsx! {
     tr{
       key: "{product.id()}",
       td{"{product.id()}"},
       td{"{product.name()}"},
       td{"{product.customer_id()}"},
       td{ {actions} }
      },
    }
 }
