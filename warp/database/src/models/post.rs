// Read them.
// 1. diesel.rs/guides/getting-started/
// 2. https://github.com/steadylearner/Rust-Full-Stack/blob/master/actix/src/database/models/product.rs

// $echo DATABASE_URL=postgres://postgres:postgres@localhost/warp > .env

// $diesel setup
// $diesel migration create_post
// Move to migrations/ folder.
// CREATE TABLE posts (
//   id SERIAL PRIMARY KEY,
//   title VARCHAR NOT NULL,
//   body TEXT NOT NULL
// )
// $diesel migration run

use crate::schema::posts;

use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::PgConnection;
use crate::schema::posts::dsl;
use crate::schema::posts::dsl::*;

use serde_derive::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
}

impl Post {
    pub fn find(post_id: &i32, connection: &PgConnection) -> Result<Post, diesel::result::Error> {
        posts::table.find(post_id).first(connection)
    }

    // pub fn destroy(product_id: &i32, connection: &PgConnection) -> Result<(), diesel::result::Error> {
    //     diesel::delete(dsl::products.find(product_id)).execute(connection)?;
    //     Ok(())
    // }

    // pub fn update(product_id: &i32, new_product: &NewProduct, connection: &PgConnection) -> Result<(), diesel::result::Error> {

    //     diesel::update(dsl::products.find(product_id))
    //         .set(new_product)
    //         .execute(connection)?;
    //     Ok(())
    // }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostList(pub Vec<Post>);

impl PostList {
    pub fn list(connection: &PgConnection) -> Self {
        let result = posts
            .limit(10)
            .load::<Post>(connection)
            .expect("Error loading posts");

        PostList(result)
    }
}

