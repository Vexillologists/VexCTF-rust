use rocket::{config::LoggingLevel, response::content, State};

use crate::schema::graphql::{Mutation, Query, Schema};

use super::pooling::{self, DbConn};

#[rocket::post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: DbConn,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[rocket::get("/graphql")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::playground_source("/graphql")
}

pub fn init() {
    let mut conf = rocket::config::Config::active().expect("Failed to get Rocket configuration");
    conf.set_log_level(LoggingLevel::Critical);
    rocket::custom(conf)
        .manage(pooling::init_pool_pg())
        .manage(pooling::init_pool_redis())
        .manage(Schema::new(Query, Mutation))
        .mount(
            "/",
            rocket::routes![graphiql, post_graphql_handler],
        )
        .launch();
}
