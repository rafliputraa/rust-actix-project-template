
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

use crate::server::server;

mod server;
mod database;
mod config;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server().await
}
