enum Trasmission { Manual, SemiAuto, Automatic }

struct Car {
	color: String,
	motor: Trasmission,
	roof: bool,
	mileage: u32,
}

fn main() {
	let colors = todo!("set the enum values: 0 = Blue, 1 = Green, 2 = Red, 3 = Silver");
	let mut car: Car = todo!("create `car` as  a `Car` struct");
	let mut engine: Trasmission = todo!("Declare `engine` as a `Tranmission` enum, initialize to `manual`");

	car = car_factory(String::from(todo!("Index into the `colors()` array")), engine, true, 0);
	println!("Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);

	engine = Tranmission::SemiAuto;
	car = car_factory(String::from(todo!("Index into the `colors()` array"), engine, true, 100));
	println!("Car order 2: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);

	engine = Tranmission::Automatic;
	car = car_factory(String::from(todo!("Index into the `colors()` array"), engine, true, 200));
	println!("Car order 3: {:?}, Hard top = {}, {:?}, {}, {} miles", car.age.0, car.roof, car.motor, car.color, car.age.1);


}

fn car_quality(miles: u32) -> (Age, u32) {
    let quality: (Age, u32) = todo!("Set the `Age` value to \"New\", set the mileage using the `miles` input argument");
	todo!("Return the tuple");
}

fn car_factory(color: String, motor: Trasmission, roof: bool, miles: u32) -> Car {
	Car {
		color: color,
		motor: motor,
		roof: roof,
		miles: miles,
	}
}