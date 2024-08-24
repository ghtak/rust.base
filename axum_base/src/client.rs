mod internal;

fn main() {
    internal::diag::init_tracing().unwrap();
    tracing::info!("Hello Client!");
}