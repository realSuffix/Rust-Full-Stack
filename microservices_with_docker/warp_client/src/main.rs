extern crate pretty_env_logger;
#[macro_use] extern crate log;
// This is to test files in benches/
extern crate reqwest;

extern crate tonic;
mod user {
    tonic::include_proto!("user");
}

extern crate argon2;
extern crate rand;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate warp;
use warp::{
    Filter,
};

extern crate console;
use console::Style;

// This is where all your custom modules(folders) will be.
mod crypto;
mod models;
mod routes;
// Files in handlers/ are what implements "Result<impl warp::Reply, warp::Rejection>"
// It will be similar to controllers in Express and you will edit it most of time with models/
mod handlers;
use self::{
    routes::{
        user_route,
    },
    handlers::{
        user_handler
    },

};

// // It will only work with $cargo bench
// #[cfg(bench)] mod benches;

// It will only work with $cargo test
#[cfg(test)] mod tests;

#[tokio::main]
async fn main() {
    let target: String = "0.0.0.0:8000".parse().unwrap();
    let blue = Style::new()
        .blue();

    let list_users = user_route::list()
        .and_then(user_handler::list);

    let get_user = user_route::get()
        .and_then(user_handler::get);

    let create_user = user_route::create()
        .and_then(user_handler::create);

    let update_user = user_route::update()
        .and_then(user_handler::update);

    let delete_user = user_route::delete()
        .and_then(user_handler::delete);

    let api = list_users
        .or(get_user)
        .or(create_user)
        .or(update_user)
        .or(delete_user);

    let routes = api.with(warp::log("api"));

    println!("\nRust Warp Server ready at {}", blue.apply_to(&target));
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}