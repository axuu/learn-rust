pub mod greeting {
    pub fn hello(name: &str) {
        println!("Hello, {}", &name)
    } // pub 才能外部可见
}
