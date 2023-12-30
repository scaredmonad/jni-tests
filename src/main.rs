use crate::env::CoreFeatures;

mod env;

fn main() {
    use env::Env;
    let mut main = Env::default();
    main.attach();
    // main.invoke_static();
    println!("Hello, world!");
}
