mod static_kv;
use static_kv::{static_kv::static_kv, read_func::read_func};
fn main() {
    static_kv();
    read_func();
    println!("Hello, world!");
}
