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
}