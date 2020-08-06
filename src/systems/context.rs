use diesel::pg::PgConnection;

use r2d2::{Pool, PooledConnection as PooledConn};
use r2d2_diesel::ConnectionManager as ConnMgrDsl;
use r2d2_redis::RedisConnectionManager as ConnMgrRedis;

use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request, State,
};

use std::cell::{RefCell, RefMut};
use std::env;

// Some type aliases that make the code cleaner
type ConnMgrPg = ConnMgrDsl<PgConnection>;
type PoolPg = Pool<ConnMgrPg>;
type PoolRedis = Pool<ConnMgrRedis>;
type PooledPg = PooledConn<ConnMgrPg>;
type PooledRedis = PooledConn<ConnMgrRedis>;

pub fn pg_init() -> PoolPg {
    let manager = ConnMgrPg::new(redis_url());
    PoolPg::new(manager).expect("Failed to initialize PostgreSQL database pool")
}

pub fn a() {}

pub fn redis_init() -> PoolRedis {
    let manager =
        ConnMgrRedis::new(pg_url()).expect("Failed to initialize Redis connection manager");
    PoolRedis::new(manager).expect("Failed to initialize Redis database pool")
}

fn pg_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

fn redis_url() -> String {
    env::var("REDIS_URL").expect("REDIS_URL must be set")
}

pub struct Context {
    pg: PooledPg,
    redis: RefCell<PooledRedis>,
}

impl Context {
    pub fn redis(&self) -> RefMut<PooledRedis> {
        self.redis.borrow_mut()
    }

    pub fn pg(&self) -> &PooledPg {
        &self.pg
    }

    fn new(pg: PooledPg, redis: PooledRedis) -> Self {
        Self {
            pg,
            redis: RefCell::new(redis),
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Context {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let pg_pool = request.guard::<State<PoolPg>>()?;
        let redis_pool = request.guard::<State<PoolRedis>>()?;

        let pg_conn = match pg_pool.get() {
            Ok(pg_conn) => pg_conn,
            Err(error) => return Outcome::Failure((Status::ServiceUnavailable, ())),
        };

        let redis_conn = match redis_pool.get() {
            Ok(redis_conn) => redis_conn,
            Err(error) => return Outcome::Failure((Status::ServiceUnavailable, ())),
        };

        Outcome::Success(Context::new(pg_conn, redis_conn))
    }
}

impl juniper::Context for Context {}
