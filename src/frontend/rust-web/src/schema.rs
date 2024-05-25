#[cynic::schema("example")]
mod schema {}

#[derive(cynic::QueryVariables, Debug)]
pub struct GetProductsVariables<'a> {
    pub after: Option<&'a str>,
    pub before: Option<&'a str>,
    pub first: Option<i32>,
    pub last: Option<i32>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "QueryType", variables = "GetProductsVariables")]
pub struct GetProducts {
    #[arguments(first: $first, after: $after, last: $last, before: $before)]
    pub products_relay: Option<ProductConnection>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct ProductConnection {
    pub edges: Option<Vec<Option<ProductEdge>>>,
    pub total_count: Option<i32>,
    pub page_info: PageInfo,
    pub items: Option<Vec<Option<Product>>>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct ProductEdge {
    pub node: Option<Product>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: String,
    pub in_stock: i32,
    pub actions_allowed: Option<Vec<Option<String>>>,
    #[cynic(rename = "categoriesWithoutBatchAsync")]
    pub selected_categories: Option<Vec<Option<Category2>>>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct PageInfo {
    pub has_next_page: bool,
    pub has_previous_page: bool,
    pub start_cursor: Option<String>,
    pub end_cursor: Option<String>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct GetProductVariables<'a> {
    pub product_id: &'a str,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "QueryType", variables = "GetProductVariables")]
pub struct GetProduct {
    #[arguments(productId: $product_id)]
    pub product: Option<Product>,
    pub categories: Option<Vec<Option<Category>>>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Category {
    pub id: String,
    pub name: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Category")]
pub struct Category2 {
    pub id: String,
}

