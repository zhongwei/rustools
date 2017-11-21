use rocket;

pub fn start() {
    rocket::ignite().mount("/", routes![index]).launch();
}

#[get("/")]
pub fn index() -> &'static str {
    "Hello, 钟逸鸣!"
}

