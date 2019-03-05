use actix::prelude::{Addr, SyncArbiter};
use actix_web::{
    http::{header, StatusCode},
    middleware::Logger,
    App,
    HttpRequest,
};
use std::env;

use crate::db::DbExecutor;
use diesel::{
    r2d2::{
        self,
        ConnectionManager,
    },
    PgConnection
};

const NUM_DB_THREADS: usize = 4;

pub struct AppState {
    pub db: Addr<DbExecutor>,
}

fn index(_req: &HttpRequest<AppState>) -> &'static str {
    "Hello world!"
}

pub fn create() -> App<AppState> {

    let jwt_secret_key = env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");
    let frontend_origin = env::var("FRONTEND_ORIGIN").ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_manager = ConnectionManager::<PgConnection>::new(database_url);
    let database_pool = r2d2::Pool::builder()
        .build(database_manager)
        .expect("Failed to create pool.");

    let database_address = SyncArbiter::start(NUM_DB_THREADS, move || DbExecutor(database_pool.clone()));

    let state = AppState {
        db: database_address.clone(),
    };

    App::with_state(state)
        .middleware(Logger::default())
}
