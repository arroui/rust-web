// Init database
pub const INIT_EMPLOYEE: &str = "
CREATE TABLE IF NOT EXISTS employee (
    id  INTEGER PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name  TEXT NOT NULL,
    home_address TEXT NOT NULL,
    level INTEGER NOT NULL,
    department TEXT NOT NULL,
    badge_number INTEGER NOT NULL
)";

pub const INIT_MONTHLY: &str = "
CREATE TABLE IF NOT EXISTS monthly (
    id  INTEGER PRIMARY KEY,
    month INTEGER NOT NULL,
    year INTEGER NOT NULL,
    badge_number INTEGER NOT NULL,
    work_hours  INTEGER NOT NULL,
    overtime_hours INTEGER NOT NULL
)";

pub const INIT_RATES: &str = "
CREATE TABLE IF NOT EXISTS rates (
    id  INTEGER PRIMARY KEY,
    year INTEGER NOT NULL,
    level INTEGER NOT NULL,
    work_rate  FLOAT NOT NULL,
    overtime_rate FLOAT NOT NULL,
    taxes_percent FLOAT NOT NULL,
    overtime_taxes FLOAT NOT NULL
)";

pub const INIT_SLIP: &str = "
CREATE TABLE IF NOT EXISTS slip (
    id  INTEGER PRIMARY KEY,
    month INTEGER NOT NULL,
    year INTEGER NOT NULL,
    badge_number INTEGER NOT NULL,
    work_hours  INTEGER NOT NULL,
    overtime_hours INTEGER NOT NULL,
    gross_payment FLOAT NOT NULL,
    taxes_due FLOAT NOT NULL,
    net_payment FLOAT NOT NULL
)";
