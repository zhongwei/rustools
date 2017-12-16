use std::io;
use std::path::{Path, PathBuf};

use rocket;
use rocket::response::NamedFile;

pub fn start() {
    rocket::ignite().mount("/", routes![index, files]).launch();
}

#[get("/")]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}


