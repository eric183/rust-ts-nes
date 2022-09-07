#[derive(PartialEq, Debug)]
enum Age { New, Used }

#[derive(PartialEq, Debug)]
struct Car { color: String, motor: Transmission, roof: bool, age: (Age, u32) }


#[derive(PartialEq, Debug)]
enum Transmission { Manual, SemiAuto, Automatic }

fn car_quality (miles: u32) -> (Age, u32) {

    let quality = (Age::New, miles);

    quality
}

fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
  
    Car {
        color: color,
        motor: motor,
        roof: roof,
        age: car_quality(miles),
    }
}

fn main() {
    // Create car color array
    let colors = ["Blue", "Green", "Red", "Silver"];

    // Declare the car type and initial values
    let mut car: Car; 
    let mut engine = Transmission:: Manual;


      // Order 3 cars, one car for each type of transmission

    // Car order #1: New, Manual, Hard top
    car = car_factory(String::from(colors[0]), engine, true, 0);
    println!("Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);

    // Car order #2: Used, Semi-automatic, Convertible
    engine = Transmission::SemiAuto;
    car = car_factory(String::from(colors[1]), engine, false, 100);
    println!("Car order 2: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);

    // Car order #3: Used, Automatic, Hard top
    engine = Transmission::Automatic;
    car = car_factory(String::from(colors[2]), engine, true, 200);
    println!("Car order 3: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);
}
// struct Human { name: String, sex: String, age: i32 }
// struct StudyWhat(String, String);
// struct DoOtherThings { work: bool, eat: String }

// fn main() {

//     let meme = Human { name: String::from("EricKuang"), sex: String::from("male"), age: 18 };

//     // println("here is {}", meme);
//     println!("here is {}, I am {}, already {}", meme.name, meme.sex, meme.age);

//     enum EricTomorrow {
//         ContactSomeone(bool),
//         Study(StudyWhat),
//         DoOthers(DoOtherThings)
//     }

//     let study = StudyWhat(String::from("Rust"), String::from("Blender"));

//     let me_do = DoOtherThings { work: false, eat: String::from("bread") };

//     let _me_do_others = EricTomorrow::DoOthers(me_do);

//     let _me_study = EricTomorrow::Study(study);

//     func_int();
      
// }

// fn func_int() {

//     println!("my num is {}", func_int_return(2));

//     println!("your num is {}", func_int_return(12));

// }
  
// fn func_int_return(num: i32) -> i32 {

//     if num >= 0 {
//         return num + 3;
//     }

//     return num - 2;
// }