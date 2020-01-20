use warp::{
    filters::BoxedFilter,
    path,
    Filter,
};

// It is equal to use it in main.rs file.
// let get_user = path!("api" / "post" / "v1")
//     .and(warp::path::param::<String>())

fn path_prefix() -> BoxedFilter<()> {
    path!("api" / "post" / "v1")
        .boxed()
}

pub fn list() -> BoxedFilter<()> {
    warp::get()
        .and(path_prefix())
        .and(warp::path::end())
        .boxed()
}
