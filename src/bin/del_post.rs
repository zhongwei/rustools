extern crate rustools;

use std::env::args;
use rustools::db;

fn main() {
    let conn = db::establish_connection();

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let num_deleted = db::delete_post(&conn, &pattern);
    println!("Deleted {} posts", num_deleted);
}

