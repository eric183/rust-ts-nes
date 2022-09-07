struct Human { name: String, sex: String, age: i32 }

fn struct_int() {
  let Me = Human { name: String::from("EricKuang"), sex: String::from("male"), age: 18 };

  println("here is {}", Me.name);
}