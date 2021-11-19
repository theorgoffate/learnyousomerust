#[get("/")]
pub fn index() ->  String {
  String::from("H")
}

#[get("/test1/<number>")]
pub fn test1(number: &str) -> String {
  // let something = String::from("32");
  // return format!("Hello, {}, {}", number, something);
  format!("hello {}", testfn(number))
}

#[get("/test2")]
pub fn test2() -> String {
  // let something = String::from("32");
  // return format!("Hello, {}, {}", number, something);
  format!("hello {}", testfn2())
}

pub fn testfn(number: &str) -> i32 {
  let mut my_int = number.parse::<i32>().unwrap();
  my_int = my_int + 5;
  //let mynumber = 
  return my_int;
}

pub fn testfn2() -> i32 {
  return 32;
}