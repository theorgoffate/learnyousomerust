mod bar;
mod foo;

use bar::nope;
use bar::prelude::say_hi;
fn main() {
    foo::say_hi();
    bar::say_hi();
    bar::nope::say_hi();
    nope::say_hi();
    say_hi();
}
