mod bar;
mod foo;

fn say_hi() {
    println!("Hello, world!");
}
fn main() {
    say_hi();
    foo::say_hi();
    bar::say_hi();
    bar::nope::say_hi();
}
