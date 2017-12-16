extern crate rustools;

use std::env::args;
use rustools::db;

fn main() {
    let conn = db::establish_connection();

    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");

    let post = db::update_post(&conn, id);
    println!("Published post {}", post.title);
}

