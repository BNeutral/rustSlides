fn main() {
    fn returns_closure() -> Box<Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
}