use crate::systems::pooling::DbConn;

pub struct Query;

#[juniper::object(
    Context = DbConn,
)]
impl Query {
    #[graphql(description = "Get the current SemVer-compliant API version")]
    pub fn api_version() -> &'static str {
        "1.0.0"
    }
}

pub struct Mutation;

#[juniper::object(
    Context = DbConn,
)]
impl Mutation {}

pub type Schema = juniper::RootNode<'static, Query, Mutation>;
