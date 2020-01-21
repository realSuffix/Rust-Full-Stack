// We will make Warp version of
// https://github.com/steadylearner/Rust-Full-Stack/tree/master/actix/src/database
// You should refer to
// https://github.com/steadylearner/Rust-Full-Stack/tree/master/microservices_with_docker/warp_client/src
use warp::{self, Filter};

use console::Style;

mod routes;
mod handlers; // This is the payload of this framework.
use self::{
    routes::{
        post_route,
    },
    handlers::{
        post_handler
    },

};

#[macro_use]
extern crate diesel;

mod schema;
mod models;
mod db_connection;

// It will only work with $cargo test
// For example, $cargo test list_post -- --nocapture
#[cfg(test)] mod tests;

// Make get_post, create_post, delete_post, update_post
// While you refer to the links given above.

// Use some test driven development with files at tests/ and CURL commands.
// If you still can't make it work, don't modulze the Warp API and test them here first.
#[tokio::main]
async fn main() {
    let target: String = "0.0.0.0:8000".parse().unwrap();
    let blue = Style::new()
        .blue();

    // curl 0.0.0.0:8000/repeat/www.steadylearner.com
    // Use this to debug and verify API chaining work or not.
    // let repeat = post_route::repeat()
    //     .and_then(post_handler::repeat);

    // curl 0.0.0.0:8000/api/post/v1
    let list_posts = post_route::list()
        .and_then(post_handler::list);

    // curl 0.0.0.0:8000/api/post/v1/1
    let get_user = post_route::get()
        .and_then(post_handler::get);

    let post_api = list_posts
        .or(get_user);

    let end = post_api.with(warp::log("post_api"));

    println!("\nRust Warp Server ready at {}", blue.apply_to(&target));
    println!("Use $curl http://0.0.0.0:8000/api/post/v1 to test the end point.");

    warp::serve(end).run(([0, 0, 0, 0], 8000)).await;
}
