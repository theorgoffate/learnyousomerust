mod patterns;

fn main() {}

fn hello(name: String) -> String {
    format!("Hello, {}!", name)
}

fn add(l: i64, r: i64) -> i64 {
    l + r
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_from_str() {
        let name = "foobar".into(); // into() is the inverse of From trait
        let exp = String::from("Hello, foobar!");
        let got = hello(name);
        assert_eq!(exp, got);
    }

    #[test]
    fn add_nums() {
        let a = 2;
        let b = 2;
        let expected = a + b;
        let got = add(a, b);
        assert_eq!(expected, got);
    }

    #[test]
    fn complex_ownership() {
        let a = "foo";
        let b = hello(a.into()); // Moves a conversion into hello
        let _c = hello(b.clone()); // Moves a copy of b into hello
                                   // let _d = hello(b); // Moves ownership of b memory into hello
                                   // let _db = b; // Tries to use b after ownership has moved
                                   // let _e = format!("{}, {}, {}", a, b, c); // Tries to borrow b after ownership has moved
    }

    #[test]
    fn references() {
        let example = String::from("foo");
        let a: *const String = &example;
        println!("value at mem {:?} is {}", a, example)
    }
}
