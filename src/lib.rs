#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]
extern crate rocket;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_infer_schema;
extern crate dotenv;


pub mod http;
pub mod cli;
pub mod db;
