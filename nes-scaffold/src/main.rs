struct Human { name: String, sex: String, age: i32 }
struct StudyWhat(String, String);
struct DoOtherThings { work: bool, eat: String }

fn main() {

    let meme = Human { name: String::from("EricKuang"), sex: String::from("male"), age: 18 };

    // println("here is {}", meme);
    println!("here is {}, I am {}, already {}", meme.name, meme.sex, meme.age);

    enum EricTomorrow {
        ContactSomeone(bool),
        Study(StudyWhat),
        DoOthers(DoOtherThings)
    }

    let study = StudyWhat(String::from("Rust"), String::from("Blender"));

    let me_do = DoOtherThings { work: false, eat: String::from("bread") };

    let _me_do_others = EricTomorrow::DoOthers(me_do);

    let _me_study = EricTomorrow::Study(study);

    func_int();
      
}

fn func_int() {

    println!("my num is {}", func_int_return(2));

    println!("your num is {}", func_int_return(12));

}
  
fn func_int_return(num: i32) -> i32 {

    if num >= 0 {
        return num + 3;
    }

    return num - 2;
}