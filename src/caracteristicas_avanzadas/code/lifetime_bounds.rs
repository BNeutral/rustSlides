fn main() {
    struct Ref<'a, T: 'a>(&'a T);
}