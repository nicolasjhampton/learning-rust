//use std::format;

fn main() {
    println!("Hello, world!");
    let tenth = fibnonacci(10);
    println!("{}", tenth);
    let cel = f_to_c(50.0);
    println!("{}", cel);
    let song = christmas_days(12);
    println!("{}.", song);
}

fn fibnonacci(nth: i32) -> i32 {
  if nth == 1 {
    return 1
  }
  nth + fibnonacci(nth - 1)
}

fn f_to_c(fair: f64) -> f64 {
  (fair - 32.0) * (5.0/9.0)
}

fn christmas_days(day: usize) -> std::string::String {
  let days = [
    "and a partridge in a pear tree",
    "Two turtle doves\n",
    "three French hens\n",
    "four calling birds\n",
    "five golden rings\n",
    "Six geese a-layin'\n",
    "seven swans a-swimmin'\n",
    "eight maids milkin'\n",
    "Nine lords a-leapin'\n",
    "ten ladies dancin'\n",
    "eleven pipers pipin'\n",
    "Twelve drummers drummin'\n",
  ];

  let mut today = format!("On day number {}, my true love gave to me...\n", day);

  if day == 1 {
    return today + "a partridge in a pear tree"
  }

  for index in (0..day).rev() {
    today += &format!("{}", days[index])
  }

  format!("{}. \n\n{}", christmas_days(day - 1), today)
}
