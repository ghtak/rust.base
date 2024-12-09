mod hi_thiserror;
mod hi_anyhow;
mod blanket_implementation;
mod async_runtime;
mod hi_tokio_echo;
mod hi_hyper;
mod hello_tower;

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