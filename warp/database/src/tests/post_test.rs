use warp::Filter;

use crate::{
    handlers::post_handler,
    routes::post_route,
};

// $cargo test -- --nocapture if you want to use println! etc.

// or test just one function each time.
// For example, $cargo test list_post and it passes.

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    // Refer to curl commands in main.rs
    #[tokio::test]
    async fn list_post() {
        let post = post_route::list()
            .and_then(post_handler::list);

        let res = warp::test::request()
            .method("GET")
            .path("/api/post/v1") // 1. [Client] - Define request(path with datas) until this
            .reply(&post) // 2. [Server] - How will you respond to it? With what?
            .await;

        assert_eq!(res.status(), 200, "Should return 200 OK.");
        println!("{:#?}", res.body());
    }
}
