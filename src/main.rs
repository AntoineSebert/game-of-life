
//https://doc.rust-lang.org/book/second-edition/ch11-01-writing-tests.html 

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        println!("it_works am a test");
    }
}

fn main() {
    println!("Hello, world!");
}
