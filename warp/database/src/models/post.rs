// Refer to diesel.rs/guides/getting-started/

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

