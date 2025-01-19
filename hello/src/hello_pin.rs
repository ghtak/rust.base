#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use pin_project_lite::pin_project;
use std::{future::Future, pin::Pin, task::Poll};

pin_project! {
    struct HelloPin<T,U>{
        #[pin]
        pinned: T,
        unpinned: U
    }
}

impl<T, U> HelloPin<T, U> {
    fn call(self: Pin<&mut Self>) {
        let this = self.project();
        let _: Pin<&mut T> = this.pinned; // Pinned reference to the field
        let _: &mut U = this.unpinned; // Normal reference to the field
        println!("called");
    }
}

impl<T, U> Future for HelloPin<T, U> {
    type Output = ();

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = self.project();
        println!("Hello Pin");
        Poll::Ready(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_pin() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let hello_pin = HelloPin::<String, u32> {
                    pinned: "hello".into(),
                    unpinned: 0,
                };
                hello_pin.await;
            });

        // let hello_pin: Pin<&mut HelloPin<_, _>> = unsafe { Pin::new_unchecked(&mut hello_pin) };
        // hello_pin.call();
        // let hello = Pin::new(HelloPin::<String, u32> {
        //     pinned: "hello".into(),
        //     unpinned: 0,
        // });
    }
}
