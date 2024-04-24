
// source from https://github.com/graphql-rust/juniper/issues/1051
use juniper::GraphQLObject;

#[derive(Debug)]
pub struct Connection<Node, Cursor> {
    pub edges: Vec<Edge<Node, Cursor>>,
    pub total_count: i32,
    pub has_next_page: bool,
    pub has_previous_page: bool,
}

impl<Node, Cursor> Connection<Node, Cursor> {
    pub fn _edges(&self) -> &[Edge<Node, Cursor>] {
        self.edges.as_slice()
    }

    pub fn _nodes(&self) -> Vec<&Node> {
        self.edges.iter().map(|edge| &edge.node).collect()
    }

    pub fn _page_info(&self) -> PageInfo<'_>
    where
        Cursor: AsRef<str>,
    {
        PageInfo {
            end_cursor: self.edges.last().map(|edge| edge.cursor.as_ref()),
            has_next_page: self.has_next_page,
            start_cursor: self.edges.first().map(|edge| edge.cursor.as_ref()),
            has_previous_page: self.has_previous_page,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Edge<Node, Cursor> {
    pub node: Node,
    pub cursor: Cursor,
}


#[derive(Debug, GraphQLObject)]
pub struct PageInfo<'a> {
    pub end_cursor: Option<&'a str>,
    pub has_next_page: bool,
    pub start_cursor: Option<&'a str>,
    pub has_previous_page: bool,
}