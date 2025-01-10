use std::fmt::Debug;

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

#[derive(Debug)]
struct ConveyorBelt<T>
where
    T: Convey + Debug,
{
    pub items: Vec<T>,
}

impl<T> ConveyorBelt<T>
where
    T: Convey + Debug,
{
    pub fn add(&mut self, item: T) {
        self.items.push(item);
    }
}

#[derive(Debug)]
struct CarPart {
    width: f64,
    height: f64,
    depth: f64,
    weight: f64,
    part_number: String,
}

impl Default for CarPart {
    fn default() -> Self {
        CarPart {
            width: 5.0,
            height: 1.0,
            depth: 2.0,
            weight: 3.0,
            part_number: "abc".to_owned(),
        }
    }
}
trait Convey {
    fn weight(&self) -> f64;
    fn dimensions(&self) -> Dimensions;
}

impl Convey for CarPart {
    fn weight(&self) -> f64 {
        self.weight
    }
    fn dimensions(&self) -> Dimensions {
        Dimensions {
            width: self.width,
            height: self.height,
            depth: self.depth,
        }
    }
}

pub fn generic_structures_demo() {
    let mut belt: ConveyorBelt<CarPart> = ConveyorBelt {
        items: vec![],
    };
    belt.add(CarPart::default());
    println!("belt: {:?}", belt);
}
