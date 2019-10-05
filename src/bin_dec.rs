use std::io;

pub fn start() {
  loop {
    let selection = opt();
    if selection == 1 {
      println!("Decimal to Binary Conversion!");
    } else if selection == 2 {
      println!("Binary to Decimal Conversion!");
    }
    let number = input("Enter an input");
    if selection == 1 {
      println!("{}", to_bin(number));
    } else if selection == 2 {
      println!("{}", to_dec(number));
    }
  }
}

fn opt() -> i32 {
  let mut option = String::new();

  println!("Select 1 or 2");
  println!("1. Decimal to Binary Conversion!");
  println!("2. Binary to Decimal Conversion!");

  io::stdin()
    .read_line(&mut option)
    .expect("Failed to read input");
  match option.trim().parse() {
    Ok(num) => num,
    Err(_e) => panic!("Not a number"),
  }
}

fn input(msg: &str) -> i32 {
  println!("{}", msg);
  let mut number = String::new();
  io::stdin()
    .read_line(&mut number)
    .expect("Failed to read input");

  match number.trim().parse() {
    Ok(num) => num,
    Err(_e) => panic!("Not a number"),
  }
}
fn to_dec(mut binary: i32) -> i32 {
  if binary == 0 {
    return 0;
  } else {
    let mut dec = 0;
    let mut base = 1;
    while binary > 0 {
      let last_digit = binary % 10;
      binary = binary / 10;
      dec += last_digit * base;
      base *= 2;
    }
    return dec;
  }
}
fn to_bin(mut decimal: i32) -> i32 {
  if decimal == 0 {
    return 0;
  } else {
    let mut bits = String::new();
    while decimal > 0 {
      if decimal % 2 == 0 {
        bits.push_str("0");
      } else {
        bits.push_str("1");
      }
      decimal /= 2;
    }
    match bits.chars().rev().collect::<String>().parse() {
      Ok(num) => num,
      Err(_e) => panic!("Something went wrong"),
    }
  }
}
