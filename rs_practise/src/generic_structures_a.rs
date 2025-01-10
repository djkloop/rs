use std::fmt::Debug;

// 泛型约束
trait Body {}

// 泛型约束
trait Color {}

// 泛型结构体
#[derive(Debug)]
struct Vehicle<T, U>
where
    T: Body,
    U: Color,
{
    body: T,
    color: U,
}

// 泛型结构体实现
impl<T, U> Vehicle<T, U>
where
    T: Body,
    U: Color,
{
    fn new(body: T, color: U) -> Self {
        Vehicle { body, color }
    }
}

#[derive(Debug)]
struct Car;
impl Body for Car {}

#[derive(Debug)]
struct Truck;
impl Body for Truck {}

#[derive(Debug)]
struct Red;
impl Color for Red {}

#[derive(Debug)]
struct Blue;
impl Color for Blue {}

pub fn generic_structures_a() {
    let red_truck = Vehicle::new(Truck, Red);
    let blue_car = Vehicle::new(Car, Blue);
    println!("red_truck: {:?}", red_truck);
    println!("red_truck_body: {:?}", red_truck.body);
    println!("red_truck_color: {:?}", red_truck.color);
    println!("blue_car: {:?}", blue_car);
    println!("blue_car_body: {:?}", blue_car.body);
    println!("blue_car_color: {:?}", blue_car.color);
}
