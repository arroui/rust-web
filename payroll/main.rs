#![allow(unreachable_code)]
#![allow(unused)]

use std::sync::Arc;
use tokio::sync::Mutex;
use handlebars::Handlebars;
use warp::Filter;

mod schemas;
mod db;
mod models;
mod services;
mod routes;

use models::{Employee, Monthly, Rates, Slip};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // SQLite connection
    let conn = db::start_db()?;
    let conn = Arc::new(Mutex::new(conn));
    let routes = routes::create_routes(conn);

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;

    Ok(())
}




