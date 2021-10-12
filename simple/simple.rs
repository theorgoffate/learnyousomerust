#[derive(Debug, Copy, Clone)]
struct StrCopy {
    value: &'static str
}

impl StrCopy {
    fn new(v: &'static str) -> Self {
        Self {
            value: v
        }
    }
}

fn main() {
    let num = 42;
    let prefix = StrCopy::new("Hello, world!");
    preprint(prefix.into(), "this is Great!", num);
    println!("{:?} from a macro {}", prefix, num);
    println!("{:?} part deux {}", prefix, num);
    preprint(prefix.into(), "this is bad!", num);
}

fn preprint(pre: StrCopy, msg: &str, num: i32) {
    println!("{:?} {} {}", pre, msg, num)
}