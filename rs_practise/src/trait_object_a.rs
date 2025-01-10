trait Material {
    fn cost_per_sq_meter(&self) -> f64;
    fn square_meters(&self) -> f64;
    fn total_cost(&self) -> f64 {
        self.cost_per_sq_meter() * self.square_meters()
    }
}

struct Carpet(f64);
impl Material for Carpet {
    fn cost_per_sq_meter(&self) -> f64 {
        10.0
    }
    fn square_meters(&self) -> f64 {
        self.0
    }
}
struct Tile(f64);
impl Material for Tile {
    fn cost_per_sq_meter(&self) -> f64 {
        15.0
    }
    fn square_meters(&self) -> f64 {
        self.0
    }
}
struct Wood(f64);
impl Material for Wood {
    fn cost_per_sq_meter(&self) -> f64 {
        20.0
    }
    fn square_meters(&self) -> f64 {
        self.0
    }
}

fn total_cost(material: Vec<Box<dyn Material>>) -> f64 {
    material.iter().map(|m| m.total_cost()).sum()
}

pub fn trait_object_a() {
    let carpet = Box::new(Carpet(10.0));
    let tile = Box::new(Tile(15.0));
    let wood = Box::new(Wood(20.0));
    // 这里需要手动指定类型，因为Material是trait，不能直接使用
    let materials: Vec<Box<dyn Material>> = vec![carpet, tile, wood];
    println!("total cost: {}", total_cost(materials));
}
