use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::Mutex;
use rusqlite::Connection;
use warp::{self, http::StatusCode};
use handlebars::Handlebars;

use crate::models::{Employee, Monthly, Rates, Slip};
use crate::db::Conn;

// GET /register_employee/<first_name>/<last_name>/<home_address>/<level>/<department>/<badge_number>
pub async fn register_employee(
    first_name: String,
    last_name: String,
    home_address: String,
    level: i32,
    department: String,
    badge_number: i32,
    conn: Conn
) -> Result<impl warp::Reply, Infallible> {
    let conn = conn.lock().await;
    match Employee::new(&conn, &first_name.as_str(), &last_name.as_str(), &home_address.as_str(), level, &department.as_str(), badge_number) {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Ok(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

// GET /register_rates/<year>/<level>/<work_rate>/<overtime_rate>/<taxes_percent>/<overtime_taxes>
pub async fn register_rates(
    year: i32,
    level: i32,
    work_rate: f32,
    overtime_rate: f32,
    taxes_percent: f32,
    overtime_taxes: f32,
    conn: Conn
) -> Result<impl warp::Reply, Infallible> {
    let conn = conn.lock().await;
    match Rates::new(&conn, year, level, work_rate, overtime_rate, taxes_percent, overtime_taxes) {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Ok(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

// GET /register_month/<month>/<year>/<badge_number>/<work_hours>/<overtime_hours>
pub async fn register_month(
    month: i32,
    year: i32,
    badge_number: i32,
    work_hours: i32,
    overtime_hours: i32,
    conn: Conn
) -> Result<impl warp::Reply, Infallible> {
    let conn = conn.lock().await;
    match Monthly::new(&conn, month, year, badge_number, work_hours, overtime_hours) {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Ok(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

// GET /register_slip/<badge_number>/<month>/<year>
pub async fn register_slip(
    badge_number: i32,
    month: i32,
    year: i32,
    conn: Conn
) -> Result<impl warp::Reply, Infallible> {
    let conn = conn.lock().await;
    match Slip::new(&conn, badge_number, month, year) {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Ok(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

// GET /slip/<badge_number>/<month>/<year>
pub async fn get_slip(slip: i32, month:i32, year: i32, conn: Conn) -> Result<impl warp::Reply, Infallible> {
    let conn = conn.lock().await;

    // Register template
    let mut handlebars = Handlebars::new();
    let source = "Slip for {{badge_number}}

Employee badge n. {{badge_number}},
worked {{work_hours}} hours,
plus {{overtime_hours}} overtime hours.

Summary payments

Gross payment {{gross_payment}}
Taxes due: {{taxes_due}}
Net Payment: {{net_payment}}";

    handlebars.register_template_string("t1", source).unwrap();
    let hb = Arc::new(handlebars);

    let data = Slip::get(&conn, slip, month, year).unwrap();
    Ok(warp::reply::html(
        hb.render("t1", &data)
        .unwrap_or_else(|err| err.to_string())
    ))
}

// GET /
pub async fn get_home(conn: Conn) -> Result<impl warp::Reply, Infallible> {
    let conn = conn.lock().await;

    // Register template
    let mut handlebars = Handlebars::new();
    let source = "
    <h1>Welcome to our server</h1>
    <p>Please, use the Terminal window to poll the server from the command line<br/>
    or the following box to poll the server live on this page.</p>

    <label>127.0.0.1:3030/</label><input type='text' id='pollapi' size='50'><button onclick='getInputValue()'>Poll</button>
	<pre id='response'></pre>
	<pre id='result'></pre>
	<p>Sample list of commands to have a bare minimum slip</p>
	<ul>
	<li>register_rates/2022/2/9.0/11.0/0.15/0.12</li>
	<li>register_employee/John/Doe/Somecity/2/IT/8691</li>
	<li>register_month/1/2022/8691/158/3</li>
	<li>register_slip/8691/1/2022</li>
	<li>slip/8691/1/2022</li>
    <script src='res/fetchapi.js'></script>
    ";
    handlebars.register_template_string("t2", source).unwrap();
    let hb = Arc::new(handlebars);

    let data = "".to_string();
    Ok(warp::reply::html(
        hb.render("t2", &data)
        .unwrap_or_else(|err| err.to_string())
    ))
}
