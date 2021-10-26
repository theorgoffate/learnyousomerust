mod bar;
mod foo;

fn sayHi() {
    println!("Hello, world!");
}
fn main() {
    sayHi();
    foo::sayHi();
    bar::sayHi();
    bar::nope::sayHi();
}
