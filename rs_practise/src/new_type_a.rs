#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
    Custom(String),
    Yellow,
    Purple,
    Orange,
    Pink,
    Brown,
    Gray,
    Black,
    White,
}

#[derive(Debug)]
struct ShirtColor(Color);
impl ShirtColor {
    fn new(color: Color) -> Result<Self, String> {
        match color {
            Color::Purple => Err("Purple is not a valid shirt color".to_string()),
            other => Ok(Self(other)),
        }
    }
}

#[derive(Debug)]
struct ShoesColor(Color);
impl ShoesColor {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

#[derive(Debug)]
struct PantsColor(Color);
impl PantsColor {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

fn print_shirt_color(shirt_color: ShirtColor) {
    println!("Shirt color: {:?}", shirt_color.0);
}

fn print_pants_color(pants_color: PantsColor) {
    println!("Pants color: {:?}", pants_color.0);
}

fn print_shoes_color(shoes_color: ShoesColor) {
    println!("Shoes color: {:?}", shoes_color.0);
}

pub fn new_type_a() {
    let shirt_color = ShirtColor::new(Color::Red).unwrap();
    let pants_color = PantsColor::new(Color::Blue);
    let shoes_color = ShoesColor::new(Color::Green);
    print_shirt_color(shirt_color);
    print_pants_color(pants_color);
    print_shoes_color(shoes_color);
}
