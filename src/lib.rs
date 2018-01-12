#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]
extern crate rocket;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate rocksdb;


pub mod http;
pub mod cli;
pub mod db;
pub mod kvdb;
