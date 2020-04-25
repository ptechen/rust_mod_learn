use crate::static_kv::mod_test;

pub mod static_kv {
    pub fn mod_test() {
        println!("test")
    }
}
fn main() {
    mod_test();
    println!("Hello, world!");
}

