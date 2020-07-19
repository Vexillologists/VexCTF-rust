use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager as ConnMgrPg;
use r2d2_redis::RedisConnectionManager as ConnMgrRedis;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use std::cell::{RefCell, RefMut};
use std::env;

type PoolPg = r2d2::Pool<ConnMgrPg<PgConnection>>;
type PoolRedis = r2d2::Pool<ConnMgrRedis>;

pub fn init_pool_pg() -> PoolPg {
    let manager = ConnMgrPg::<PgConnection>::new(database_url_pg());
    PoolPg::new(manager).expect("pg pool")
}

pub fn init_pool_redis() -> PoolRedis {
    let manager = ConnMgrRedis::new(database_url_redis()).expect("redis mgr");
    PoolRedis::new(manager).expect("redis pool")
}

fn database_url_pg() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

fn database_url_redis() -> String {
    env::var("REDIS_URL").expect("REDIS_URL must be set")
}

pub struct DbConn {
    pub pg: r2d2::PooledConnection<ConnMgrPg<PgConnection>>,
    pub redis: RefCell<r2d2::PooledConnection<ConnMgrRedis>>,
}

impl DbConn {
    pub fn redis(&self) -> RefMut<r2d2::PooledConnection<ConnMgrRedis>> {
        self.redis.borrow_mut()
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, Self::Error> {
        let pool_pg = request.guard::<State<PoolPg>>()?;
        let pool_redis = request.guard::<State<PoolRedis>>()?;
        match pool_pg.get() {
            Ok(conn_pg) => match pool_redis.get() {
                Ok(conn_redis) => Outcome::Success(DbConn {
                    pg: conn_pg,
                    redis: RefCell::new(conn_redis),
                }),
                Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
            },
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl AsRef<PgConnection> for DbConn {
    fn as_ref(&self) -> &PgConnection {
        &*self.pg
    }
}

impl juniper::Context for DbConn {}
