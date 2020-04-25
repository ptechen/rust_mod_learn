#[cfg(test)]
mod tests {
    fn read_func_test() -> String {
        return "b29827ecc0ced7dc011014868259a780".to_string()
    }

    #[test]
    fn it_works() {
        assert_eq!(read_func_test(), "b29827ecc0ced7dc011014868259a780".to_string());
    }
}

mod static_kv;

pub fn read_func() {
    static_kv::static_kv();
    println!("read_func")
}