mod hi_thiserror;
mod hi_anyhow;
mod blanket_implementation;

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert!(true);
    }
}