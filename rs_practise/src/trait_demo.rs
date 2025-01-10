trait Fall {
    fn hit_ground(&self);
}

struct Vase;
impl Fall for Vase {
    fn hit_ground(&self) {
        println!("vase hit ground");
    }
}

struct Cat;
impl Fall for Cat {
    fn hit_ground(&self) {
        println!("cat hit ground");
    }
}

fn fall(item: impl Fall) {
    item.hit_ground();
}

pub fn trait_demo() {
    let vase = Vase;
    fall(vase);

    let cat = Cat;
    fall(cat);
}


