pub mod schema;
pub mod models;

use std::env;
use dotenv::dotenv;
use diesel::prelude::*;
use diesel::{insert_into, update, delete};

use self::models::{Post, NewPost};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn qry_posts() {
    use self::schema::posts::dsl::*;

    let conn = establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&conn)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}

pub fn create_post(conn: &PgConnection, title: &str, body: &str) -> Post {
    use self::schema::posts;

    let new_post = NewPost {
        title: title,
        body: body,
    };

    insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn update_post(conn: &PgConnection, id: i32) -> Post {
    use db::schema::posts::dsl::{posts, published};

    return update(posts.find(id))
        .set(published.eq(true))
        .get_result::<Post>(conn)
        .expect(&format!("Unable to find post {}", id));
}

pub fn delete_post(conn: &PgConnection, pattern: &str) -> usize {
    use db::schema::posts::dsl::*;

    return delete(posts.filter(title.like(&pattern)))
        .execute(conn)
        .expect("Error deleting posts");
}
