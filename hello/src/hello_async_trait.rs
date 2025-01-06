#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::{future::Future, pin::Pin};

trait AsyncTrait{
    // fn something(&self) -> impl Future<Output = ()>;
    fn something(&self) -> Pin<Box<dyn Future<Output = ()>>>;
}

struct AsyncTraitImpl{}

impl AsyncTrait for AsyncTraitImpl{
    fn something(&self) -> Pin<Box<dyn Future<Output = ()>>>{
        Box::pin(async move {
            println!("SomeThing");
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let x : Box<dyn AsyncTrait> = Box::new(AsyncTraitImpl{});
                x.something().await;
            });

    }
}
