use std::sync::Arc;
use tokio::sync::Mutex;
use  rusqlite::Connection;
use crate::schemas::*;

pub type Conn = Arc<Mutex<Connection>>;

pub fn start_db() -> Result<Connection, rusqlite::Error> {

    let conn = Connection::open_in_memory()?;

    conn.execute(INIT_EMPLOYEE, [])?;
    conn.execute(INIT_MONTHLY, [])?;
    conn.execute(INIT_RATES, [])?;
    conn.execute(INIT_SLIP, [])?;

    Ok(conn)
}
