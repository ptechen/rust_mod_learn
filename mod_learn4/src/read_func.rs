mod static_kv;

pub fn read_func() {
    static_kv::static_kv();
    println!("read_func")
}