use super::db::users;
use diesel_citext::types::CiString;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: CiString,
    pub password_hash: String,
    pub email: CiString,
    pub email_confirmed: bool,
    pub admin: bool,
    pub team_id: Option<i32>,
}

#[derive(juniper::GraphQLInputObject)]
pub struct InputUser {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(juniper::GraphQLInputObject)]
pub struct InputCreds {
    pub username: Option<String>,
    pub password: String,
    pub email: Option<String>,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: CiString,
    pub password_hash: String,
    pub email: CiString,
    pub email_confirmed: bool,
    pub admin: bool,
    pub team_id: Option<i32>,
}
