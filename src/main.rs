
type MyResult<T, E> = Result<T, E>;

fn main() {
    println!("Hello, world!");
}

fn myfn1() -> MyResult<String, String> {
    unimplemented!()
}
