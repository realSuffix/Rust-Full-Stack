use warp;

use crate::models::post::{
    PostList,
    Post,
};
use crate::db_connection::{ establish_connection };

pub async fn list() -> Result<impl warp::Reply, warp::Rejection> {
    let conn = establish_connection();
    let list_of_posts = PostList::list(&conn);
    println!("{:#?}", &list_of_posts);

    Ok(warp::reply::json(&list_of_posts))
}
