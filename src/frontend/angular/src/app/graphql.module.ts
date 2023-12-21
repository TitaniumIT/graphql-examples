import { NgModule } from '@angular/core';
import { ApolloModule, APOLLO_OPTIONS } from 'apollo-angular';
import { ApolloClientOptions, InMemoryCache, split } from '@apollo/client/core';
import { HttpLink } from 'apollo-angular/http';
import { createPersistedQueryLink } from "@apollo/client/link/persisted-queries";
import { onError } from '@apollo/client/link/error';
import { ToastrService } from 'ngx-toastr';
import { getMainDefinition, Reference, relayStylePagination } from '@apollo/client/utilities';
import { OperationDefinitionNode } from 'graphql';
import result from './models/generated-cacheOptions';
import { StrictTypedTypePolicies } from './models/generated-clientHelpers'
import { sha256 } from 'crypto-hash'
import { BasketService } from './services/basket-service.service';
import { GraphQLWsLink } from "@apollo/client/link/subscriptions";
import { createClient } from 'graphql-ws';
import { merge } from 'rxjs';


const uri = 'localhost:7265/graphql'; // <-- add the URL of the GraphQL server here
export function createApollo(httpLink: HttpLink, toastr: ToastrService): ApolloClientOptions<any> {
  const http = httpLink.create({ uri:`https://${uri}` });
  const error = onError(({ graphQLErrors, networkError }) => {
    if (graphQLErrors) {

    }
    if (networkError) {
      toastr.error(networkError.message, "Product service is down");
    }
  });


  const ws = new GraphQLWsLink(
    createClient({
      url: `wss://`+uri,
    })
  );
  
  const persistedQueriesLink = createPersistedQueryLink({
      sha256,useGETForHashedQueries:true
    });

  const spliter = split(
    ({ query }) => {
      const { kind, operation } = getMainDefinition(query) as OperationDefinitionNode;
      return (
        kind === 'OperationDefinition' && operation === 'subscription'
      );
    },
    ws,
    persistedQueriesLink.concat(http),
  );

  const typePolicies: StrictTypedTypePolicies = {
    QueryType:{
      fields:{
        productsRelay: relayStylePagination()
      }
    },
    ProductConnection:{
      fields:{ 
        items:{
          merge: (e:Reference[],n:Reference[],options)=>{
            if( !e) return n;
            const merged = [...e];
            n.forEach(element => {
              console.log(element);
              if ( merged.find((old)=> old.__ref == element.__ref )  == null){
                merged.push(element);
              }
            });
            return merged;
          }
        }
      }
    },
    Product: {
      fields: {
        productsInTransit:{
          merge: (e:[],n:[],options)=>{
            if( !n)
              return e;
            else 
              return n;
          }
        },
        ... BasketService.ProductLocalFields()
      }
    }
  }
  const link = error.concat(spliter);
  return {
    link,
    cache: new InMemoryCache({
      possibleTypes: result.possibleTypes,
      typePolicies:{
        ... typePolicies
      } ,
    }),
  };
}

@NgModule({
  exports: [ApolloModule],
  providers: [
    {
      provide: APOLLO_OPTIONS,
      useFactory: createApollo,
      deps: [HttpLink, ToastrService ],
    },
  ],
})
export class GraphQLModule { }


