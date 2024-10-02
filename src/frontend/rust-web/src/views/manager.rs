
use std::str::FromStr;

use dioxus::prelude::*;
use graphql_client::reqwest::post_graphql;
use log::info;
use reqwest::Client;
use crate::{controls::bootstrap::{Card, Input, Table}, models::{get_manager_products, GetManagerProducts}, APIURL};

#[component]
pub fn Manager() -> Element {

    let fetch = use_resource(move || async move {

     let client = Client::builder()
        .default_headers(
            std::iter::once((
                reqwest::header::HeaderName::from_str("managersecret").unwrap(),
                reqwest::header::HeaderValue::from_str("I'm Manager").unwrap()
            )).collect(),
        )
        .build().unwrap(); 
    
      let vars= get_manager_products::Variables{
      };

      let result = post_graphql::<GetManagerProducts,_>(&client, APIURL, vars)
        .await
        .unwrap();

       result.data
    });
      
    rsx!{
       Card {
        title: "Manager",
       Table {
         columns:[
            "Product",
            "Name",
            "BasketId",
            "Actions"
         ].map(String::from).to_vec()
       }
      }
    }
}

//<p>manager</p>
//
//<table class="table">
//    <thead>
//        <th scope="col">Product kind</th>
//        <th scope="col">Name</th>
//        <th scope="col">basketId</th>
//        <th scope="col">Actions</th>
//    </thead>
//    <tbody *ngIf="! loading">
//            <tr *ngFor="let product of products" scope="row">
//                <td>{{product.__typename}}</td>
//                <td>{{product.name}}</td>
//                <td>{{ basketId(product)  }}</td>
//                <td>
//                   <button *ngFor="let action of product.actionsAllowed" type="button" class="btn btn-primary me-1" (click)="doAction( product , action!)">
//                        {{action}}
//                   </button>
//                </td>
//            </tr>
//    </tbody>
//    <tbody *ngIf="loading">
//        <tr>
//            <td colspan="4">Loading</td>
//        </tr>
//    </tbody>
//</table>