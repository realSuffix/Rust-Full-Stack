use warp;

use crate::models::post::{
    PostList,
    Post,
};
use crate::db_connection::{ establish_connection };

// Use this to debug and verify API chaining work or not.
// pub async fn repeat(input: String) -> Result<impl warp::Reply, warp::Rejection> {
//     println!("{:#?}", &input);
//     Ok(warp::reply::html(input))
// }

pub async fn list() -> Result<impl warp::Reply, warp::Rejection> {
    let conn = establish_connection();
    let list_of_posts = PostList::list(&conn);
    println!("{:#?}", &list_of_posts);

    Ok(warp::reply::json(&list_of_posts))
}

pub async fn get(id: String) -> Result<impl warp::Reply, warp::Rejection> {
    // let conn = establish_connection();
    // let list_of_posts = PostList::list(&conn);
    // println!("{:#?}", &list_of_posts);

    Ok(warp::reply::html(format!("get {}", &id)))
}
