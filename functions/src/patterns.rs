#[derive(Debug)]
enum Color {
    Red,
    Blue,
    Green,
}

fn print_color(color: Color) {
    let msg = match color {
        Color::Red => "Rojah",
        Color::Blue => "Azure",
        Color::Green => "All the colors of Ireland",
    };
    println!("Your color choice is: {}", msg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prints_color() {
        print_color(Color::Red);
        print_color(Color::Blue);
        print_color(Color::Green)
    }
}
