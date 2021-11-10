#[derive(Debug, Copy, Clone)]
enum EyeColor {
    Blue,
    Black,
    Red,
    None,
}

fn main() {
    let mut a = EyeColor::None;
    //let mut b = EyeColor::Black;
    //let mut b = "blue";
    //print_eye_color(a);
    select_eye_color(&mut a, EyeColor::Blue);
    println!("Selected EyeColor: {:?}", a);
    print_eye_color(a);
    // select_eye_color_two(&mut a, &mut b);
    print_eye_color(a);
}

fn print_eye_color(color: EyeColor) {
    //println!("Selected EyeColor: {:?}", color);
    let value = match color {
        EyeColor::None => "Vacant",
        EyeColor::Black => "Deep",
        EyeColor::Red => "Fire",
        EyeColor::Blue => "Ocean",
    };
    println!("Color feels like: {}", value)
}

fn select_eye_color(current_color: &mut EyeColor, select: EyeColor) {
    *current_color = select;
}

// fn select_eye_color_two(current_color_two: &'static mut EyeColor, select: &'static mut EyeColor) {
//     current_color_two = select;
//     //println!("I feel like {:?}", current_color_two)
// }
