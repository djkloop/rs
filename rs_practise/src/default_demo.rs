struct Package {
    weight: f64,
}

impl Package {
    fn new(weight: f64) -> Self {
        Self { weight }
    }
}

impl Default for Package {
    fn default() -> Self {
        Self { weight: 3.0 }
    }
}

pub fn default_demo() {
    let package = Package::default();
    println!("Weight: {}", package.weight);
}

