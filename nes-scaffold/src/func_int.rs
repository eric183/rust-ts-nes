fn func_int() {

  println!("my num is {}", func_int_return(2));
  
  println!("your num is {}", func_int_return(12));
  
}

fn func_int_return(num: i32) -> i32 {

  if(num >= 5) {
    return num + 3;
  }
  num - 2;
}