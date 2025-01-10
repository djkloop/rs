trait Perimeter {
    fn calculate_perimeter(&self) -> f64;
}

struct Square {
    side: f64,
}

struct Triangle {
    side1: f64,
    side2: f64,
    side3: f64,
}

impl Perimeter for Square {
    fn calculate_perimeter(&self) -> f64 {
        self.side * 4.0
    }
}

impl Perimeter for Triangle {
    fn calculate_perimeter(&self) -> f64 {
        self.side1 + self.side2 + self.side3
    }
}

fn print_perimeter(shape: impl Perimeter) { 
    println!("Perimeter: {}", shape.calculate_perimeter());
}

pub fn trait_a() {
    let square = Square { side: 10.0 };
    print_perimeter(square);

    let triangle = Triangle {
        side1: 3.0,
        side2: 4.0,
        side3: 5.0,
    };
    print_perimeter(triangle);
}


