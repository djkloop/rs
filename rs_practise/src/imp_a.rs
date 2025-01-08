enum Color {
    Brown,
    Red,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Brown => println!("brown"),
            Color::Red => println!("red"),
        }
    }
}

struct Dimensions {
    width: i32,
    height: i32,
    depth: i32,
}

impl Dimensions {
    fn print(&self) {
        println!("width = {:?}, height = {:?}, depth = {:?}", self.width, self.height, self.depth);
    }
}

struct ShippingBox {
    color: Color,
    weight: f64,
    dimensions: Dimensions,
}

impl ShippingBox {
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            weight,
            color,
            dimensions,
        }
    }

    fn print(&self) {   
        self.color.print();
        self.dimensions.print();
        println!("Weight: {} kg", self.weight);
    }
}

pub fn imp_a() {
    let snall_dimensions = Dimensions { width: 10, height: 10, depth: 10 };
    let small_box = ShippingBox::new(1.0, Color::Brown, snall_dimensions);
    small_box.print();

    let big_dimensions = Dimensions { width: 100, height: 100, depth: 100 };
    let big_box = ShippingBox::new(1.0, Color::Red, big_dimensions);
    big_box.print();
}
