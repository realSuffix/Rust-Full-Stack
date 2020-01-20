use warp;

use crate::models::post::{
    PostList,
    NewPost,
    Post,
};
use crate::db_connection::{ establish_connection };

pub async fn list() -> Result<impl warp::Reply, warp::Rejection> {
    let conn = establish_connection();
    let list_of_posts = PostList::list(&conn);
    println!("{:#?}", &list_of_posts);

    Ok(warp::reply::json(&list_of_posts))
}

pub async fn get(post_id: i32) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = establish_connection();
    let a_post = Post::get(&post_id, &conn);
    println!("{:#?}", &a_post);

    let reply = match a_post {
        Ok(post) => {
            println!("{:#?}", &post);
            post
        },
        Err(e) => {
            // https://docs.rs/warp/0.1.20/warp/reject/fn.custom.html
            println!("{:#?}", e);
            // Temporay solution to make the project grow first.
            // You may build custom error Struct if necessary.
            // return Err(warp::reject::custom(UserError))
            return Err(warp::reject::not_found())
        }
    };

    Ok(warp::reply::json(&reply))
}

// pub async fn get(title: String) -> Result<impl warp::Reply, warp::Rejection> {
//     // let reply = format!("Hello, {}!", name);
//     // println!("{}", &reply);
//     Ok(warp::reply::html(reply))
// }
