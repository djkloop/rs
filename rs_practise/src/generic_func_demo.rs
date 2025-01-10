trait CheckIn {
    fn check_in(&self);
    fn process(&self);
}

struct Pilot;
impl CheckIn for Pilot {
    fn check_in(&self) {
        println!("Pilot check in");
    }
    fn process(&self) {
        println!("Pilot process");
    }
}
struct Passenger;
impl CheckIn for Passenger {
    fn check_in(&self) {
        println!("Passenger check in");
    }
    fn process(&self) {
        println!("Passenger process");
    }
}

struct Cargo;
impl CheckIn for Cargo {
    fn check_in(&self) {
        println!("Cargo check in");
    }
    fn process(&self) {
        println!("Cargo process");
    }
}

fn process_item<T>(item: T)
where
    T: CheckIn,
{
    item.check_in();
    item.process();
}

pub fn generic_func_demo() {
    let pilot = Pilot;
    let passenger = Passenger;
    let cargo = Cargo;
    process_item(pilot);
    process_item(passenger);
    process_item(cargo);
}
