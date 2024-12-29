#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use anyhow::Result;

pub trait Foo {
    fn bar(&self) -> Result<()>;
}

pub trait FooExt: Foo {
    fn convenient_bar(&self) -> Result<()> {
        println!("convenient bar begin");
        let ret = self.bar();
        println!("convenient bar end");
        ret
    }
}

// Blanket Implementation
impl<T: Foo + ?Sized> FooExt for T {}

// async
pub trait AsyncFoo{
    fn poll_bar(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<Result<()>>;
}

pub trait AsyncFooExt: AsyncFoo {
    fn convenient_async_bar(self: &mut Self) -> FooFut<Self> {
        FooFut { inner: self }
    }
}

impl <T: AsyncFoo + ?Sized> AsyncFooExt for T {}

pub struct FooFut<'a, T: ?Sized> {
    inner: &'a mut T,
}

impl<T: ?Sized + Unpin> Unpin for FooFut<'_, T> {}

impl <'a, T: AsyncFoo + ?Sized + Unpin> Future for FooFut<'_, T> {
    type Output = Result<()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut *self.inner).poll_bar(cx)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    struct Baz {}

    impl Foo for Baz {
        fn bar(&self) -> Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_blanket_implementation() {
        let baz = Baz {};
        baz.convenient_bar().unwrap();
    }

    struct AsyncBaz{}

    impl AsyncFoo for AsyncBaz{
        fn poll_bar(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<()>> {
            print!("async baz");
            Poll::Ready(Ok(()))
        }
    }

    #[test]
    fn test_async_foo(){
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let mut async_baz = AsyncBaz{};
            async_baz.convenient_async_bar().await.unwrap();
        })
    }
}