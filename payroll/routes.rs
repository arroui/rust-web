use std::convert::Infallible;
use warp::{self, Filter};

use crate::db::Conn;
use crate::services;

// GET /register_employee/<first_name>/<last_name>/<home_address>/<level>/<department>/<badge_number>
fn register_employee_rt(
    conn: Conn,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("register_employee" / String / String / String / i32 / String / i32)
    .and(warp::get())
    .and(warp::any().map(move || conn.clone()))
    .and_then(services::register_employee)
}

// GET /register_rates/<year>/<level>/<work_rate>/<overtime_rate>/<taxes_percent>/<overtime_taxes>
fn register_rates_rt(
    conn: Conn,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("register_rates" / i32 / i32 / f32 / f32 / f32 / f32)
    .and(warp::get())
    .and(warp::any().map(move || conn.clone()))
    .and_then(services::register_rates)
}

// GET /register_month/<month>/<year>/<badge_number>/<work_hours>/<overtime_hours>
fn register_month_rt(
    conn: Conn,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("register_month" / i32 / i32 / i32 / i32 / i32)
    .and(warp::get())
    .and(warp::any().map(move || conn.clone()))
    .and_then(services::register_month)
}

// GET /register_slip/<badge_number>/<month>/<year>
fn register_slip_rt(
    conn: Conn,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("register_slip" / i32 / i32 / i32)
    .and(warp::get())
    .and(warp::any().map(move || conn.clone()))
    .and_then(services::register_slip)
}

// GET /slip/<badge_number>/<month>/<year>
fn get_slip_rt(
    conn: Conn,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("slip" / i32 / i32 / i32)
    .and(warp::get())
    .and(warp::any().map(move || conn.clone()))
    .and_then(services::get_slip)
}

// GET /
fn get_home_rt(
    conn: Conn,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end()
    .and(warp::get())
    .and(warp::any().map(move || conn.clone()))
    .and_then(services::get_home)
}

// GET static files in res/
fn get_static_rt(
    conn: Conn,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("res")
        .and(warp::fs::dir("/usercode/res"))
}

// Routes
pub fn create_routes(
    conn: Conn,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_slip_rt(conn.clone())
        .or(register_employee_rt(conn.clone()))
        .or(register_rates_rt(conn.clone()))
        .or(register_month_rt(conn.clone()))
        .or(register_slip_rt(conn.clone()))
        .or(get_static_rt(conn.clone()))
        .or(get_home_rt(conn.clone()))
}

