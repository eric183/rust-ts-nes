struct Human { name: String, sex: String, age: i32 }

fn main() {

    let meme = Human { name: String::from("EricKuang"), sex: String::from("male"), age: 18 };

    // println("here is {}", meme);
    println!("here is {}, i am {}, already {}", meme.name, meme.sex, meme.age);

}
