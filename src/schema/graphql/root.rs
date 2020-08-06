use crate::systems::context::Context;

pub struct Query;

#[juniper::object(
    Context = Context,
)]
impl Query {
    #[graphql(description = "Get the current SemVer-compliant API version")]
    pub fn api_version() -> &'static str {
        "1.0.0"
    }
}

pub struct Mutation;

#[juniper::object(
    Context = Context,
)]
impl Mutation {}

pub type Schema = juniper::RootNode<'static, Query, Mutation>;
