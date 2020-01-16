// 1. https://doc.rust-lang.org/rust-by-example/macros.html, https://danielkeep.github.io/practical-intro-to-macros.html
// 2. Read some blog posts from https://www.google.com/search?&q=rust+macro+example
// 3. Refer to https://github.com/dtolnay/quote and https://doc.rust-lang.org/proc_macro/
// 4. Clone and read the code of https://github.com/mjkillough/factori 

use std::collections::HashMap;

macro_rules! map {
    // => or, :
    ($( $key:expr => $value:expr ),*) => {{
        let mut hm = HashMap::new();
        $( hm.insert($key, $value); )*
        hm
    }}
}

fn main() {
    let steadylearner = map!(
       "name" => "Steadylearner",
       "language" => "Rust, Python, JavaScript, (Golang)",
       "website" => "https://www.steadylearner.com",
       "blog" => "https://www.steadylearner.com/blog/search/Rust",
       "portfolio" => "https://github.com/steadylearner/Rust-Full-Stack",
       "wanting_to_work_with_rust" => "true",
       "but_there_is_no_rust_jobs" => "true at least here"
    );
    println!("{:#?}", &steadylearner);
}
