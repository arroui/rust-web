use std::sync::Arc;
use rusqlite::{params, Result};
use serde::Serialize;

#[derive(Debug)]
pub struct Employee {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub home_address: String,
    pub level: i32,
    pub department: String,
    pub badge_number: i32,
}

#[derive(Debug)]
pub struct Monthly {
    pub id: i32,
    pub month: i32,
    pub year: i32,
    pub badge_number: i32,
    pub work_hours: i32,
    pub overtime_hours: i32,
}

#[derive(Debug)]
pub struct Rates {
    pub id: i32,
    pub year: i32,
    pub level: i32,
    pub work_rate: f32,
    pub overtime_rate: f32,
    pub taxes_percent: f32,
    pub overtime_taxes: f32,
}

#[derive(Debug, Serialize, Clone)]
pub struct Slip {
    pub id: i32,
    pub month: i32,
    pub year: i32,
    pub badge_number: i32,
    pub work_hours: i32,
    pub overtime_hours: i32,
    pub gross_payment: f32,
    pub taxes_due: f32,
    pub net_payment: f32,
}


impl Employee {
    pub fn new(
        conn: &rusqlite::Connection,
        first_name: &str,
        last_name: &str,
        home_address: &str,
        level: i32,
        department: &str,
        badge_number: i32
    ) -> Result<usize, rusqlite::Error> {
        conn.execute(
        "INSERT INTO employee (first_name, last_name, home_address, level, department, badge_number)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![first_name, last_name, home_address, level, department, badge_number],
        )
    }

    pub fn get(
        conn: &rusqlite::Connection,
        badge_number: i32
    ) -> Result<Employee, rusqlite::Error> {
        conn.query_row(
            "SELECT id, first_name, last_name, home_address, level, department, badge_number FROM employee
            WHERE badge_number = ?1",
            [badge_number],
            |row| {
            Ok(Employee {
                id: row.get(0)?,
                first_name: row.get(1)?,
                last_name: row.get(2)?,
                home_address: row.get(3)?,
                level: row.get(4)?,
                department: row.get(5)?,
                badge_number: row.get(6)?,
            })
    })
    }
}

impl Monthly {
    pub fn new(
        conn: &rusqlite::Connection,
        month: i32,
        year: i32,
        badge_number: i32,
        work_hours: i32,
        overtime_hours: i32
    ) -> Result<usize, rusqlite::Error> {
        conn.execute(
        "INSERT INTO monthly (month, year, badge_number, work_hours, overtime_hours)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![month, year, badge_number, work_hours, overtime_hours],
        )
    }

    pub fn get(
        conn: &rusqlite::Connection,
        badge_number: i32,
        month: i32,
        year: i32
    ) -> Result<Monthly, rusqlite::Error> {
        conn.query_row(
            "SELECT id, month, year, badge_number, work_hours, overtime_hours FROM monthly
            WHERE badge_number = ?1 AND month = ?2 AND year = ?3",
            [badge_number, month, year],
            |row| {
            Ok(Monthly {
                id: row.get(0)?,
                month: row.get(1)?,
                year: row.get(2)?,
                badge_number: row.get(3)?,
                work_hours: row.get(4)?,
                overtime_hours: row.get(5)?,
            })
        })
    }
}

impl Rates {
    pub fn new(
        conn: &rusqlite::Connection,
        year: i32,
        level: i32,
        work_rate: f32,
        overtime_rate: f32,
        taxes_percent: f32,
        overtime_taxes: f32
    ) -> Result<usize, rusqlite::Error> {
        conn.execute(
        "INSERT INTO rates (year, level, work_rate, overtime_rate, taxes_percent, overtime_taxes)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![year, level, work_rate, overtime_rate, taxes_percent, overtime_taxes],
        )
    }

    pub fn get(
        conn: &rusqlite::Connection,
        year: i32,
        level: i32
    ) -> Result<Rates, rusqlite::Error> {
        conn.query_row(
            "SELECT id, year, level, work_rate, overtime_rate, taxes_percent, overtime_taxes FROM rates
            WHERE year = ?1 AND level = ?2",
            [year, level],
            |row| {
            Ok(Rates {
                id: row.get(0)?,
                year: row.get(1)?,
                level: row.get(2)?,
                work_rate: row.get(3)?,
                overtime_rate: row.get(4)?,
                taxes_percent: row.get(5)?,
                overtime_taxes: row.get(6)?,
            })
        })
    }
}

impl Slip {
    pub fn new(
        conn: &rusqlite::Connection,
        badge_number: i32,
        month: i32,
        year: i32
    ) -> Result<usize, rusqlite::Error> {
        // Get needed info
        let employee = Employee::get(&conn, badge_number)?;
        let this_month = Monthly::get(&conn, badge_number, month, year)?;
        let rates = Rates::get(&conn, year, employee.level)?;

        let work_hours = this_month.work_hours;
        let overtime_hours = this_month.overtime_hours;

        // Calculate the rest
        let work_hrs_payment = this_month.work_hours as f32 * rates.work_rate;
        let overtime_hrs_payment = this_month.overtime_hours as f32 * rates.overtime_rate;
        let gross_payment = work_hrs_payment + overtime_hrs_payment;

        let work_taxes = work_hrs_payment * rates.taxes_percent;
        let overtime_taxes = overtime_hrs_payment * rates.overtime_taxes;
        let taxes_due = work_taxes + overtime_taxes;

        let net_payment = gross_payment - taxes_due;

        conn.execute(
        "INSERT INTO slip (month, year, badge_number, work_hours, overtime_hours, gross_payment, taxes_due, net_payment)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![month, year, badge_number, work_hours, overtime_hours, gross_payment, taxes_due, net_payment],
        )
    }

    pub fn get(
        conn: &rusqlite::Connection,
        badge_number: i32,
        month: i32,
        year: i32,
    ) -> Result<Slip, rusqlite::Error> {
        conn.query_row(
            "SELECT id, month, year, badge_number, work_hours, overtime_hours, gross_payment, taxes_due, net_payment FROM slip
            WHERE badge_number = ?1 AND month = ?2 AND year = ?3",
            [badge_number, month, year],
            |row| {
            Ok(Slip {
                id: row.get(0)?,
                month: row.get(1)?,
                year: row.get(2)?,
                badge_number: row.get(3)?,
                work_hours: row.get(4)?,
                overtime_hours: row.get(5)?,
                gross_payment: row.get(6)?,
                taxes_due: row.get(7)?,
                net_payment: row.get(8)?,
            })
        })
    }
}
