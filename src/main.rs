fn main() {
    println!("Hello, world!");
    unsafe {
        demo();
    }
}

extern {
    fn demo();
}
