import gql from 'graphql-tag';
import { Injectable } from '@angular/core';
import * as Apollo from 'apollo-angular';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: string;
  String: string;
  Boolean: boolean;
  Int: number;
  Float: number;
  EmailAddressScalar: string;
};

export type BasketEventsVariables = Exact<{
  customerId: Scalars['EmailAddressScalar'];
}>;


export type BasketEvents = { __typename?: 'SubscriptionType', statusChanged?: (
    { __typename?: 'ProductInTransit' }
    & ProductInTransitView
  ) | null };

export type ProductInTransitView = { __typename?: 'ProductInTransit', id: string, productId: string, state: string, name: string };

export type BuyProductVariables = Exact<{
  productId: Scalars['String'];
  customerId: Scalars['EmailAddressScalar'];
}>;


export type BuyProduct = { __typename?: 'MutationType', buy?: (
    { __typename?: 'Product' }
    & BuyProductView
  ) | null };

export type BuyProductView = { __typename?: 'Product', id: string, inStock: number, actionsAllowed?: Array<string | null> | null, name: string, inBasket?: boolean | null, inTransit?: Array<{ __typename?: 'ProductInTransit', id: string, productId: string, state: string } | null> | null };

export type GetBasketProductsVariables = Exact<{
  customerId: Scalars['EmailAddressScalar'];
}>;


export type GetBasketProducts = { __typename?: 'QueryType', inBasketProducts?: { __typename?: 'ProductConnection', items?: Array<(
      { __typename?: 'Product' }
      & BasketView
    ) | null> | null } | null };

export type BasketView = { __typename?: 'Product', id: string, name: string, inBasket?: boolean | null, nrOrderd?: number | null, nrInTransit?: number | null, nrDeliverd?: number | null, nrCancelled?: number | null, inTransit?: Array<{ __typename?: 'ProductInTransit', id: string, productId: string, state: string } | null> | null };

export type GetManagerProductsVariables = Exact<{ [key: string]: never; }>;


export type GetManagerProducts = { __typename?: 'QueryType', allProducts?: Array<(
    { __typename?: 'Product' }
    & AnyProductView_Product_
  ) | (
    { __typename?: 'ProductInBackorder' }
    & AnyProductView_ProductInBackorder_
  ) | (
    { __typename?: 'ProductInTransit' }
    & AnyProductView_ProductInTransit_
  ) | null> | null };

export type AnyProductView_Product_ = { __typename?: 'Product', id: string, name: string, actionsAllowed?: Array<string | null> | null };

export type AnyProductView_ProductInBackorder_ = { __typename?: 'ProductInBackorder', id: string, name: string, actionsAllowed?: Array<string | null> | null };

export type AnyProductView_ProductInTransit_ = { __typename?: 'ProductInTransit', id: string, name: string, actionsAllowed?: Array<string | null> | null, customerId?: string | null };

export type AnyProductView = AnyProductView_Product_ | AnyProductView_ProductInBackorder_ | AnyProductView_ProductInTransit_;

export type GetProductVariables = Exact<{
  productId: Scalars['String'];
  loadCategories: Scalars['Boolean'];
}>;


export type GetProduct = { __typename?: 'QueryType', product?: (
    { __typename?: 'Product' }
    & ProductDetailView
  ) | null, categories?: Array<(
    { __typename?: 'Category' }
    & CategoryView
  ) | null> | null };

export type CategoryView = { __typename?: 'Category', id: string, name: string };

export type ProductDetailView = { __typename?: 'Product', id: string, name: string, description: string, inStock: number, actionsAllowed?: Array<string | null> | null, selectedCategories?: Array<{ __typename?: 'Category', id: string } | null> | null };

export type GetProductsVariables = Exact<{
  first?: InputMaybe<Scalars['Int']>;
  after?: InputMaybe<Scalars['String']>;
  last?: InputMaybe<Scalars['Int']>;
  before?: InputMaybe<Scalars['String']>;
}>;


export type GetProducts = { __typename?: 'QueryType', productsRelay?: { __typename?: 'ProductConnection', totalCount?: number | null, edges?: Array<{ __typename?: 'ProductEdge', node?: (
        { __typename?: 'Product' }
        & ProductView
      ) | null } | null> | null, pageInfo: (
      { __typename?: 'PageInfo' }
      & PageInfoView
    ), items?: Array<(
      { __typename?: 'Product' }
      & ProductView
    ) | null> | null } | null };

export type PageInfoView = { __typename?: 'PageInfo', hasNextPage: boolean, hasPreviousPage: boolean, startCursor?: string | null, endCursor?: string | null };

export type ProductView = { __typename?: 'Product', id: string, name: string, description: string, inStock: number, actionsAllowed?: Array<string | null> | null };

export type DeliverVariables = Exact<{
  productId: Scalars['String'];
}>;


export type Deliver = { __typename?: 'MutationType', response?: { __typename?: 'ProductInTransit', id: string, actionsAllowed?: Array<string | null> | null } | null };

export type CancelVariables = Exact<{
  productId: Scalars['String'];
}>;


export type Cancel = { __typename?: 'MutationType', response?: { __typename?: 'ProductInTransit', id: string, actionsAllowed?: Array<string | null> | null } | null };

export type RestockVariables = Exact<{
  productId: Scalars['String'];
}>;


export type Restock = { __typename?: 'MutationType', response?: { __typename?: 'ProductInBackorder', id: string, actionsAllowed?: Array<string | null> | null } | null };

export const ProductInTransitView = gql`
    fragment ProductInTransitView on ProductInTransit {
  id
  productId
  state
  name
}
    `;
export const BuyProductView = gql`
    fragment buyProductView on Product {
  id
  inStock
  actionsAllowed
  name
  inBasket @client
  inTransit: productsInTransit(customerId: $customerId) {
    id
    productId
    state
  }
}
    `;
export const BasketView = gql`
    fragment BasketView on Product {
  id
  name
  inBasket @client
  nrOrderd @client
  nrInTransit @client
  nrDeliverd @client
  nrCancelled @client
  inTransit: productsInTransit(customerId: $customerId) {
    id
    productId
    state
  }
}
    `;
export const AnyProductView = gql`
    fragment AnyProductView on AllProductTypes {
  ... on IProduct {
    id
    name
  }
  ... on AvailableActionsInterfaceType {
    actionsAllowed
  }
  ... on ProductInTransit {
    customerId
  }
}
    `;
export const CategoryView = gql`
    fragment CategoryView on Category {
  id
  name
}
    `;
export const ProductDetailView = gql`
    fragment ProductDetailView on Product {
  id
  name
  description
  inStock
  actionsAllowed
  selectedCategories: categoriesWithoutBatchAsync {
    id
  }
}
    `;
export const PageInfoView = gql`
    fragment pageInfoView on PageInfo {
  hasNextPage
  hasPreviousPage
  startCursor
  endCursor
}
    `;
export const ProductView = gql`
    fragment productView on Product {
  id
  name
  description
  inStock
  actionsAllowed
}
    `;
export const BasketEventsDocument = gql`
    subscription basketEvents($customerId: EmailAddressScalar!) {
  statusChanged(customerId: $customerId) {
    ...ProductInTransitView
  }
}
    ${ProductInTransitView}`;

  @Injectable({
    providedIn: 'root'
  })
  export class BasketEventsSubscriptionService extends Apollo.Subscription<BasketEvents, BasketEventsVariables> {
    override document = BasketEventsDocument;
    
    constructor(apollo: Apollo.Apollo) {
      super(apollo);
    }
  }
export const BuyProductDocument = gql`
    mutation buyProduct($productId: String!, $customerId: EmailAddressScalar!) {
  buy(productId: $productId, customerId: $customerId) {
    ...buyProductView
  }
}
    ${BuyProductView}`;

  @Injectable({
    providedIn: 'root'
  })
  export class BuyProductMutationService extends Apollo.Mutation<BuyProduct, BuyProductVariables> {
    override document = BuyProductDocument;
    
    constructor(apollo: Apollo.Apollo) {
      super(apollo);
    }
  }
export const GetBasketProductsDocument = gql`
    query getBasketProducts($customerId: EmailAddressScalar!) {
  inBasketProducts: productsRelay {
    items {
      ...BasketView
    }
  }
}
    ${BasketView}`;

  @Injectable({
    providedIn: 'root'
  })
  export class GetBasketProductsQueryService extends Apollo.Query<GetBasketProducts, GetBasketProductsVariables> {
    override document = GetBasketProductsDocument;
    
    constructor(apollo: Apollo.Apollo) {
      super(apollo);
    }
  }
export const GetManagerProductsDocument = gql`
    query getManagerProducts {
  allProducts {
    ...AnyProductView
  }
}
    ${AnyProductView}`;

  @Injectable({
    providedIn: 'root'
  })
  export class GetManagerProductsQueryService extends Apollo.Query<GetManagerProducts, GetManagerProductsVariables> {
    override document = GetManagerProductsDocument;
    
    constructor(apollo: Apollo.Apollo) {
      super(apollo);
    }
  }
export const GetProductDocument = gql`
    query getProduct($productId: String!, $loadCategories: Boolean!) {
  product(productId: $productId) {
    ...ProductDetailView
  }
  categories @include(if: $loadCategories) {
    ...CategoryView
  }
}
    ${ProductDetailView}
${CategoryView}`;

  @Injectable({
    providedIn: 'root'
  })
  export class GetProductQueryService extends Apollo.Query<GetProduct, GetProductVariables> {
    override document = GetProductDocument;
    
    constructor(apollo: Apollo.Apollo) {
      super(apollo);
    }
  }
export const GetProductsDocument = gql`
    query getProducts($first: Int, $after: String, $last: Int, $before: String) {
  productsRelay(first: $first, after: $after, last: $last, before: $before) @connection(key: "productsRelay") {
    edges {
      node {
        ...productView
      }
    }
    totalCount
    pageInfo {
      ...pageInfoView
    }
    items {
      ...productView
    }
  }
}
    ${ProductView}
${PageInfoView}`;

  @Injectable({
    providedIn: 'root'
  })
  export class GetProductsQueryService extends Apollo.Query<GetProducts, GetProductsVariables> {
    override document = GetProductsDocument;
    
    constructor(apollo: Apollo.Apollo) {
      super(apollo);
    }
  }
export const DeliverDocument = gql`
    mutation deliver($productId: String!) {
  response: deliver(productInTransitId: $productId) {
    id
    actionsAllowed
  }
}
    `;

  @Injectable({
    providedIn: 'root'
  })
  export class DeliverMutationService extends Apollo.Mutation<Deliver, DeliverVariables> {
    override document = DeliverDocument;
    
    constructor(apollo: Apollo.Apollo) {
      super(apollo);
    }
  }
export const CancelDocument = gql`
    mutation cancel($productId: String!) {
  response: cancel(productInTransitId: $productId) {
    id
    actionsAllowed
  }
}
    `;

  @Injectable({
    providedIn: 'root'
  })
  export class CancelMutationService extends Apollo.Mutation<Cancel, CancelVariables> {
    override document = CancelDocument;
    
    constructor(apollo: Apollo.Apollo) {
      super(apollo);
    }
  }
export const RestockDocument = gql`
    mutation restock($productId: String!) {
  response: restock(productId: $productId) {
    id
    actionsAllowed
  }
}
    `;

  @Injectable({
    providedIn: 'root'
  })
  export class RestockMutationService extends Apollo.Mutation<Restock, RestockVariables> {
    override document = RestockDocument;
    
    constructor(apollo: Apollo.Apollo) {
      super(apollo);
    }
  }